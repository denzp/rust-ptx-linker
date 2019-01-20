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
        let current_name = unsafe {
            let mut current_name_len = 0;

            CStr::from_ptr(LLVMGetValueName2(value, &mut current_name_len)).to_string_lossy()
        };

        if current_name.contains('.') {
            let updated_name = current_name.replace(".", "_");

            let updated_name_len = updated_name.len();
            let updated_name_ffi = unsafe { CString::from_vec_unchecked(updated_name.into()) };

            unsafe {
                LLVMSetValueName2(
                    value,
                    updated_name_ffi.as_ptr() as *const i8,
                    updated_name_len,
                );
            }
        }

        false
    }
}

impl FunctionVisitor for RenameFunctionsPass {
    fn visit_function(&mut self, value: LLVMValueRef) -> bool {
        let current_name = unsafe {
            let mut current_name_len = 0;

            CStr::from_ptr(LLVMGetValueName2(value, &mut current_name_len)).to_string_lossy()
        };

        if current_name.contains("..") {
            let updated_name = current_name.replace(".", "_");

            let updated_name_len = updated_name.len();
            let updated_name_ffi = unsafe { CString::from_vec_unchecked(updated_name.into()) };

            unsafe {
                LLVMSetValueName2(
                    value,
                    updated_name_ffi.as_ptr() as *const i8,
                    updated_name_len,
                );
            }
        }

        false
    }
}
