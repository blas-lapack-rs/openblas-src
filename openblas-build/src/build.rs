//! Execute make of OpenBLAS, and its options

use crate::{check::*, error::*};
use std::{env, fs, path::*, process::Command, str::FromStr};

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
    // for DYNNAMIC_ARCH=1
    GENERIC,
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
    CORTEXA76,
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
    RISCV64_ZVL128B,
    C910V,
    x280,
    RISCV64_ZVL236B,

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

    // CSKY
    CSKY,
    CK860FV,
}

impl FromStr for Target {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let target = match s.to_ascii_lowercase().as_str() {
            "generic" => Self::GENERIC,
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
            "cortexa76" => Self::CORTEXA76,
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
            "riscv64_zvl128b" => Self::RISCV64_ZVL128B,
            "c910v" => Self::C910V,
            "x280" => Self::x280,
            "riscv64_zvl236b" => Self::RISCV64_ZVL236B,

            // LOONGARCH64:
            "loongsongeneric" => Self::LOONGSONGENERIC,
            "longson3r5" => Self::LOONGSON3R5,
            "longson2k1000" => Self::LOONGSON2K1000,

            // Elbrus E2000:
            "e2k" => Self::E2K,

            // Alpha
            "ev4" => Self::EV4,
            "ev5" => Self::EV5,
            "ev6" => Self::EV6,

            // CSKY
            "csky" => Self::CSKY,
            "ck860fv" => Self::CK860FV,

            _ => {
                return Err(Error::UnsupportedTarget {
                    target: s.to_string(),
                })
            }
        };
        Ok(target)
    }
}

impl Target {
    fn get_generic_target() -> Option<Self> {
        let target = env::var("TARGET").unwrap();
        let target_arch = target.split('-').nth(0).unwrap();
        match target_arch {
            "aarch64" => Some(Target::ARMV8),
            "arm" => Some(Target::ARMV6),
            "armv5te" => Some(Target::ARMV5),
            "armv6" => Some(Target::ARMV6),
            "armv7" => Some(Target::ARMV7),
            "loongarch64" => Some(Target::LOONGSONGENERIC),
            "mips64" => Some(Target::MIPS64_GENERIC),
            "mips64el" => Some(Target::MIPS64_GENERIC),
            "riscv64" => Some(Target::RISCV64_GENERIC),
            "csky" => Some(Target::CK860FV),
            "sparc" => Some(Target::SPARCV7),
            //TODO: add more generic targets
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Compilers {
    pub cc: Option<String>,
    pub fc: Option<String>,
    pub hostcc: Option<String>,
    pub ranlib: Option<String>,
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
    pub compilers: Compilers,
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
            compilers: Compilers::default(),
        }
    }
}

impl Configure {
    fn make_args(&self) -> Result<Vec<String>, Error> {
        // check if it is cross-compilation
        let build_target = env::var("TARGET").unwrap_or_default();
        let build_host = env::var("HOST").unwrap_or_default();
        let is_cross_compile = build_target != build_host;

        let mut args = Vec::new();
        if self.no_static {
            args.push("NO_STATIC=1".into());
        }
        if self.no_shared {
            args.push("NO_SHARED=1".into());
        }
        if self.no_cblas {
            args.push("NO_CBLAS=1".into());
        }
        if self.no_lapack {
            args.push("NO_LAPACK=1".into());
        }
        if self.no_lapacke {
            args.push("NO_LAPACKE=1".into());
        }
        if self.use_thread {
            args.push("USE_THREAD=1".into());
            args.push("NUM_THREADS=256".into());
        }
        if self.use_openmp {
            args.push("USE_OPENMP=1".into());
            args.push("NUM_THREADS=256".into());
        }
        if matches!(self.interface, Interface::ILP64) {
            args.push("INTERFACE64=1".into());
        }
        if let Some(target) = self.target.as_ref() {
            args.push(format!("TARGET={:?}", target));
        } else if is_cross_compile {
            if let Some(target) = Target::get_generic_target() {
                args.push(format!("TARGET={:?}", target));
            } else {
                return Err(Error::MissingCrossCompileInfo {
                    info: "TARGET".to_string(),
                });
            }
        }

        let mut cc_compiler = self.compilers.cc.clone();
        if let Some(cc) = self.compilers.cc.as_ref() {
            args.push(format!("CC={}", cc));
        } else if is_cross_compile {
            let compiler = cc::Build::new().get_compiler();
            let compiler_path = compiler.path().to_str();
            if let Some(cc) = compiler_path {
                args.push(format!("CC={}", cc));
                cc_compiler = Some(cc.to_string());
            } else {
                return Err(Error::MissingCrossCompileInfo {
                    info: "CC".to_string(),
                });
            }
        }
        if let Some(fc) = self.compilers.fc.as_ref() {
            args.push(format!("FC={}", fc))
        } else if is_cross_compile {
            let mut fortran = false;
            if let Some(cc) = cc_compiler {
                let fc = cc
                    .replace("gcc", "gfortran")
                    .replace("clang", "flang")
                    .replace("icc", "ifort");

                if Command::new(&fc).arg("--version").check_call().is_ok() {
                    args.push(format!("FC={}", fc));
                    fortran = true;
                }
            }
            if !fortran {
                println!("cargo:warning=OpenBLAS: Detecting fortran compiler failed. Can only compile BLAS and f2c-converted LAPACK.");
                args.push("NOFORTRAN=1".into());
            }
        }
        if let Some(hostcc) = self.compilers.hostcc.as_ref() {
            args.push(format!("HOSTCC={}", hostcc))
        } else if is_cross_compile {
            let compiler = cc::Build::new().target(build_host.as_str()).get_compiler();
            let compiler_path = compiler.path().to_str();
            if let Some(hostcc) = compiler_path {
                args.push(format!("HOSTCC={}", hostcc));
            } else {
                return Err(Error::MissingCrossCompileInfo {
                    info: "HOSTCC".to_string(),
                });
            }
        }
        if let Some(ranlib) = self.compilers.ranlib.as_ref() {
            args.push(format!("RANLIB={}", ranlib))
        }
        Ok(args)
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
    pub fn build<P: AsRef<Path>>(self, openblas_root: P) -> Result<MakeConf, Error> {
        let root = openblas_root.as_ref();
        // Do not build if libraries and Makefile.conf already exist and are valid
        if let Ok(make_conf) = MakeConf::new(root.join("Makefile.conf")) {
            return Ok(make_conf);
        }

        // check if cross compile is needed
        // let build_target = env::var("TARGET").unwrap_or_default();
        // let build_host = env::var("HOST").unwrap_or_default();
        // let is_cross_compile = build_target != build_host;
        // if is_cross_compile && (self.compilers.cc.is_none() || self.compilers.hostcc.is_none()) {
        //     return Err(Error::MissingCrossCompileInfo);
        // }

        // Run `make` as an subprocess
        //
        // - This will automatically run in parallel without `-j` flag
        // - The `make` of OpenBLAS outputs 30k lines,
        //   which will be redirected into `out.log` and `err.log`.
        // - cargo sets `TARGET` environment variable as target triple (e.g. x86_64-unknown-linux-gnu)
        //   while binding build.rs, but `make` read it as CPU target specification.
        //
        let out = fs::File::create(root.join("out.log")).expect("Cannot create log file");
        let err = fs::File::create(root.join("err.log")).expect("Cannot create log file");
        match Command::new("make")
            .current_dir(root)
            .stdout(out)
            .stderr(err)
            .args(self.make_args()?)
            .args(["all"])
            .env_remove("TARGET")
            .check_call()
        {
            Ok(_) => {}
            Err(err @ Error::NonZeroExitStatus { .. }) => {
                eprintln!(
                    "{}",
                    fs::read_to_string(root.join("err.log")).expect("Cannot read log file")
                );
                return Err(err);
            }
            Err(e) => {
                return Err(e);
            }
        }

        let make_conf = MakeConf::new(root.join("Makefile.conf"))?;
        if make_conf.no_fortran {
            println!("cargo:warning=OpenBLAS: Detecting fortran compiler failed. Only BLAS and f2c-converted LAPACK are compiled.");
        }
        Ok(make_conf)
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

    fn get_openblas_source<P: AsRef<Path>>(out_dir: P) -> PathBuf {
        let openblas_src_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../openblas-src/");
        let source = crate::download(&openblas_src_root).unwrap();
        // copy files to the target directory
        let out_dir = out_dir.as_ref();
        fs::create_dir_all(out_dir).unwrap();
        for entry in walkdir::WalkDir::new(&source) {
            let entry = entry.unwrap();
            let src = entry.path();
            let dest = out_dir.join(src.strip_prefix(&source).unwrap());
            if entry.file_type().is_dir() {
                fs::create_dir_all(&dest).unwrap();
            } else {
                fs::copy(src, dest).unwrap();
            }
        }
        out_dir.to_path_buf()
    }

    #[ignore]
    #[test]
    fn build_default() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let out_dir = root.join("test_build/build_default");
        let opt = Configure::default();
        let _ = opt.build(get_openblas_source(&out_dir)).unwrap();
    }

    #[ignore]
    #[test]
    fn build_no_shared() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let out_dir = root.join("test_build/build_no_shared");
        let mut opt = Configure::default();
        opt.no_shared = true;
        opt.build(get_openblas_source(&out_dir)).unwrap();
        let _ = LibInspect::new(out_dir.join("libopenblas.a")).unwrap();
    }

    #[ignore]
    #[test]
    fn build_no_lapacke() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let out_dir = root.join("test_build/build_no_lapacke");
        let mut opt = Configure::default();
        opt.no_lapacke = true;
        let _ = opt.build(get_openblas_source(&out_dir)).unwrap();
        let lib_name = if cfg!(target_os = "macos") {
            "libopenblas.dylib"
        } else {
            "libopenblas.so"
        };
        let lib_inspect = LibInspect::new(out_dir.join(lib_name)).unwrap();

        assert!(lib_inspect.has_lapack());
        assert!(!lib_inspect.has_lapacke());
    }

    #[ignore]
    #[test]
    fn build_no_cblas() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let out_dir = root.join("test_build/build_no_cblas");
        let mut opt = Configure::default();
        opt.no_lapacke = true;
        let _ = opt.build(get_openblas_source(&out_dir)).unwrap();
        let lib_name = if cfg!(target_os = "macos") {
            "libopenblas.dylib"
        } else {
            "libopenblas.so"
        };
        let lib_inspect = LibInspect::new(out_dir.join(lib_name)).unwrap();

        assert!(!lib_inspect.has_cblas());
    }

    #[ignore]
    #[test]
    fn build_openmp() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let out_dir = root.join("test_build/build_openmp");
        let mut opt = Configure::default();
        opt.use_openmp = true;
        let _ = opt.build(get_openblas_source(&out_dir)).unwrap();
        let lib_name = if cfg!(target_os = "macos") {
            "libopenblas.dylib"
        } else {
            "libopenblas.so"
        };
        let lib_inspect = LibInspect::new(out_dir.join(lib_name)).unwrap();
        assert!(lib_inspect.has_lib("gomp"));
    }
}
