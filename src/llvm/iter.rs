use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub trait FunctionsIterableModule {
    fn functions_iter(&self) -> FunctionIter;
}

pub trait GlobalsIterableModule {
    fn globals_iter(&self) -> GlobalIter;
}

pub trait BlocksIterableFunction {
    fn blocks_iter(&self) -> BlockIter;
}

pub trait InstructionsIterableBlock {
    fn instructions_iter(&self) -> InstructionIter;
}

pub struct FunctionIter<'a> {
    module: PhantomData<&'a LLVMModuleRef>,
    next: Option<LLVMValueRef>,
}

pub struct GlobalIter<'a> {
    module: PhantomData<&'a LLVMModuleRef>,
    next: Option<LLVMValueRef>,
}

pub struct BlockIter<'a> {
    function: PhantomData<&'a LLVMValueRef>,
    next: Option<LLVMBasicBlockRef>,
}

pub struct InstructionIter<'a> {
    block: PhantomData<&'a LLVMBasicBlockRef>,
    next: Option<LLVMValueRef>,
}

impl FunctionsIterableModule for LLVMModuleRef {
    fn functions_iter(&self) -> FunctionIter {
        FunctionIter::new(self)
    }
}

impl<'a> FunctionIter<'a> {
    pub fn new(module: &'a LLVMModuleRef) -> Self {
        FunctionIter {
            module: PhantomData::default(),
            next: Option::from_ptr(unsafe { LLVMGetFirstFunction(*module) }),
        }
    }
}

impl<'a> Iterator for FunctionIter<'a> {
    type Item = LLVMValueRef;

    fn next(&mut self) -> Option<LLVMValueRef> {
        let next = self.next;

        self.next = match next {
            Some(ref next) => Option::from_ptr(unsafe { LLVMGetNextFunction(*next) }),
            None => None,
        };

        next
    }
}

impl GlobalsIterableModule for LLVMModuleRef {
    fn globals_iter(&self) -> GlobalIter {
        GlobalIter::new(self)
    }
}

impl<'a> GlobalIter<'a> {
    pub fn new(module: &'a LLVMModuleRef) -> Self {
        GlobalIter {
            module: PhantomData::default(),
            next: Option::from_ptr(unsafe { LLVMGetFirstGlobal(*module) }),
        }
    }
}

impl<'a> Iterator for GlobalIter<'a> {
    type Item = LLVMValueRef;

    fn next(&mut self) -> Option<LLVMValueRef> {
        let next = self.next;

        self.next = match next {
            Some(ref next) => Option::from_ptr(unsafe { LLVMGetNextGlobal(*next) }),
            None => None,
        };

        next
    }
}

impl BlocksIterableFunction for LLVMValueRef {
    fn blocks_iter(&self) -> BlockIter {
        BlockIter::new(self)
    }
}

impl<'a> BlockIter<'a> {
    pub fn new(function: &'a LLVMValueRef) -> Self {
        BlockIter {
            function: PhantomData::default(),
            next: Option::from_ptr(unsafe { LLVMGetFirstBasicBlock(*function) }),
        }
    }
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = LLVMBasicBlockRef;

    fn next(&mut self) -> Option<LLVMBasicBlockRef> {
        let next = self.next;

        self.next = match next {
            Some(ref next) => Option::from_ptr(unsafe { LLVMGetNextBasicBlock(*next) }),
            None => None,
        };

        next
    }
}

impl InstructionsIterableBlock for LLVMBasicBlockRef {
    fn instructions_iter(&self) -> InstructionIter {
        InstructionIter::new(self)
    }
}

impl<'a> InstructionIter<'a> {
    pub fn new(block: &'a LLVMBasicBlockRef) -> Self {
        InstructionIter {
            block: PhantomData::default(),
            next: Option::from_ptr(unsafe { LLVMGetFirstInstruction(*block) }),
        }
    }
}

impl<'a> Iterator for InstructionIter<'a> {
    type Item = LLVMValueRef;

    fn next(&mut self) -> Option<LLVMValueRef> {
        let next = self.next;

        self.next = match next {
            Some(ref next) => Option::from_ptr(unsafe { LLVMGetNextInstruction(*next) }),
            None => None,
        };

        next
    }
}

trait FromPtr<T> {
    fn from_ptr(ptr: T) -> Self;
}

impl FromPtr<LLVMValueRef> for Option<LLVMValueRef> {
    fn from_ptr(ptr: LLVMValueRef) -> Self {
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }
}

impl FromPtr<LLVMBasicBlockRef> for Option<LLVMBasicBlockRef> {
    fn from_ptr(ptr: LLVMBasicBlockRef) -> Self {
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }
}
