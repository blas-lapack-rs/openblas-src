//! Execute make of OpenBLAS, and its options

use super::*;
use std::{
    fs,
    os::unix::io::*,
    path::*,
    process::{Command, Stdio},
};
use walkdir::WalkDir;

fn openblas_source_dir() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("source");
    if !path.join("Makefile").exists() {
        panic!("OpenBLAS repository has not been cloned. Run `git submodule update --init`");
    }
    path
}

/// Interface for 32-bit interger (LP64) and 64-bit integer (ILP64)
#[derive(Debug, Clone, Copy)]
pub enum Interface {
    LP64,
    ILP64,
}

/// CPU list in [TargetList](https://github.com/xianyi/OpenBLAS/blob/v0.3.10/TargetList.txt)
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)] // to use original identifiers
pub enum Target {
    // X86/X86_64 Intel
    P2,
    KATMAI,
    COPPERMINE,
    NORTHWOOD,
    PRESCOTT,
    BANIAS,
    YONAH,
    CORE2,
    PENRYN,
    DUNNINGTON,
    NEHALEM,
    SANDYBRIDGE,
    HASWELL,
    SKYLAKEX,
    ATOM,

    // X86/X86_64 AMD
    ATHLON,
    OPTERON,
    OPTERON_SSE3,
    BARCELONA,
    SHANGHAI,
    ISTANBUL,
    BOBCAT,
    BULLDOZER,
    PILEDRIVER,
    STEAMROLLER,
    EXCAVATOR,
    ZEN,

    // X86/X86_64 generic
    SSE_GENERIC,
    VIAC3,
    NANO,

    // Power
    POWER4,
    POWER5,
    POWER6,
    POWER7,
    POWER8,
    POWER9,
    PPCG4,
    PPC970,
    PPC970MP,
    PPC440,
    PPC440FP2,
    CELL,

    // MIPS
    P5600,
    MIPS1004K,
    MIPS24K,

    // MIPS64
    SICORTEX,
    LOONGSON3A,
    LOONGSON3B,
    I6400,
    P6600,
    I6500,

    // IA64
    ITANIUM2,

    // Sparc
    SPARC,
    SPARCV7,

    // ARM
    CORTEXA15,
    CORTEXA9,
    ARMV7,
    ARMV6,
    ARMV5,

    // ARM64
    ARMV8,
    CORTEXA53,
    CORTEXA57,
    CORTEXA72,
    CORTEXA73,
    NEOVERSEN1,
    EMAG8180,
    FALKOR,
    THUNDERX,
    THUNDERX2T99,
    TSV110,

    // System Z
    ZARCH_GENERIC,
    Z13,
    Z14,
}

/// make option generator
#[derive(Debug, Clone)]
pub struct Configure {
    pub no_static: bool,
    pub no_shared: bool,
    pub no_cblas: bool,
    pub no_lapack: bool,
    pub no_lapacke: bool,
    pub use_thread: bool,
    pub use_openmp: bool,
    pub dynamic_arch: bool,
    pub interface: Interface,
    pub target: Option<Target>,
}

impl Default for Configure {
    fn default() -> Self {
        Configure {
            no_static: false,
            no_shared: false,
            no_cblas: false,
            no_lapack: false,
            no_lapacke: false,
            use_thread: false,
            use_openmp: false,
            dynamic_arch: false,
            interface: Interface::LP64,
            target: None,
        }
    }
}

/// Deliverables of `make` command
pub struct Deliverables {
    /// None if `no_static`
    pub static_lib: Option<LibInspect>,
    /// None if `no_shared`
    pub shared_lib: Option<LibInspect>,
    /// Inspection what `make` command really show.
    pub make_conf: MakeConf,
}

impl Configure {
    fn make_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if self.no_static {
            args.push("NO_STATIC=1".into())
        }
        if self.no_shared {
            args.push("NO_SHARED=1".into())
        }
        if self.no_cblas {
            args.push("NO_CBLAS=1".into())
        }
        if self.no_lapack {
            args.push("NO_LAPACK=1".into())
        }
        if self.no_lapacke {
            args.push("NO_LAPACKE=1".into())
        }
        if self.use_thread {
            args.push("USE_THREAD=1".into())
        }
        if self.use_openmp {
            args.push("USE_OPENMP=1".into())
        }
        if matches!(self.interface, Interface::ILP64) {
            args.push("INTERFACE64=1".into())
        }
        if let Some(target) = self.target.as_ref() {
            args.push(format!("TARGET={:?}", target))
        }
        args
    }

    /// Shared or static library will be created
    /// at `out_dir/libopenblas.so` or `out_dir/libopenblas.a`
    pub fn build<P: AsRef<Path>>(self, out_dir: P) -> Result<Deliverables> {
        let out_dir = out_dir.as_ref();
        if !out_dir.exists() {
            fs::create_dir_all(out_dir)?;
        }
        let root = openblas_source_dir();
        for entry in WalkDir::new(&root) {
            let entry = entry.unwrap();
            let dest = out_dir.join(entry.path().strip_prefix(&root)?);
            if dest.exists() {
                // Do not overwrite
                // Cache of previous build should be cleaned by `cargo clean`
                continue;
            }
            if entry.file_type().is_dir() {
                fs::create_dir(&dest)?;
            }
            if entry.file_type().is_file() {
                fs::copy(entry.path(), &dest)?;
            }
        }

        let out = fs::File::create(out_dir.join("out.log")).expect("Cannot create log file");
        let err = fs::File::create(out_dir.join("err.log")).expect("Cannot create log file");

        // This will automatically run in parallel without `-j` flag
        Command::new("make")
            .current_dir(out_dir)
            .stdout(unsafe { Stdio::from_raw_fd(out.into_raw_fd()) }) // this works only for unix
            .stderr(unsafe { Stdio::from_raw_fd(err.into_raw_fd()) })
            .args(&self.make_args())
            .check_call()?;

        Ok(Deliverables {
            static_lib: if !self.no_static {
                Some(LibInspect::new(out_dir.join("libopenblas.a")))
            } else {
                None
            },
            shared_lib: if !self.no_shared {
                Some(LibInspect::new(out_dir.join("libopenblas.so")))
            } else {
                None
            },
            make_conf: MakeConf::new(out_dir.join("Makefile.conf"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn build_no_shared() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_build/build_no_shared");
        let mut opt = Configure::default();
        opt.no_shared = true;
        let detail = opt.build(path).unwrap();
        assert!(detail.shared_lib.is_none());
    }

    #[ignore]
    #[test]
    fn build_no_lapacke() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_build/build_no_lapacke");
        let mut opt = Configure::default();
        opt.no_lapacke = true;
        let detail = opt.build(path).unwrap();
        let shared_lib = detail.shared_lib.unwrap();
        assert!(shared_lib.has_lapack());
        assert!(!shared_lib.has_lapacke());
    }

    #[ignore]
    #[test]
    fn build_openmp() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_build/build_openmp");
        let mut opt = Configure::default();
        opt.use_openmp = true;
        let detail = opt.build(path).unwrap();
        assert!(detail.shared_lib.unwrap().has_lib("gomp"));
    }
}
