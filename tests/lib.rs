extern crate libc;
extern crate openblas_src;

use libc::c_float;

extern "C" {
    pub fn srotg_(a: *mut c_float, b: *mut c_float, c: *mut c_float, s: *mut c_float);
}

#[test]
fn link() {
    unsafe {
        let mut a: f32 = 0.0;
        let mut b: f32 = 0.0;
        let mut c: f32 = 42.0;
        let mut d: f32 = 42.0;
        srotg_(
            &mut a as *mut _,
            &mut b as *mut _,
            &mut c as *mut _,
            &mut d as *mut _,
        );
        assert!(c == 1.0);
        assert!(d == 0.0);
    }
}
