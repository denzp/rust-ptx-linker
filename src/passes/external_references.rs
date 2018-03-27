use std::ffi::CStr;
use std::collections::BTreeSet;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

use llvm::CallVisitor;

pub struct FindExternalReferencesPass {
    references: BTreeSet<String>,
}

impl FindExternalReferencesPass {
    pub fn new() -> Self {
        FindExternalReferencesPass {
            references: BTreeSet::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.references.len()
    }

    pub fn references(self) -> Vec<String> {
        self.references.into_iter().collect()
    }
}

impl CallVisitor for FindExternalReferencesPass {
    fn visit_call(&mut self, caller: LLVMValueRef, callee: LLVMValueRef) -> bool {
        let callee_name = unsafe { CStr::from_ptr(LLVMGetValueName(callee)).to_string_lossy() };

        let is_declaration = unsafe { LLVMIsDeclaration(callee) == 1 };
        let is_intrinsic = callee_name.starts_with("llvm.");

        if is_declaration && !is_intrinsic {
            self.references.insert(callee_name.into());
        }

        false
    }
}
