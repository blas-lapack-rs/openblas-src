// FIXME This should be test also on Windows and macOS.
//
// However, we have several problems:
//
// - we cannot build Fortran part of OpenBLAS on windows-msvc platform
//   because the absence of Fortran compiler
//
// - In macOS, we can get gfortran by `brew install gcc`,
//   but it is too time-consuming to execute on CI.
//   GitHub Actions's macOS instance says gfotran is "installed",
//   but it is too fragile me to give up using it.
//
#![cfg(target_os = "linux")]

extern crate openblas_src as _src;

extern "C" {
    fn dormbr_(
        vect: *const u8,
        side: *const u8,
        trans: *const u8,
        m: *const i32,
        n: *const i32,
        k: *const i32,
        A: *const f64,
        lda: *const i32,
        tau: *const f64,
        C: *mut f64,
        ldc: *const i32,
        work: *mut f64,
        lwork: *const i32,
        info: *mut i32,
    );
}

// `dormbr_` is imported from reference LAPACK written in Fortran into OpenBLAS project.
// This test will fail to link when OpenBLAS does not build Fortran part.
#[test]
fn test_link_lapack() {
    let m = 1;
    let n = 1;
    let k = 1;
    let vect = b'Q';
    let side = b'L';
    let trans = b'N';
    let a = vec![0.0];
    let lda = 1;
    let mut c = vec![0.0];
    let ldc = 1;
    let tau = 0.0;
    let mut work = vec![0.0];
    let lwork = 1;
    let mut info = 0;
    unsafe {
        dormbr_(
            &vect,
            &side,
            &trans,
            &m,
            &n,
            &k,
            a.as_ptr(),
            &lda,
            &tau,
            c.as_mut_ptr(),
            &ldc,
            work.as_mut_ptr(),
            &lwork,
            &mut info,
        );
    }
}
