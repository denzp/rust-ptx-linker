#![deny(warnings)]
#![feature(abi_ptx)]
#![no_std]

extern crate dummy_math;
use dummy_math::{dummy_mul_2, dummy_square};

#[no_mangle]
pub unsafe extern "ptx-kernel" fn top_level_kernel(x: *const f64, y: *mut f64, a: f64) {
    *y.offset(0) = {
        indirect_update(*x.offset(0), a, dummy_square)
            + indirect_update(*x.offset(0), a, dummy_mul_2)
    };
}

#[inline(never)]
fn indirect_update(x: f64, a: f64, func: fn(f64) -> f64) -> f64 {
    func(x) * a
}

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}
