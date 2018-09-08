use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::*;
use std::ffi::CStr;

use llvm::{FunctionVisitor, GlobalValueVisitor};

const PTX_KERNEL_CALL_CONV: u32 = 71;

pub struct InternalizePass;

impl InternalizePass {
    pub fn new() -> Self {
        InternalizePass {}
    }
}

impl FunctionVisitor for InternalizePass {
    fn visit_function(&mut self, function: LLVMValueRef) -> bool {
        let function_name = unsafe { CStr::from_ptr(LLVMGetValueName(function)).to_string_lossy() };

        let is_kernel = unsafe { LLVMGetFunctionCallConv(function) == PTX_KERNEL_CALL_CONV };
        let is_intrinsic = function_name.starts_with("llvm.");

        if !is_kernel && !is_intrinsic {
            debug!("internalizing {:?}", function_name);

            unsafe {
                LLVMSetLinkage(function, LLVMLinkage::LLVMInternalLinkage);
                LLVMSetVisibility(function, LLVMVisibility::LLVMDefaultVisibility);
            }
        }

        false
    }
}

impl GlobalValueVisitor for InternalizePass {
    fn visit_global_value(&mut self, value: LLVMValueRef) -> bool {
        unsafe {
            if LLVMIsAGlobalVariable(value) == value {
                LLVMSetLinkage(value, LLVMLinkage::LLVMInternalLinkage);
            }
        }

        false
    }
}
