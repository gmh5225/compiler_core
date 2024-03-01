use llvm::prelude::LLVMContextRef;
use llvm::core::*;
use llvm::execution_engine::*;
use llvm::LLVMModule;
use llvm_sys::ir_reader::LLVMParseIRInContext;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;
use std::path::PathBuf;

use crate::backend::llvm_lib::{
    ir_lib::init_ir,
    ee_lib::init_ee
};

pub struct ExecutionEngine {
    engine: LLVMExecutionEngineRef,
    context: LLVMContextRef,
}


impl ExecutionEngine {
    fn new() -> Self {
        init_ee::init_ee_targets();

        let mut context: *mut llvm::LLVMContext = init_ir::create_context();
        let mut engine: *mut LLVMOpaqueExecutionEngine = std::ptr::null_mut(); 
        (context, engine) = init_ee::init_engine(context, engine);

        if context.is_null() {
            panic!("Context is null at initialization");
        }
        if engine.is_null() {
            panic!("Engine is null at initialization");
        }
        ExecutionEngine { engine, context }
    }

    pub fn execute_ir(module: *mut LLVMModule, args: &[String]) -> Result<i64, String> {
        let mut ee: ExecutionEngine = ExecutionEngine::new();
        ee.run_file(module, "main", args)
    }

    fn run_file(&mut self, module: *mut LLVMModule, function_name: &str, args: &[String]) -> Result<i64, String> {
        unsafe {
            // Ensure the module is valid
            if module.is_null() {
                return Err("Invalid module pointer".into());
            }

            let function_name_cstr = CString::new(function_name).map_err(|_| "Failed to create CString for function name")?;
            let function = LLVMGetNamedFunction(module, function_name_cstr.as_ptr());
            if function.is_null() {
                return Err("Function not found".into());
            }
            let mut generic_values = Vec::with_capacity(args.len());
            for arg in args {
                let arg_cstr = CString::new(arg.as_str()).unwrap();
                let generic_value = LLVMCreateGenericValueOfPointer(arg_cstr.as_ptr() as *mut _);
                generic_values.push(generic_value);
            }

            // Execute the function
            let result = LLVMRunFunction(self.engine, function, generic_values.len() as u32, generic_values.as_mut_ptr());
            let return_val = LLVMGenericValueToInt(result, 0);

            // Cleanup generic values after execution
            for gv in generic_values {
                LLVMDisposeGenericValue(gv);
            }

            Ok(return_val as i64)
        }
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.engine);
            LLVMContextDispose(self.context);
        }
    }
}
