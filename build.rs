use std::env;
use std::fs;
use std::hash::Hasher;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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

        // Determine build arguments.
        let mut build_args = vec![
            format!("BINARY={}", binary!()),
            format!("{}_CBLAS=1", switch!(cblas)),
            format!("{}_LAPACKE=1", switch!(lapacke)),
            format!("-j{}", variable!("NUM_JOBS")),
        ];
        let target = match env::var("OPENBLAS_TARGET") {
            Ok(target) => {
                build_args.push(format!("TARGET={}", target));
                target
            }
            _ => variable!("TARGET"),
        };
        env::remove_var("TARGET");
        for name in &vec!["CC", "FC", "HOSTCC"] {
            if let Ok(value) = env::var(format!("OPENBLAS_{}", name)) {
                build_args.push(format!("{}={}", name, value));
            }
        }

        // Get information about the build options and compiler versions.
        let build_info = run_with_stdout(
            Command::new("make")
                .args(&["-f", "make_build_info", "echo-build-info", "TOPDIR=source"])
                .args(&build_args),
            Stdio::piped(),
        );
        run(Command::new("make").arg("clean").current_dir("source"));
        let build_info_hash: u64 = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            hasher.write(&build_info);
            hasher.finish()
        };

        // Copy the source files and build info into the build directory.
        let build_dir = PathBuf::from(format!(
            "source_{}_{:016x}",
            target.to_lowercase(),
            build_info_hash
        ));
        if !build_dir.exists() {
            let tmp = PathBuf::from(format!("{}_tmp", build_dir.display()));
            if tmp.exists() {
                fs::remove_dir_all(&tmp).unwrap();
            }
            run(Command::new("cp").arg("-R").arg("source").arg(&tmp));
            fs::rename(&tmp, &build_dir).unwrap();
            fs::File::create(build_dir.join(".openblas-src_build_info"))
                .unwrap()
                .write_all(&build_info)
                .unwrap();
        }

        // Run the build.
        run(Command::new("make")
            .args(&["libs", "netlib", "shared"])
            .args(&build_args)
            .current_dir(&build_dir));

        // Copy the binaries into the destination directory and tell Cargo
        // where they are.
        run(Command::new("make")
            .arg("install")
            .arg(format!("DESTDIR={}", output.display()))
            .current_dir(&build_dir));
        println!(
            "cargo:rustc-link-search={}",
            output.join("opt/OpenBLAS/lib").display(),
        );
    }
    println!("cargo:rustc-link-lib=dylib=gfortran");
    println!("cargo:rustc-link-lib={}=openblas", kind);
}

/// Runs the command, inheriting stdin/stdout/stderr.
///
/// **Panics** if the command fails.
fn run(command: &mut Command) {
    run_with_stdout(command, Stdio::inherit());
}

/// Runs the command, using the specified stdout.
///
/// If `stdout` is `Stdio::piped()`, then the standard output will be captured
/// and returned.
///
/// **Panics** if the command fails.
fn run_with_stdout(command: &mut Command, stdout: Stdio) -> Vec<u8> {
    println!("Running: `{:?}`", command);
    let output = command
        .stdin(Stdio::inherit())
        .stdout(stdout)
        .stderr(Stdio::inherit())
        .output();
    match output {
        Ok(out) => {
            if out.status.success() {
                out.stdout
            } else {
                panic!("Failed: `{:?}` ({})", command, out.status);
            }
        }
        Err(error) => {
            panic!("Failed: `{:?}` ({})", command, error);
        }
    }
}
