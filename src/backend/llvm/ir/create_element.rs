use llvm::{core, prelude::*}; // change to not use wild star import
use std::ffi::CString;

/// creates an integer
pub fn create_integer(val: i64, context: LLVMContextRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstInt(
            core::LLVMInt64TypeInContext(context),
            val as u64,
            0 // isSigned flag
        )
    }
}

// creates a float
pub fn create_float(val: f64, context: LLVMContextRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstReal(core::LLVMDoubleTypeInContext(context), val)
    }
}

/// creates a boolean
pub fn create_boolean(val: bool, context: LLVMContextRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstInt(core::LLVMInt1TypeInContext(context), val as u64, 0)
    }
}

/// creates a function type
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

/// creates an array
pub fn create_array(value: LLVMValueRef, num_elements: u64) -> LLVMValueRef {
    let values = vec![value; num_elements as usize];
    unsafe {
        core::LLVMConstArray2(core::LLVMTypeOf(value), values.as_ptr() as *mut _, num_elements)
    }
}

/// creates a pointer
pub fn create_pointer(value: LLVMValueRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstPointerNull(core::LLVMPointerType(core::LLVMTypeOf(value), 0))
    }
}

/// creates a struct
pub fn create_struct(values: &[LLVMValueRef], context: LLVMContextRef, packed: bool) -> LLVMValueRef {
    unsafe {
        core::LLVMConstStructInContext(context, values.as_ptr() as *mut _, values.len() as u32, packed as i32)
    }
}

/// creates a global variable
pub fn create_global_variable(module: LLVMModuleRef, initializer: LLVMValueRef, name: &str) -> LLVMValueRef {
    let c_name = CString::new(name).expect("Failed to create global variable name");
    unsafe {
        let global_var = core::LLVMAddGlobal(module, core::LLVMTypeOf(initializer), c_name.as_ptr());
        core::LLVMSetInitializer(global_var, initializer);
        global_var
    }
}

/// creates a string
pub fn create_string(val: &str, builder: LLVMBuilderRef) -> LLVMValueRef {
    let c_val = CString::new(val).expect("Failed to create string");
    let c_str_name = CString::new("const_str").expect("Failed to create string name");
    unsafe {
        core::LLVMBuildGlobalStringPtr(builder, c_val.as_ptr() as *const i8, c_str_name.as_ptr())
    }
}

/// creates a null pointer
pub fn create_null_pointer(ty: LLVMTypeRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstPointerNull(ty)
    }
}
