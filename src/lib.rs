
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub mod hello;

use hello::dval;

pub fn blah() {
    let _result = dval(5);
}

#[test]
fn test_dval() {
    let d = dval(5);
    assert_eq!(d, 10);
}

