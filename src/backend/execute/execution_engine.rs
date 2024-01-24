use llvm::prelude::LLVMContextRef;
use llvm::core::*;
use llvm::execution_engine::*;
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

    pub fn execute_ir(file_path: &str, args: &[String]) -> Result<i64, String> {
        let absolute_path: PathBuf = PathBuf::from(file_path);

        let ir_code: String = fs::read_to_string(absolute_path)
            .map_err(|e| format!("Error reading file: {}", e))?;

        let mut ee: ExecutionEngine = ExecutionEngine::new();
        ee.run_file(&ir_code, "main", args)
    }

    fn run_file(&mut self, ir_code: &str, function_name: &str, args: &[String]) -> Result<i64, String> {
        let c_ir_code = CString::new(ir_code).map_err(|_| "Failed to create CString from ir_code")?;
    
        let buffer_name = CString::new("ir_buffer").map_err(|_| "Failed to create CString for buffer_name")?;
        let ir_buffer = unsafe {
            LLVMCreateMemoryBufferWithMemoryRange(
                c_ir_code.as_ptr(), 
                ir_code.len(), 
                buffer_name.as_ptr(), 
                0
            )
        };
    
        let mut module = std::ptr::null_mut();
        let mut err = std::ptr::null_mut();
        let result = unsafe {
            LLVMParseIRInContext(self.context, ir_buffer, &mut module, &mut err)
        };
    
        if result != 0 {
            let err_msg = unsafe { CStr::from_ptr(err).to_string_lossy().into_owned() };
            return Err(err_msg);
        }
    
        let c_args: Vec<CString> = args.iter().map(|arg| CString::new(arg.as_str()).unwrap()).collect();
        let c_args_ptrs: Vec<*const i8> = c_args.iter().map(|arg| arg.as_ptr()).collect();
    
        let function = unsafe { LLVMGetNamedFunction(module, CString::new(function_name).unwrap().as_ptr()) };
        if function.is_null() {
            return Err("No main found".to_string());
        }
    
        let args_values = c_args_ptrs
        .iter()
        .map(|&arg| unsafe { LLVMCreateGenericValueOfPointer(arg as *mut _) })
        .collect::<Vec<LLVMGenericValueRef>>();
    
        let mut args_values_ptrs = args_values
            .iter()
            .map(|&val| val as *mut LLVMOpaqueGenericValue)
            .collect::<Vec<*mut LLVMOpaqueGenericValue>>();

        let args_values_ptrs_mut = args_values_ptrs.as_mut_ptr();

        let result = unsafe {
            LLVMRunFunction(
                self.engine, 
                function, 
                args_values_ptrs.len() as u32, 
                args_values_ptrs_mut
            )
        };
    
        let return_val = unsafe { LLVMGenericValueToInt(result, 0) };
    
        Ok(return_val as i64)
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
