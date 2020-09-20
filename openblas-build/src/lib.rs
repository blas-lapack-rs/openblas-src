//! openblas-build
//! ---------------
//!
//! Helper crate for openblas-src/build.rs

use anyhow::{bail, Result};
use std::{
    collections::HashSet,
    fs,
    hash::Hash,
    io::{self, BufRead},
    os::unix::io::*,
    path::*,
    process::{Command, Stdio},
};
use walkdir::WalkDir;

pub fn openblas_source_dir() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("source");
    if !path.join("Makefile").exists() {
        panic!("OpenBLAS repository has not been cloned. Run `git submodule update --init`");
    }
    path
}

#[derive(Debug, Clone, Copy)]
pub enum Interface {
    LP64,
    ILP64,
}

impl Default for Interface {
    fn default() -> Self {
        Interface::LP64
    }
}

/// Target CPU list
/// from https://github.com/xianyi/OpenBLAS/blob/v0.3.10/TargetList.txt
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

#[derive(Debug, Clone, Default)] // default of bool is false
pub struct BuildOption {
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

impl BuildOption {
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
    pub fn build<P: AsRef<Path>>(self, out_dir: P) -> Result<Detail> {
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

        Command::new("make")
            .current_dir(out_dir)
            .stdout(unsafe { Stdio::from_raw_fd(out.into_raw_fd()) })
            .stderr(unsafe { Stdio::from_raw_fd(err.into_raw_fd()) })
            .args(&self.make_args())
            .check_call()?;

        Ok(Detail::from_make_conf(out_dir.join("Makefile.conf"))?)
    }
}

trait CheckCall {
    fn check_call(&mut self) -> Result<()>;
}

impl CheckCall for Command {
    fn check_call(&mut self) -> Result<()> {
        match self.status() {
            Ok(status) => {
                if !status.success() {
                    bail!(
                        "Subprocess returns with non-zero status: `{:?}` ({})",
                        self,
                        status
                    );
                }
                Ok(())
            }
            Err(error) => {
                bail!("Subprocess execution failed: `{:?}` ({})", self, error);
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LinkInfo {
    pub search_paths: Vec<PathBuf>,
    pub libs: Vec<String>,
}

fn as_sorted_vec<T: Hash + Ord>(set: HashSet<T>) -> Vec<T> {
    let mut v: Vec<_> = set.into_iter().collect();
    v.sort();
    v
}

impl LinkInfo {
    fn parse(line: &str) -> Result<Self> {
        let mut search_paths = HashSet::new();
        let mut libs = HashSet::new();
        for entry in line.split(" ") {
            if entry.starts_with("-L") {
                search_paths.insert(PathBuf::from(entry.trim_start_matches("-L")).canonicalize()?);
            }
            if entry.starts_with("-l") {
                libs.insert(entry.trim_start_matches("-l").into());
            }
        }
        Ok(LinkInfo {
            search_paths: as_sorted_vec(search_paths),
            libs: as_sorted_vec(libs),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Detail {
    os_name: String,
    no_fortran: bool,
    c_extra_libs: LinkInfo,
    f_extra_libs: LinkInfo,
}

impl Detail {
    /// Parse Makefile.conf which generated by OpenBLAS make system
    pub fn from_make_conf<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut detail = Detail::default();
        let f = fs::File::open(path)?;
        let buf = io::BufReader::new(f);
        for line in buf.lines() {
            let line = line.unwrap();
            if line.len() == 0 {
                continue;
            }
            let entry: Vec<_> = line.split("=").collect();
            if entry.len() != 2 {
                continue;
            }
            match entry[0] {
                "OSNAME" => detail.os_name = entry[1].into(),
                "NOFORTRAN" => detail.no_fortran = true,
                "CEXTRALIB" => detail.c_extra_libs = LinkInfo::parse(entry[1])?,
                "FEXTRALIB" => detail.f_extra_libs = LinkInfo::parse(entry[1])?,
                _ => continue,
            }
        }
        Ok(detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn build_default() -> Result<()> {
        let opt = BuildOption::default();
        let _detail = opt.build("test_build/build_default")?;
        Ok(())
    }

    #[test]
    fn detail_from_makefile_conf() -> Result<()> {
        let detail = Detail::from_make_conf("Makefile.conf")?;
        assert!(!detail.no_fortran);
        Ok(())
    }

    #[test]
    fn detail_from_nofortran_conf() -> Result<()> {
        let detail = Detail::from_make_conf("nofortran.conf")?;
        assert!(detail.no_fortran);
        Ok(())
    }

    #[test]
    fn link_info_parse() -> Result<()> {
        // from nofortran.conf
        let info = LinkInfo::parse("-L/usr/lib/gcc/x86_64-pc-linux-gnu/10.2.0 -L/usr/lib/gcc/x86_64-pc-linux-gnu/10.2.0/../../../../lib -L/lib/../lib -L/usr/lib/../lib -L/usr/lib/gcc/x86_64-pc-linux-gnu/10.2.0/../../..  -lc")?;
        assert_eq!(
            info.search_paths,
            vec![
                PathBuf::from("/usr/lib"),
                PathBuf::from("/usr/lib/gcc/x86_64-pc-linux-gnu/10.2.0")
            ]
        );
        assert_eq!(info.libs, vec!["c"]);
        Ok(())
    }
}
