use std::{env, path::*, process::Command};

fn feature_enabled(feature: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

/// Add path where pacman (on msys2) install OpenBLAS
///
/// - `pacman -S mingw-w64-x86_64-openblas` will install
///   - `libopenbla.dll` into `/mingw64/bin`
///   - `libopenbla.a`   into `/mingw64/lib`
/// - But we have to specify them using `-L` in **Windows manner**
///   - msys2 `/` is `C:\msys64\` in Windows by default install
///   - It can be convert using `cygpath` command
fn windows_gnu_system() {
    let lib_path = String::from_utf8(
        Command::new("cygpath")
            .arg("-w")
            .arg(if feature_enabled("static") {
                "/mingw64/bin"
            } else {
                "/mingw64/lib"
            })
            .output()
            .expect("Failed to exec cygpath")
            .stdout,
    )
    .expect("cygpath output includes non UTF-8 string");
    println!("cargo:rustc-link-search={}", lib_path);
}

/// Use vcpkg for msvc "system" feature
fn windows_msvc_system() {
    if !feature_enabled("static") {
        env::set_var("VCPKGRS_DYNAMIC", "1");
    }
    #[cfg(target_env = "msvc")]
    vcpkg::find_package("openblas").unwrap();
    if !cfg!(target_env = "msvc") {
        unreachable!();
    }
}

/// homebrew says
///
/// > openblas is keg-only, which means it was not symlinked into /usr/local,
/// > because macOS provides BLAS in Accelerate.framework.
/// > For compilers to find openblas you may need to set:
///
/// ```text
/// export LDFLAGS="-L/usr/local/opt/openblas/lib"
/// export CPPFLAGS="-I/usr/local/opt/openblas/include"
/// ```
fn macos_system() {
    println!("cargo:rustc-link-search=/usr/local/opt/openblas/lib");
}

fn main() {
    let link_kind = if feature_enabled("static") {
        "static"
    } else {
        "dylib"
    };
    if feature_enabled("system") {
        if cfg!(target_os = "windows") {
            if cfg!(target_env = "gnu") {
                windows_gnu_system();
            } else if cfg!(target_env = "msvc") {
                windows_msvc_system();
            } else {
                panic!(
                    "Unsupported ABI for Windows: {}",
                    env::var("CARGO_CFG_TARGET_ENV").unwrap()
                );
            }
        }
        if cfg!(target_os = "macos") {
            macos_system();
        }
        println!("cargo:rustc-link-lib={}=openblas", link_kind);
    } else {
        if cfg!(target_env = "msvc") {
            panic!(
                "Non-vcpkg builds are not supported on Windows. You must use the 'system' feature."
            )
        }
        build();
    }
    println!("cargo:rustc-link-lib={}=openblas", link_kind);
}

/// Build OpenBLAS using openblas-build crate
#[cfg(target_os = "linux")]
fn build() {
    let mut cfg = openblas_build::Configure::default();
    if !feature_enabled("cblas") {
        cfg.no_cblas = true;
    }
    if !feature_enabled("lapacke") {
        cfg.no_lapacke = true;
    }
    if feature_enabled("static") {
        cfg.no_shared = true;
    } else {
        cfg.no_static = true;
    }
    if let Ok(target) = env::var("OPENBLAS_TARGET") {
        cfg.target = Some(
            target
                .parse()
                .expect("Unsupported target is specified by $OPENBLAS_TARGET"),
        )
        // Do not default to the native target (represented by `cfg.target == None`)
        // because most user set `$OPENBLAS_TARGET` explicitly will hope not to use the native target.
    }

    let output = if feature_enabled("cache") {
        use std::{collections::hash_map::DefaultHasher, hash::*};
        // Build OpenBLAS on user's data directory.
        // See https://docs.rs/dirs/3.0.1/dirs/fn.data_dir.html
        //
        // On Linux, `data_dir` returns `$XDG_DATA_HOME` or `$HOME/.local/share`.
        // This build script creates a directory based on the hash value of `cfg`,
        // i.e. `$XDG_DATA_HOME/openblas_build/[Hash of cfg]`, and build OpenBLAS there.
        //
        // This build will be shared among several projects using openblas-src crate.
        // It makes users not to build OpenBLAS in every `cargo build`.
        let mut hasher = DefaultHasher::new();
        cfg.hash(&mut hasher);

        dirs::data_dir()
            .expect("Cannot get user's data directory")
            .join("openblas_build")
            .join(format!("{:x}", hasher.finish()))
    } else {
        PathBuf::from(env::var("OUT_DIR").unwrap())
    };

    // If OpenBLAS is build as shared, user of openblas-src will have to find `libopenblas.so` at runtime.
    //
    // `cargo run` appends the link paths to `LD_LIBRARY_PATH` specified by `cargo:rustc-link-search`,
    // and user's crate can find it then.
    //
    // However, when user try to run it directly like `./target/release/user_crate_exe`, it will say
    // "error while loading shared libraries: libopenblas.so: cannot open shared object file: No such file or directory".
    //
    // Be sure that `cargo:warning` is shown only when openblas-src is build as path dependency...
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargowarningmessage
    if !feature_enabled("static") {
        println!(
            "cargo:warning=OpenBLAS is built as a shared library. You need to set LD_LIBRARY_PATH={}",
            output.display()
        );
    }

    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("source");
    let deliv = cfg.build(&source, &output).unwrap();

    println!("cargo:rustc-link-search={}", output.display());
    for search_path in &deliv.make_conf.c_extra_libs.search_paths {
        println!("cargo:rustc-link-search={}", search_path.display());
    }
    for lib in &deliv.make_conf.c_extra_libs.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
    for search_path in &deliv.make_conf.f_extra_libs.search_paths {
        println!("cargo:rustc-link-search={}", search_path.display());
    }
    for lib in &deliv.make_conf.f_extra_libs.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}

/// openblas-src 0.9.0 compatible `make` runner
///
/// This cannot detect that OpenBLAS skips LAPACK build due to the absense of Fortran compiler.
/// openblas-build crate can detect it by sneaking OpenBLAS build system, but only works on Linux.
///
#[cfg(not(target_os = "linux"))]
fn build() {
    use std::fs;

    let output = PathBuf::from(env::var("OUT_DIR").unwrap().replace(r"\", "/"));
    let mut make = Command::new("make");
    make.args(&["libs", "netlib", "shared"])
        .arg(format!("BINARY={}", binary()))
        .arg(format!(
            "{}_CBLAS=1",
            if feature_enabled("cblas") {
                "YES"
            } else {
                "NO"
            }
        ))
        .arg(format!(
            "{}_LAPACKE=1",
            if feature_enabled("lapacke") {
                "YES"
            } else {
                "NO"
            }
        ));
    match env::var("OPENBLAS_ARGS") {
        Ok(args) => {
            make.args(args.split_whitespace());
        }
        _ => (),
    };
    if let Ok(num_jobs) = env::var("NUM_JOBS") {
        make.arg(format!("-j{}", num_jobs));
    }
    let target = match env::var("OPENBLAS_TARGET") {
        Ok(target) => {
            make.arg(format!("TARGET={}", target));
            target
        }
        _ => env::var("TARGET").unwrap(),
    };
    env::remove_var("TARGET");
    let source = if feature_enabled("cache") {
        PathBuf::from(format!("source_{}", target.to_lowercase()))
    } else {
        output.join(format!("source_{}", target.to_lowercase()))
    };

    if !source.exists() {
        let source_tmp = PathBuf::from(format!("{}_tmp", source.display()));
        if source_tmp.exists() {
            fs::remove_dir_all(&source_tmp).unwrap();
        }
        run(Command::new("cp").arg("-R").arg("source").arg(&source_tmp));
        fs::rename(&source_tmp, &source).unwrap();
    }
    for name in &vec!["CC", "FC", "HOSTCC"] {
        if let Ok(value) = env::var(format!("OPENBLAS_{}", name)) {
            make.arg(format!("{}={}", name, value));
        }
    }
    run(&mut make.current_dir(&source));
    run(Command::new("make")
        .arg("install")
        .arg(format!("DESTDIR={}", output.display()))
        .current_dir(&source));
    println!(
        "cargo:rustc-link-search={}",
        output.join("opt/OpenBLAS/lib").display(),
    );

    fn run(command: &mut Command) {
        println!("Running: `{:?}`", command);
        match command.status() {
            Ok(status) => {
                if !status.success() {
                    panic!("Failed: `{:?}` ({})", command, status);
                }
            }
            Err(error) => {
                panic!("Failed: `{:?}` ({})", command, error);
            }
        }
    }

    fn binary() -> &'static str {
        if cfg!(target_pointer_width = "32") {
            "32"
        } else {
            "64"
        }
    }
}
