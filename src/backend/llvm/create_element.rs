extern crate llvm_sys as llvm;

use llvm::{core, prelude::*}; // change to not use wild star import
use std::ffi::CString;

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