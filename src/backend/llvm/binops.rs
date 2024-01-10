extern crate llvm_sys as llvm;

use llvm::core; // change to not use wild star import
use std::ffi::CString;

pub fn build_add(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, sum: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildAdd(builder, param_a, param_b, sum.as_ptr())
    }
}