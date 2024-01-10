/*
LLVM API
 */

extern crate llvm_sys as llvm;

use llvm::{core, prelude::*}; // change to not use wild star import
use std::{ffi::CString, path::Path, fs};

pub fn create_context() -> LLVMContextRef {
    unsafe {
        core::LLVMContextCreate()
    }
}

pub fn create_module(module_name: &str, context: LLVMContextRef) -> LLVMModuleRef {
    let c_module_name = CString::new(module_name).expect("Failed to create module name");
    unsafe {
        core::LLVMModuleCreateWithNameInContext(
            c_module_name.as_ptr(),
            context,
        )
    }
}

pub fn create_builder(context: LLVMContextRef) -> LLVMBuilderRef {
    unsafe {
        core::LLVMCreateBuilderInContext(context)
    }
}

pub fn create_integer(val: i64, context: LLVMContextRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstInt(
            core::LLVMInt64TypeInContext(context),
            val as u64,
            0 // isSigned flag
        )
    }
}

pub fn create_float(val: f64, context: LLVMContextRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstReal(core::LLVMDoubleTypeInContext(context), val)
    }
}

pub fn write_to_file(module: LLVMModuleRef, file_name: &str) {
    let output_dir = Path::new("src/backend/code_generation/target");
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

pub fn create_function(name: &str, return_type: Option<LLVMTypeRef>, param_types: &[LLVMTypeRef], 
        is_var_arg: bool, module: LLVMModuleRef) -> LLVMValueRef {
    let llvm_return_type = match return_type {
        Some(ty) => ty,
        None => unsafe { core::LLVMVoidTypeInContext(core::LLVMGetModuleContext(module)) },
    };

    let function_type = unsafe {
        core::LLVMFunctionType(llvm_return_type, param_types.as_ptr() as *mut _, param_types.len() as u32, is_var_arg as i32)
    };
    let c_name = CString::new(name).expect("Failed to create function name");
    unsafe {
        core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
    }
}


pub fn create_basic_block(context: LLVMContextRef, function: LLVMValueRef, name: &str) -> LLVMBasicBlockRef {
    let c_name = CString::new(name).expect("Failed to create basic block name");
    unsafe { 
        core::LLVMAppendBasicBlockInContext(context, function, c_name.as_ptr()) 
    }
}

pub fn position_builder_at_end(builder: *mut llvm::LLVMBuilder, bb: *mut llvm::LLVMBasicBlock) {
    unsafe {
        core::LLVMPositionBuilderAtEnd(builder, bb)
    }
}

/// returns
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

/// types
pub fn void_type(context: *mut llvm::LLVMContext) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMVoidTypeInContext(context)
    }
}

pub fn int_type(context: *mut llvm::LLVMContext) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMIntTypeInContext(context, 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> (*mut llvm::LLVMContext, *mut llvm::LLVMModule, *mut llvm::LLVMBuilder) {
        let context: *mut llvm::LLVMContext = create_context();
        let module: *mut llvm::LLVMModule = create_module("test_module", context);
        let builder: *mut llvm::LLVMBuilder = create_builder(context); // basically a pointer to where the llvm ir will write
        (context, module, builder)
    }

    #[test]
    fn test_basic_block_creation() {
        let (context, module, builder) = init();

        let return_type: *mut llvm::LLVMType = void_type(context);
        let function: *mut llvm::LLVMValue = create_function("test_function", Some(return_type), &[], false, module);

        let bb: *mut llvm::LLVMBasicBlock = create_basic_block(context, function, "entry");

        position_builder_at_end(builder, bb);

        void_return(builder);

        write_to_file(module, "output_basic.ll");
    }

    #[test]
    fn test_basic_add_expression() {
        let (context, module, builder) = init();
    
        let int_type: *mut llvm::LLVMType = int_type(context);
        let param_types = vec![int_type, int_type]; 
    
        let function: *mut llvm::LLVMValue = create_function("add", Some(int_type), &param_types, false, module);
    
        let bb: *mut llvm::LLVMBasicBlock = create_basic_block(context, function, "entry");
        position_builder_at_end(builder, bb);
    
        let param_a = unsafe { core::LLVMGetParam(function, 0) };
        let param_b = unsafe { core::LLVMGetParam(function, 1) };

        let sum = CString::new("sum").expect("Failed to create sum name");

        let sum = unsafe { core::LLVMBuildAdd(builder, param_a, param_b, sum.as_ptr()) };
    
        int_return(builder, sum);
    
        write_to_file(module, "output_add.ll");
    }
    
}
