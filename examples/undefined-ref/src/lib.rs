//~ GLOBAL-ERROR-REGEX linking with `(.+)ptx-linker` failed: exit code: 1

//~ GLOBAL-NOTE-REGEX ptx-linker(.+)-o(.+)example.ptx
//~ GLOBAL-NOTE-REGEX Unable to link modules
//~ GLOBAL-NOTE-REGEX Undefined references: \["bar"\]

#![deny(warnings)]
#![feature(abi_ptx)]
#![no_std]

extern "C" {
    fn bar();
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn kernel() {
    bar();
}

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}
