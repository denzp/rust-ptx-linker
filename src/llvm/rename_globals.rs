use std::ffi::{CStr, CString};

use llvm_sys::core::*;
use llvm_sys::prelude::*;

use llvm::GlobalValueVisitor;

pub struct RenameGlobalsPass;

impl GlobalValueVisitor for RenameGlobalsPass {
    fn visit_global_value(&mut self, value: LLVMValueRef) -> bool {
        let current_name = unsafe { CStr::from_ptr(LLVMGetValueName(value)) };
        let updated_name = unsafe {
            CString::from_vec_unchecked(current_name.to_string_lossy().replace(".", "_").into())
        };

        unsafe {
            LLVMSetValueName(value, updated_name.as_ptr() as *const i8);
        }

        false
    }
}
