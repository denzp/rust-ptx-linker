use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::BTreeSet;
use std::ffi::CStr;

use llvm::CallVisitor;

pub const SYSCALLS: &[&str] = &["vprintf", "__assertfail", "malloc", "free"];

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
    fn visit_call(&mut self, _caller: LLVMValueRef, callee: LLVMValueRef) -> bool {
        let mut len = 0;
        let callee_name = unsafe { CStr::from_ptr(LLVMGetValueName2(callee, &mut len)).to_string_lossy() };

        let is_declaration = unsafe { LLVMIsDeclaration(callee) == 1 };
        let is_intrinsic = callee_name.starts_with("llvm.");
        let is_syscall = SYSCALLS.contains(&callee_name.as_ref());

        if is_declaration && !is_intrinsic && !is_syscall {
            self.references.insert(callee_name.into());
        }

        false
    }
}
