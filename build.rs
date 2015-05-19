use std::env::{var, remove_var};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let kind = if var("CARGO_FEATURE_STATIC_OPENBLAS").is_ok() {
        "static"
    } else {
        "dylib"
    };

    if var("CARGO_FEATURE_SYSTEM_OPENBLAS").is_err() {
        let cblas = var("CARGO_FEATURE_EXCLUDE_CBLAS").is_err();

        let src = PathBuf::from(&var("CARGO_MANIFEST_DIR").unwrap()).join("OpenBLAS");
        let dst = PathBuf::from(&var("OUT_DIR").unwrap());

        remove_var("TARGET");

        run(Command::new("make")
                    .args(&["libs", "netlib", "shared"])
                    .arg(&format!("{}_CBLAS=1", if cblas { "YES" } else { "NO" }))
                    .arg(&format!("-j{}", var("NUM_JOBS").unwrap()))
                    .current_dir(&src), "make libs netlib shared");

        run(Command::new("make")
                    .arg("install")
                    .arg(&format!("DESTDIR={}", dst.display()))
                    .current_dir(&src), "make install");

        println!("cargo:rustc-link-search={}", dst.join("opt/OpenBLAS/lib").display());

        match read("FC", &src.join("Makefile.conf")) {
            Ok(ref name) => match &name[..] {
                "gfortran" => println!("cargo:rustc-link-lib=dylib=gfortran"),
                _ => {},
            },
            Err(error) => panic!("failed to detect Fortran: {}", error),
        }
    }

    println!("cargo:rustc-link-lib={}=openblas", kind);
}

fn run(command: &mut Command, program: &str) {
    println!("running: {:?}", command);
    match command.status() {
        Ok(status) => if !status.success() {
            panic!("`{}` failed: {}", program, status);
        },
        Err(error) => {
            panic!("failed to execute `{}`: {}", program, error);
        },
    }
}

fn read(name: &str, path: &Path) -> Result<String> {
    let mut file = try!(File::open(path));
    let reader = BufReader::new(&mut file);
    let prefix = format!("{}=", name);
    for line in reader.lines() {
        let line = try!(line);
        if line.starts_with(&prefix) {
            return Ok(String::from(&line[prefix.len()..]))
        }
    }
    Err(Error::new(ErrorKind::Other, format!("failed to find `{}` in {}", name, path.display())))
}
