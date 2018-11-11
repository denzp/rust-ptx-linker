use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::{CStr, CString};

use llvm::{FunctionVisitor, GlobalValueVisitor};

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
        let mut len = 0;
        let current_name = unsafe { CStr::from_ptr(LLVMGetValueName2(value, &mut len)).to_string_lossy() };

        if current_name.contains(".") {
            let updated_name =
                unsafe { CString::from_vec_unchecked(current_name.replace(".", "_").into()) };

            unsafe {
                LLVMSetValueName2(value, updated_name.as_ptr() as *const i8, updated_name.to_bytes_with_nul().len() as _);
            }
        }

        false
    }
}

impl FunctionVisitor for RenameFunctionsPass {
    fn visit_function(&mut self, value: LLVMValueRef) -> bool {
        let mut len = 0;
        let current_name = unsafe { CStr::from_ptr(LLVMGetValueName2(value, &mut len)).to_string_lossy() };

        if current_name.contains("..") {
            let updated_name =
                unsafe { CString::from_vec_unchecked(current_name.replace(".", "_").into()) };

            unsafe {
                LLVMSetValueName2(value, updated_name.as_ptr() as *const i8, updated_name.to_bytes_with_nul().len() as _);
            }
        }

        false
    }
}
