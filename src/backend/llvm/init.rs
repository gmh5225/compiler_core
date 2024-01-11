extern crate llvm_sys as llvm;

use llvm::{core, prelude::*}; // change to not use wild star import
use std::ffi::CString;

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

pub fn create_basic_block(context: LLVMContextRef, function: LLVMValueRef, name: &str) -> LLVMBasicBlockRef {
    let c_name = CString::new(name).expect("Failed to create basic block name");
    unsafe { 
        core::LLVMAppendBasicBlockInContext(context, function, c_name.as_ptr()) 
    }
}


/// TESTS /// - tests entire llvm folder
#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::llvm::{
        init::{create_basic_block, create_builder, create_context, create_module},
        binops::build_add,
        return_type::{nonvoid_return, void_return},
        create_element::create_function,
        types::{int_type, void_type},
        utils::{get_param, write_to_file, position_builder_at_end}
    };

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
        let param_types: Vec<*mut llvm::LLVMType> = vec![int_type, int_type]; 
    
        let function: *mut llvm::LLVMValue = create_function("add", Some(int_type), &param_types, false, module);
    
        let bb: *mut llvm::LLVMBasicBlock = create_basic_block(context, function, "entry");
        position_builder_at_end(builder, bb);
    
        let param_a: *mut llvm::LLVMValue = get_param(function, 0);
        let param_b: *mut llvm::LLVMValue = get_param(function, 1);

        let sum: CString = CString::new("sum").expect("Failed to create sum name");

        let sum_build: *mut llvm::LLVMValue = build_add(builder, param_a, param_b, sum);
    
        nonvoid_return(builder, sum_build);
    
        write_to_file(module, "output_add.ll");
    }
    
}