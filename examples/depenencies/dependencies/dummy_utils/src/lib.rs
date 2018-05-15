#![feature(abi_ptx)]
#![no_std]

pub const PAIR: [f32; 2] = [5.0, 7.0];

#[no_mangle]
#[inline(never)]
pub fn dummy_mul(x1: f64, x2: f64) -> f64 {
    dummy_mul_inner(x1, x2)
}

#[no_mangle]
#[inline(never)]
pub fn dummy_mul_inner(x1: f64, x2: f64) -> f64 {
    x1 * x2
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn dummy_utils_kernel(x1: *const f64, x2: *const f64, y: *mut f64) {
    *y.offset(0) = dummy_mul(*x1.offset(0), *x2.offset(0));
}
