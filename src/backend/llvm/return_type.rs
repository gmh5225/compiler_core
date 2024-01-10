extern crate llvm_sys as llvm;

use llvm::{core, prelude::*}; // change to not use wild star import

pub fn void_return(builder: *mut llvm::LLVMBuilder) -> LLVMValueRef {
    unsafe {
        core::LLVMBuildRetVoid(builder)
    }
}

pub fn int_return(builder: *mut llvm::LLVMBuilder, value: LLVMValueRef) -> LLVMValueRef {
    unsafe {
        core::LLVMBuildRet(builder, value)
    }
}