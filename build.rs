use std::env::{remove_var, var};
use std::path::PathBuf;
use std::process::Command;

macro_rules! feature(
    ($name:expr) => (var(concat!("CARGO_FEATURE_", $name)).is_ok());
);

macro_rules! switch(
    ($condition:expr) => (if $condition { "YES" } else { "NO" });
);

macro_rules! variable(
    ($name:expr) => (var($name).unwrap());
);

fn main() {
    let kind = if feature!("STATIC") { "static" } else { "dylib" };

    if feature!("SYSTEM") {
        println!("cargo:rustc-link-lib={}=openblas", kind);
        return;
    }

    let cblas = feature!("CBLAS");
    let lapacke = feature!("LAPACKE");
    let source = PathBuf::from("source");
    let output = PathBuf::from(variable!("OUT_DIR").replace(r"\", "/"));

    remove_var("TARGET");

    run(Command::new("make")
                .args(&["libs", "netlib", "shared"])
                .arg(format!("{}_CBLAS=1", switch!(cblas)))
                .arg(format!("{}_LAPACKE=1", switch!(lapacke)))
                .arg(format!("-j{}", variable!("NUM_JOBS")))
                .current_dir(&source));

    run(Command::new("make")
                .arg("install")
                .arg(format!("DESTDIR={}", output.display()))
                .current_dir(&source));

    println!("cargo:rustc-link-search={}", output.join("opt/OpenBLAS/lib").display());
    println!("cargo:rustc-link-lib={}=openblas", kind);
    println!("cargo:rustc-link-lib=dylib=gfortran");
}

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => if !status.success() {
            panic!("`{:?}` failed: {}", command, status);
        },
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        },
    }
}
