use std::env;
use std::path::*;
use std::process::*;

fn main() {
    let kind = if env::var("CARGO_FEATURE_STATIC_OPENBLAS").is_ok() {
        "static"
    } else {
        "dylib"
    };

    if !env::var("CARGO_FEATURE_SYSTEM_OPENBLAS").is_ok() {
        let build = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("build");
        let output = PathBuf::from(&env::var("OUT_DIR").unwrap());

        run(Command::new("make").current_dir(&build), "make");

        println!("cargo:rustc-link-search={}", output.join("opt/OpenBLAS/lib").display());
    }

    println!("cargo:rustc-link-lib={}=openblas", kind);
    println!("cargo:rustc-link-lib=dylib=gfortran");
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(error) => fail(&format!("failed to execute `{}`: {}", program, error)),
    };
    if !status.success() {
        fail(&format!("`{}` failed: {}", program, status));
    }
}

fn fail(message: &str) -> ! {
    panic!("\n{}\n\nbuild script failed", message)
}
