use std::ffi::CStr;

use llvm_sys::core::*;
use llvm_sys::prelude::*;

use llvm::CallVisitor;

pub struct FindExternalReferencesPass {
    references: Vec<String>,
}

impl FindExternalReferencesPass {
    pub fn new() -> Self {
        FindExternalReferencesPass { references: vec![] }
    }

    pub fn count(&self) -> usize {
        self.references.len()
    }

    pub fn references(self) -> Vec<String> {
        self.references
    }
}

impl CallVisitor for FindExternalReferencesPass {
    fn visit_call(&mut self, instruction: LLVMValueRef) -> bool {
        let callee = unsafe { LLVMGetCalledValue(instruction) };
        let callee_name = unsafe { CStr::from_ptr(LLVMGetValueName(callee)).to_string_lossy() };

        let is_declaration = unsafe { LLVMIsDeclaration(callee) == 1 };
        let is_intrinsic = callee_name.starts_with("llvm.");

        if is_declaration && !is_intrinsic {
            self.references.push(callee_name.into());
        }

        false
    }
}
