extern crate llvm_sys as llvm;

use std::ops::Deref;

use llvm::prelude::LLVMValueRef;


#[derive(Debug, Clone, Copy)]
pub struct IRPointer {
    ptr: *mut LLVMValueRef, 
}

impl IRPointer {
    fn new(ptr: *mut LLVMValueRef) -> Self {
        IRPointer { ptr }
    }

    fn as_ref(&self) -> LLVMValueRef {
        unsafe { *self.ptr }
    }
}

impl Deref for IRPointer {
    type Target = LLVMValueRef;

    fn deref(&self) -> &Self::Target {
        self
    }
}