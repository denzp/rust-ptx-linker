//~ GLOBAL-ERROR-REGEX linking with `(.+)ptx-linker` failed: exit code: 1

//~ GLOBAL-NOTE-REGEX ptx-linker(.+)-o(.+)example.ptx
//~ GLOBAL-NOTE-REGEX Unable to link modules
//~ GLOBAL-NOTE-REGEX Undefined references: \["bar"\]

#![deny(warnings)]
#![feature(abi_ptx)]
#![no_std]

// Actual "undefined reference"
extern "C" {
    fn bar();
}

// Syscalls that are allowed
extern "C" {
    pub fn vprintf(format: *const u8, valist: *const u8) -> i32;
    pub fn malloc(size: u64) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn kernel() {
    bar();

    vprintf("allocating memory".as_ptr(), [].as_ptr());
    let ptr = malloc(32);

    vprintf("writing into the memory".as_ptr(), [].as_ptr());
    *ptr.offset(0) = 128;

    vprintf("releasing memory".as_ptr(), [].as_ptr());
    free(ptr);
}

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}
