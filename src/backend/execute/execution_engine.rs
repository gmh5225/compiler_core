use llvm::prelude::LLVMContextRef;
use llvm::core::*;
use llvm::execution_engine::*;
use std::ffi::CString;

use crate::backend::llvm_lib::{
    ir_lib::init_ir,
    ee_lib::{
        init_ee,
        utils,
    }
};

pub struct ExecutionEngine {
    engine: LLVMExecutionEngineRef,
    context: LLVMContextRef,
}

use std::fs;
use std::path::PathBuf;

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

    pub fn execute_ir(file_path: &str, function_name: &str, args: &[i64]) 
        -> Result<i64, String> {
        let absolute_path: PathBuf = PathBuf::from(file_path);

        let ir_code: String = fs::read_to_string(absolute_path)
            .map_err(|e| format!("Error reading file: {}", e))?;

        let mut ee: ExecutionEngine = ExecutionEngine::new();
        ee.run_file(&ir_code, function_name, args)
    }

    fn run_file(&mut self, ir_code: &str, function_name: &str, args: &[i64]) 
            -> Result<i64, String> {
        let ir: CString = CString::new(ir_code).unwrap();
        let buffer_name = CString::new("abc").expect("Cstring failed");
        if ir.as_ptr().is_null() {
            panic!("Null pointer encountered in ir.as_ptr()");
        }
        let module: *mut llvm::LLVMModule = std::ptr::null_mut(); 
        let result: i64 = utils::parse_ir(ir, buffer_name, self.engine,
                            module, args, function_name, self.context);
        Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*; 
    use std::path::PathBuf;

    #[test]
    fn run_file_from_codegen_target() {
        let file: PathBuf = PathBuf::from("./src/backend/codegen/target/output_add.ll");

        assert!(file.exists(), "The specified LLVM IR file does not exist");

        let result: Result<i64, String> = ExecutionEngine::execute_ir(file.to_str().unwrap(), "add", &[10, 5]);

        assert_eq!(result.unwrap(), 15, "The add function did not execute correctly");
    }
}


