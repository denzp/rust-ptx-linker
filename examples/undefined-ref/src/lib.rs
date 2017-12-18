#![feature(abi_ptx, lang_items)]
#![no_std]

extern "C" {
    fn bar();
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn kernel() {
    bar()
}

// Needed because we compile `dylib`...
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}
