use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::{CStr, CString};

use crate::llvm::{FunctionVisitor, GlobalValueVisitor};

pub struct RenameGlobalsPass;
pub struct RenameFunctionsPass;

impl RenameGlobalsPass {
    pub fn new() -> Self {
        RenameGlobalsPass {}
    }
}

impl RenameFunctionsPass {
    pub fn new() -> Self {
        RenameFunctionsPass {}
    }
}

impl GlobalValueVisitor for RenameGlobalsPass {
    fn visit_global_value(&mut self, value: LLVMValueRef) -> bool {
        let current_name = unsafe { CStr::from_ptr(LLVMGetValueName(value)).to_string_lossy() };

        if current_name.contains('.') {
            let updated_name =
                unsafe { CString::from_vec_unchecked(current_name.replace(".", "_").into()) };

            unsafe {
                LLVMSetValueName(value, updated_name.as_ptr() as *const i8);
            }
        }

        false
    }
}

impl FunctionVisitor for RenameFunctionsPass {
    fn visit_function(&mut self, value: LLVMValueRef) -> bool {
        let current_name = unsafe { CStr::from_ptr(LLVMGetValueName(value)).to_string_lossy() };

        if current_name.contains("..") {
            let updated_name =
                unsafe { CString::from_vec_unchecked(current_name.replace(".", "_").into()) };

            unsafe {
                LLVMSetValueName(value, updated_name.as_ptr() as *const i8);
            }
        }

        false
    }
}
