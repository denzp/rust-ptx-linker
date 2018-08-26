#![deny(warnings)]
#![feature(abi_ptx, panic_handler)]
#![no_std]

extern crate dummy_math;
use dummy_math::dummy_square;

#[no_mangle]
pub unsafe extern "ptx-kernel" fn top_level_kernel(x: *const f64, y: *mut f64, a: f64) {
    *y.offset(0) = dummy_square(*x.offset(0)) * a;
}

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}
