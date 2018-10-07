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
        let output = PathBuf::from(variable!("OUT_DIR").replace(r"\", "/"));
        let mut make = Command::new("make");
        make.args(&["libs", "netlib", "shared"])
            .arg(format!("BINARY={}", binary!()))
            .arg(format!("{}_CBLAS=1", switch!(cblas)))
            .arg(format!("{}_LAPACKE=1", switch!(lapacke)))
            .arg(format!("-j{}", variable!("NUM_JOBS")));
        let target: PathBuf = match env::var("OPENBLAS_TARGET") {
            Ok(target) => {
                make.arg(format!("TARGET={}", target));
                target
            }
            _ => variable!("TARGET"),
        }.to_lowercase().into();
        env::remove_var("TARGET");
        if !target.exists() {
            let make_working_dir_tmp =
                PathBuf::from(format!("{}_TMP", target.to_str().unwrap()));
            if make_working_dir_tmp.exists() {
                fs::remove_dir_all(&make_working_dir_tmp).unwrap();
            }
            run(Command::new("cp")
                .arg("-R")
                .arg("source")
                .arg(&make_working_dir_tmp));
            fs::rename(&make_working_dir_tmp, &target).unwrap();
        }
        for make_env_key in &vec!["CC", "FC", "HOSTCC"] {
            match env::var(format!("OPENBLAS_{}", make_env_key)) {
                Ok(value) => {
                    make.arg(format!("{}={}", make_env_key, value));
                }
                _ => {}
            }
        }
        run(&mut make.current_dir(&target));
        run(Command::new("make")
            .arg("install")
            .arg(format!("DESTDIR={}", output.display()))
            .current_dir(&target));
        println!(
            "cargo:rustc-link-search={}",
            output.join("opt/OpenBLAS/lib").display(),
        );
    }
    println!("cargo:rustc-link-lib=dylib=gfortran");
    println!("cargo:rustc-link-lib={}=openblas", kind);
}

fn run(command: &mut Command) {
    println!("Running: `{:?}`", command);
    match command.status() {
        Ok(status) => if !status.success() {
            panic!("Failed: `{:?}` ({})", command, status);
        },
        Err(error) => {
            panic!("Failed: `{:?}` ({})", command, error);
        }
    }
}
