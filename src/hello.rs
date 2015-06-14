
// Rust integration to hello.c

extern crate libc;

use libc::c_int;

#[link(name="hello", kind="static")]
extern "C" {
    fn double_val(x: c_int) -> c_int;
}

pub fn dval(x: i32) -> i32 {

    let result = unsafe {double_val(x)};
    return result
}

