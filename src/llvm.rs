use std::ptr;
use std::fmt;
use std::ffi::CStr;
use cty::{c_char, c_uint};

pub use rustc_llvm::*;

extern "C" {
    pub fn LLVMModuleCreateWithName(id: *const c_char) -> ModuleRef;
    pub fn LLVMGetTarget(module: ModuleRef) -> *const c_char;

    pub fn LLVMDisposeMessage(message: *const c_char);

    pub fn LLVMPrintModuleToFile(module: ModuleRef,
                                 file_path: *const c_char,
                                 out_message: &mut Message)
                                 -> Bool;

    pub fn LLVMPassManagerBuilderSetOptLevel(builder: PassManagerBuilderRef, opt_level: c_uint);

    pub fn LLVMInitializeNVPTXTargetInfo();
    pub fn LLVMInitializeNVPTXTarget();
    pub fn LLVMInitializeNVPTXTargetMC();
    pub fn LLVMInitializeNVPTXAsmPrinter();

    /// Returns count of external references that are found.
    /// Also writes semicolon (";") separated list to the `out_messages`.
    ///
    /// Defined in `llvm/find-external-refs.cpp`
    pub fn FindExternalReferences(module: ModuleRef, out_message: &mut Message) -> c_uint;

    // Remove every function but kernels and their dependent functions.
    ///
    /// Defined in `llvm/internalize.cpp`
    pub fn StripInternalFunctions(module: ModuleRef);
}

/// Convinient LLVM Message pointer wrapper.
/// Does not own the ptr, so we have to call `LLVMDisposeMessage` to free message memory.
#[repr(C)]
pub struct Message {
    ptr: *const c_char,
}

impl Message {
    pub fn new() -> Self {
        Message { ptr: ptr::null() }
    }

    pub fn is_empty(&self) -> bool {
        self.ptr == ptr::null()
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        if !self.is_empty() {
            unsafe {
                LLVMDisposeMessage(self.ptr);
            }
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_empty() {
            let contents = unsafe { CStr::from_ptr(self.ptr).to_str().unwrap() };
            write!(f, "{}", contents)
        } else {
            write!(f, "(empty)")
        }
    }
}
