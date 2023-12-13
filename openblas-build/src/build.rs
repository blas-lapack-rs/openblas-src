//! Execute make of OpenBLAS, and its options

use crate::{check::*, error::*};
use std::{fs, path::*, process::Command, str::FromStr};
use walkdir::WalkDir;

/// Interface for 32-bit interger (LP64) and 64-bit integer (ILP64)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interface {
    LP64,
    ILP64,
}

/// CPU list in [TargetList](https://github.com/xianyi/OpenBLAS/blob/v0.3.10/TargetList.txt)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    COOPERLAKE,
    SAPPHIRERAPIDS,

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
    POWER10,
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
    MIPS64_GENERIC,
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
    CORTEXA510,
    CORTEXA710,
    CORTEXX1,
    CORTEXX2,
    NEOVERSEN1,
    NEOVERSEV1,
    NEOVERSEN2,
    CORTEXA55,
    EMAG8180,
    FALKOR,
    THUNDERX,
    THUNDERX2T99,
    TSV110,
    THUNDERX3T110,
    VORTEX,
    A64FX,
    ARMV8SVE,
    FT2000,

    // System Z
    ZARCH_GENERIC,
    Z13,
    Z14,

    // RISC-V 64:
    RISCV64_GENERIC,
    C910V,

    // LOONGARCH64:
    LOONGSONGENERIC,
    LOONGSON3R5,
    LOONGSON2K1000,

    // Elbrus E2000:
    E2K,

    // Alpha
    EV4,
    EV5,
    EV6,
}

impl FromStr for Target {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let target = match s.to_ascii_lowercase().as_str() {
            // X86/X86_64 Intel
            "p2" => Self::P2,
            "katamai" => Self::KATMAI,
            "coppermine" => Self::COPPERMINE,
            "northwood" => Self::NORTHWOOD,
            "prescott" => Self::PRESCOTT,
            "banias" => Self::BANIAS,
            "yonah" => Self::YONAH,
            "core2" => Self::CORE2,
            "penryn" => Self::PENRYN,
            "dunnington" => Self::DUNNINGTON,
            "nehalem" => Self::NEHALEM,
            "sandybridge" => Self::SANDYBRIDGE,
            "haswell" => Self::HASWELL,
            "skylakex" => Self::SKYLAKEX,
            "atom" => Self::ATOM,
            "cooperlake" => Self::COOPERLAKE,
            "sapphirerapids" => Self::SAPPHIRERAPIDS,

            // X86/X86_64 AMD
            "athlon" => Self::ATHLON,
            "opteron" => Self::OPTERON,
            "opteron_sse3" => Self::OPTERON_SSE3,
            "barcelona" => Self::BARCELONA,
            "shanghai" => Self::SHANGHAI,
            "istanbul" => Self::ISTANBUL,
            "bobcat" => Self::BOBCAT,
            "bulldozer" => Self::BULLDOZER,
            "piledriver" => Self::PILEDRIVER,
            "steamroller" => Self::STEAMROLLER,
            "excavator" => Self::EXCAVATOR,
            "zen" => Self::ZEN,

            // X86/X86_64 generic
            "sse_generic" => Self::SSE_GENERIC,
            "viac3" => Self::VIAC3,
            "nano" => Self::NANO,

            // Power
            "power4" => Self::POWER4,
            "power5" => Self::POWER5,
            "power6" => Self::POWER6,
            "power7" => Self::POWER7,
            "power8" => Self::POWER8,
            "power9" => Self::POWER9,
            "power10" => Self::POWER10,
            "ppcg4" => Self::PPCG4,
            "ppc970" => Self::PPC970,
            "ppc970mp" => Self::PPC970MP,
            "ppc440" => Self::PPC440,
            "ppc440fp2" => Self::PPC440FP2,
            "cell" => Self::CELL,

            // MIPS
            "p5600" => Self::P5600,
            "mips1004k" => Self::MIPS1004K,
            "mips24k" => Self::MIPS24K,

            // MIPS64
            "mips64_generic" => Self::MIPS64_GENERIC,
            "sicortex" => Self::SICORTEX,
            "loongson3a" => Self::LOONGSON3A,
            "loongson3b" => Self::LOONGSON3B,
            "i6400" => Self::I6400,
            "p6600" => Self::P6600,
            "i6500" => Self::I6500,

            // IA64
            "itanium2" => Self::ITANIUM2,

            // Sparc
            "sparc" => Self::SPARC,
            "sparcv7" => Self::SPARCV7,

            // ARM
            "cortexa15" => Self::CORTEXA15,
            "cortexa9" => Self::CORTEXA9,
            "armv7" => Self::ARMV7,
            "armv6" => Self::ARMV6,
            "armv5" => Self::ARMV5,

            // ARM64
            "armv8" => Self::ARMV8,
            "cortexa53" => Self::CORTEXA53,
            "cortexa57" => Self::CORTEXA57,
            "cortexa72" => Self::CORTEXA72,
            "cortexa73" => Self::CORTEXA73,
            "cortexa510" => Self::CORTEXA510,
            "cortexa710" => Self::CORTEXA710,
            "cortexx1" => Self::CORTEXX1,
            "cortexx2" => Self::CORTEXX2,
            "neoversen1" => Self::NEOVERSEN1,
            "neoversev1" => Self::NEOVERSEV1,
            "neoversen2" => Self::NEOVERSEN2,
            "cortexa55" => Self::CORTEXA55,
            "emag8180" => Self::EMAG8180,
            "falkor" => Self::FALKOR,
            "thunderx" => Self::THUNDERX,
            "thunderx2t99" => Self::THUNDERX2T99,
            "tsv110" => Self::TSV110,
            "thunderx3t110" => Self::THUNDERX3T110,
            "vortex" => Self::VORTEX,
            "a64fx" => Self::A64FX,
            "armv8sve" => Self::ARMV8SVE,
            "ft2000" => Self::FT2000,

            // System Z
            "zarch_generic" => Self::ZARCH_GENERIC,
            "z13" => Self::Z13,
            "z14" => Self::Z14,

            // RISC-V 64:
            "riscv64_generic" => Self::RISCV64_GENERIC,
            "c910v" => Self::C910V,

            // LOONGARCH64:
            "loongsongeneric" => Self::LOONGSONGENERIC,
            "loongson3r5" => Self::LOONGSON3R5,
            "loongson2k1000" => Self::LOONGSON2K1000,

            // Elbrus E2000:
            "e2k" => Self::E2K,

            // Alpha
            "ev4" => Self::EV4,
            "ev5" => Self::EV5,
            "ev6" => Self::EV6,

            _ => {
                return Err(Error::UnsupportedTarget {
                    target: s.to_string(),
                })
            }
        };
        Ok(target)
    }
}

/// make option generator
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    /// Inspect existing build deliverables, and validate them.
    ///
    /// Error
    /// ------
    /// - No build deliverables exist
    /// - Build deliverables are not valid
    ///   - e.g. `self.no_lapack == false`, but the existing library does not contains LAPACK symbols.
    ///
    pub fn inspect(&self, out_dir: impl AsRef<Path>) -> Result<Deliverables, Error> {
        let out_dir = out_dir.as_ref();
        let make_conf = MakeConf::new(out_dir.join("Makefile.conf"))?;

        if !self.no_lapack && make_conf.no_fortran {
            return Err(Error::FortranCompilerNotFound);
        }

        Ok(Deliverables {
            static_lib: if !self.no_static {
                Some(LibInspect::new(out_dir.join("libopenblas.a"))?)
            } else {
                None
            },
            shared_lib: if !self.no_shared {
                Some(LibInspect::new(if cfg!(target_os = "macos") {
                    out_dir.join("libopenblas.dylib")
                } else {
                    out_dir.join("libopenblas.so")
                })?)
            } else {
                None
            },
            make_conf,
        })
    }

    /// Build OpenBLAS
    ///
    /// Libraries are created directly under `out_dir` e.g. `out_dir/libopenblas.a`
    ///
    /// Error
    /// -----
    /// - Build deliverables are invalid same as [inspect].
    ///   This means that the system environment is not appropriate to execute `make`,
    ///   e.g. LAPACK is required but there is no Fortran compiler.
    ///
    pub fn build(
        self,
        openblas_root: impl AsRef<Path>,
        out_dir: impl AsRef<Path>,
    ) -> Result<Deliverables, Error> {
        let out_dir = out_dir.as_ref();
        if !out_dir.exists() {
            fs::create_dir_all(out_dir)?;
        }

        // Do not build if libraries and Makefile.conf already exist and are valid
        if let Ok(deliv) = self.inspect(out_dir) {
            return Ok(deliv);
        }

        // Copy OpenBLAS sources from this crate to `out_dir`
        let root = openblas_root.as_ref();
        for entry in WalkDir::new(root) {
            let entry = entry.expect("Unknown IO error while walkdir");
            let dest = out_dir.join(
                entry
                    .path()
                    .strip_prefix(root)
                    .expect("Directory entry is not under root"),
            );
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

        // Run `make` as an subprocess
        //
        // - This will automatically run in parallel without `-j` flag
        // - The `make` of OpenBLAS outputs 30k lines,
        //   which will be redirected into `out.log` and `err.log`.
        // - cargo sets `TARGET` environment variable as target triple (e.g. x86_64-unknown-linux-gnu)
        //   while binding build.rs, but `make` read it as CPU target specification.
        //
        let out = fs::File::create(out_dir.join("out.log")).expect("Cannot create log file");
        let err = fs::File::create(out_dir.join("err.log")).expect("Cannot create log file");
        match Command::new("make")
            .current_dir(out_dir)
            .stdout(out)
            .stderr(err)
            .args(&self.make_args())
            .args(["all"])
            .env_remove("TARGET")
            .check_call()
        {
            Ok(_) => {}
            Err(err @ Error::NonZeroExitStatus { .. }) => {
                eprintln!(
                    "{}",
                    fs::read_to_string(out_dir.join("err.log")).expect("Cannot read log file")
                );
                return Err(err);
            }
            Err(e) => {
                return Err(e);
            }
        }

        self.inspect(out_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_from_str() {
        assert_eq!(Target::from_str("p2").unwrap(), Target::P2);
        assert!(matches!(
            Target::from_str("p3").unwrap_err(),
            crate::error::Error::UnsupportedTarget { .. }
        ));
    }

    fn get_openblas_source() -> PathBuf {
        let openblas_src_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../openblas-src");
        crate::download(&openblas_src_root).unwrap()
    }

    #[ignore]
    #[test]
    fn build_default() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let opt = Configure::default();
        let _detail = opt
            .build(get_openblas_source(), root.join("test_build/build_default"))
            .unwrap();
    }

    #[ignore]
    #[test]
    fn build_no_shared() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut opt = Configure::default();
        opt.no_shared = true;
        let detail = opt
            .build(
                get_openblas_source(),
                root.join("test_build/build_no_shared"),
            )
            .unwrap();
        assert!(detail.shared_lib.is_none());
    }

    #[ignore]
    #[test]
    fn build_no_lapacke() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut opt = Configure::default();
        opt.no_lapacke = true;
        let detail = opt
            .build(
                get_openblas_source(),
                root.join("test_build/build_no_lapacke"),
            )
            .unwrap();
        let shared_lib = detail.shared_lib.unwrap();
        assert!(shared_lib.has_lapack());
        assert!(!shared_lib.has_lapacke());
    }

    #[ignore]
    #[test]
    fn build_openmp() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut opt = Configure::default();
        opt.use_openmp = true;
        let detail = opt
            .build(get_openblas_source(), root.join("test_build/build_openmp"))
            .unwrap();
        assert!(detail.shared_lib.unwrap().has_lib("gomp"));
    }
}
