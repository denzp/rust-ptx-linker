use llvm_sys::prelude::*;

pub trait CallVisitor {
    fn visit_call(&mut self) -> bool;
}

pub trait FunctionVisitor {
    fn visit_function(&mut self) -> bool;
}

pub trait GlobalValueVisitor {
    fn visit_global_value(&mut self, value: LLVMValueRef) -> bool;
}

mod iter;
use self::iter::{FunctionsIter, GlobalsIter};

mod rename_globals;
pub use self::rename_globals::RenameGlobalsPass;

pub struct PassRunner {
    module: LLVMModuleRef,
}

impl PassRunner {
    pub fn new(module: LLVMModuleRef) -> Self {
        PassRunner { module }
    }

    pub fn globals_iter<'a>(&'a self) -> GlobalsIter<'a> {
        GlobalsIter::new(&self.module)
    }

    pub fn functions_iter<'a>(&'a self) -> FunctionsIter<'a> {
        FunctionsIter::new(&self.module)
    }

    pub fn run<V: GlobalValueVisitor>(&self, visitor: &mut V) {
        let mut touched = true;

        while touched {
            touched = false;

            for value in self.globals_iter() {
                touched |= visitor.visit_global_value(value);
            }
        }
    }
}
