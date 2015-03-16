extern crate libc;

use libc::{c_int, c_char};

extern "C" {
    pub fn openblas_set_num_threads(num_threads: c_int) -> ();
    pub fn goto_set_num_threads(num_threads: c_int) -> ();
    pub fn openblas_get_config() -> *mut c_char;
    pub fn openblas_get_corename() -> *mut c_char;
    pub fn openblas_get_parallel() -> c_int;
}
