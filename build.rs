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
        let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("OpenBLAS");
        let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());

        run(Command::new("make")
                    .arg("NO_CBLAS=1")
                    .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
                    .current_dir(&src), "make");

        run(Command::new("make")
                    .arg("install")
                    .arg(&format!("DESTDIR={}", dst.display()))
                    .current_dir(&src), "make install");

        println!("cargo:rustc-link-search={}", dst.join("opt/OpenBLAS/lib").display());
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
