use std::path::*;
use std::process::*;
use std::env;
use std::io::*;

fn main() {
    let kind = if env::var("CARGO_FEATURE_STATIC_OPENBLAS").is_ok() {
        "static"
    } else {
        "dylib"
    };

    if !env::var("CARGO_FEATURE_SYSTEM_OPENBLAS").is_ok() {
        let mut cflags = env::var("CFLAGS").unwrap_or(String::new());
        let target = env::var("TARGET").unwrap();

        if target.contains("i686") {
            cflags.push_str(" -m32");
        } else if target.contains("x86_64") {
            cflags.push_str(" -m64");
        }

        if !target.contains("i686") {
            cflags.push_str(" -fPIC");
        }

        let src = PathBuf::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("OpenBLAS");
        let dst = PathBuf::new(&env::var("OUT_DIR").unwrap());

        run(Command::new("make").current_dir(&src), "make");
        run(Command::new("make").current_dir(&src).arg("install").arg(&format!("DESTDIR={}", dst.display())), "make install");

        println!("cargo:rustc-flags=-L {}", dst.join("opt/OpenBLAS/lib").display());
    }

    println!("cargo:rustc-flags=-l {}=openblas", kind);
    println!("cargo:rustc-flags=-l gfortran");
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::FileNotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
