extern crate llvm_sys as llvm;

use llvm::{core, prelude::LLVMBasicBlockRef};

pub fn position_builder(builder: *mut llvm::LLVMBuilder, bb: *mut llvm::LLVMBasicBlock) {
    unsafe {
        core::LLVMPositionBuilderAtEnd(builder, bb);
    }
}

pub fn get_current_block(builder: *mut llvm::LLVMBuilder) -> LLVMBasicBlockRef {
    unsafe {
        core::LLVMGetInsertBlock(builder)
    }
}
