use std::env::{var, remove_var};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let kind = if var("CARGO_FEATURE_STATIC_OPENBLAS").is_ok() { "static" } else { "dylib" };

    if var("CARGO_FEATURE_SYSTEM_OPENBLAS").is_ok() {
        println!("cargo:rustc-link-lib={}=openblas", kind);
        return;
    }

    let cblas = var("CARGO_FEATURE_CBLAS").is_ok();

    let source = PathBuf::from(&var("CARGO_MANIFEST_DIR").unwrap()).join("source");
    let output = PathBuf::from(&var("OUT_DIR").unwrap());

    remove_var("TARGET");

    run(Command::new("make")
                .args(&["libs", "netlib", "shared"])
                .arg(&format!("{}_CBLAS=1", if cblas { "YES" } else { "NO" }))
                .arg(&format!("-j{}", var("NUM_JOBS").unwrap()))
                .current_dir(&source));

    run(Command::new("make")
                .arg("install")
                .arg(&format!("DESTDIR={}", output.display()))
                .current_dir(&source));

    match read("FC", &source.join("Makefile.conf")) {
        Ok(ref name) => {
            if name.contains("gfortran") {
                println!("cargo:rustc-link-lib=dylib=gfortran");
            }
        },
        Err(error) => panic!("failed to detect Fortran: {}", error),
    }

    println!("cargo:rustc-link-search={}", output.join("opt/OpenBLAS/lib").display());
    println!("cargo:rustc-link-lib={}=openblas", kind);
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

fn read(prefix: &str, path: &Path) -> Result<String> {
    let mut file = try!(File::open(path));
    let reader = BufReader::new(&mut file);
    for line in reader.lines() {
        let line = try!(line);
        if line.starts_with(&prefix) {
            return Ok(line)
        }
    }
    Err(Error::new(ErrorKind::Other, format!("failed to find `{}` in {}", prefix, path.display())))
}
