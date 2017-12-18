#![feature(abi_ptx)]
#![no_std]

extern crate dummy_utils;
use dummy_utils::dummy_mul;

#[no_mangle]
pub fn dummy_square(x: f64) -> f64 {
    dummy_mul(x, x)
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn dummy_math_kernel(x: *mut f64, y: *mut f64) {
    *y.offset(0) = dummy_square(*x.offset(0));
}

