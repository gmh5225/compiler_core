use llvm::{core, prelude::*}; // change to not use wild star import
use std::{ffi::CString, path::Path, fs};

pub fn get_param(function: *mut llvm::LLVMValue, index: u32) -> *mut llvm::LLVMValue{
    unsafe {
        core::LLVMGetParam(function, index)
    }
}

pub fn write_to_file(module: LLVMModuleRef, file_name: &str) {
    let output_dir = Path::new("src/backend/codegen/target");
    let output_file_path = output_dir.join(file_name);

    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Failed to create target directory");
    }
    
    let output_file_cstr = CString::new(output_file_path.to_str()
        .expect("Failed to convert path to string"))
        .expect("Failed to create CString for filename");
    unsafe {
        core::LLVMPrintModuleToFile(module, output_file_cstr.as_ptr(), std::ptr::null_mut());
    }
}


pub fn position_builder_at_end(builder: *mut llvm::LLVMBuilder, bb: *mut llvm::LLVMBasicBlock) {
    unsafe {
        core::LLVMPositionBuilderAtEnd(builder, bb)
    }
}