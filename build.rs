use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

macro_rules! binary(() => (if cfg!(target_pointer_width = "32") { "32" } else { "64" }));
macro_rules! feature(($name:expr) => (env::var(concat!("CARGO_FEATURE_", $name)).is_ok()));
macro_rules! switch(($condition:expr) => (if $condition { "YES" } else { "NO" }));
macro_rules! variable(($name:expr) => (env::var($name).unwrap()));

fn main() {
    let kind = if feature!("STATIC") {
        "static"
    } else {
        "dylib"
    };
    if !feature!("SYSTEM") {
        let cblas = feature!("CBLAS");
        let lapacke = feature!("LAPACKE");
        if cfg!(target_os="windows") {
            if kind == "dynamic" {
                env::set_var("VCPKGRS_DYNAMIC", "1");
            } else {
                env::set_var("CARGO_CFG_TARGET_FEATURE", "crt-static");
            }
            vcpkg::find_package("openblas").unwrap();
        } else {
            let output = PathBuf::from(variable!("OUT_DIR").replace(r"\", "/"));
            let mut make = Command::new("make");
            make.args(&["libs", "netlib", "shared"])
                .arg(format!("BINARY={}", binary!()))
                .arg(format!("{}_CBLAS=1", switch!(cblas)))
                .arg(format!("{}_LAPACKE=1", switch!(lapacke)))
                .arg(format!("-j{}", variable!("NUM_JOBS")));
            let target = match env::var("OPENBLAS_TARGET") {
                Ok(target) => {
                    make.arg(format!("TARGET={}", target));
                    target
                }
                _ => variable!("TARGET"),
            };
            env::remove_var("TARGET");
            let source = if feature!("CACHE") {
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
        }
    }
    println!("cargo:rustc-link-lib={}=openblas", kind);
}

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
