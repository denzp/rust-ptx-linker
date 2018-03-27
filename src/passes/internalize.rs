use std::collections::BTreeSet;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use llvm::{CallVisitor, FunctionVisitor};

const PTX_KERNEL_CALL_CONV: u32 = 71;

pub struct FindInternalFunctionsPass {
    used_functions: BTreeSet<LLVMValueRef>,
}

pub struct RemoveInternalFunctionsPass {
    exceptions: BTreeSet<LLVMValueRef>,
}

impl FindInternalFunctionsPass {
    pub fn new() -> Self {
        FindInternalFunctionsPass {
            used_functions: BTreeSet::new(),
        }
    }

    pub fn into_remove_pass(self) -> RemoveInternalFunctionsPass {
        RemoveInternalFunctionsPass {
            exceptions: self.used_functions,
        }
    }
}

impl CallVisitor for FindInternalFunctionsPass {
    fn visit_call(&mut self, caller: LLVMValueRef, callee: LLVMValueRef) -> bool {
        if self.used_functions.contains(&callee) {
            return false;
        }

        let is_caller_kernel = unsafe { LLVMGetFunctionCallConv(caller) == PTX_KERNEL_CALL_CONV };

        if is_caller_kernel {
            self.used_functions.insert(callee);
            return true;
        }

        if self.used_functions.contains(&caller) {
            self.used_functions.insert(callee);
            return true;
        }

        false
    }
}

impl FunctionVisitor for RemoveInternalFunctionsPass {
    fn visit_function(&mut self, function: LLVMValueRef) -> bool {
        let is_kernel = unsafe { LLVMGetFunctionCallConv(function) == PTX_KERNEL_CALL_CONV };

        if is_kernel || self.exceptions.contains(&function) {
            return false;
        }

        unsafe {
            LLVMReplaceAllUsesWith(function, LLVMGetUndef(LLVMTypeOf(function)));
            LLVMDeleteFunction(function);
        }

        false
    }
}
