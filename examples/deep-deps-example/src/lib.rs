#![feature(abi_ptx, lang_items)]
#![no_std]

extern crate dummy_math;
use dummy_math::dummy_square;

#[no_mangle]
pub unsafe extern "ptx-kernel" fn top_level_kernel(x: *const f64, y: *mut f64, a: f64) {
    *y.offset(0) = dummy_square(*x.offset(0)) * a;
}

// Needed because we compile `dylib`...
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

