//~ GLOBAL-ERROR-REGEX linking with `(.+)ptx-linker` failed: exit code: 1

//~ GLOBAL-NOTE-REGEX ptx-linker(.+)-o(.+)example.ptx
//~ GLOBAL-NOTE-REGEX Unable to link modules
//~ GLOBAL-NOTE-REGEX Undefined references: \["bar"\]

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
