use std::marker::PhantomData;
use std::ptr::null_mut;

use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub struct FunctionsIter<'a> {
    module: PhantomData<&'a LLVMModuleRef>,
    next: Option<LLVMValueRef>,
}

pub struct GlobalsIter<'a> {
    module: PhantomData<&'a LLVMModuleRef>,
    next: Option<LLVMValueRef>,
}

impl<'a> FunctionsIter<'a> {
    pub fn new(module: &'a LLVMModuleRef) -> Self {
        FunctionsIter {
            module: PhantomData::default(),
            next: Option::from_ptr(unsafe { LLVMGetFirstFunction(*module) }),
        }
    }
}

impl<'a> GlobalsIter<'a> {
    pub fn new(module: &'a LLVMModuleRef) -> Self {
        GlobalsIter {
            module: PhantomData::default(),
            next: Option::from_ptr(unsafe { LLVMGetFirstGlobal(*module) }),
        }
    }
}

impl<'a> Iterator for FunctionsIter<'a> {
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

impl<'a> Iterator for GlobalsIter<'a> {
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

trait FromPtr<T> {
    fn from_ptr(ptr: T) -> Self;
}

impl FromPtr<LLVMValueRef> for Option<LLVMValueRef> {
    fn from_ptr(ptr: LLVMValueRef) -> Self {
        if ptr == null_mut() {
            None
        } else {
            Some(ptr)
        }
    }
}
