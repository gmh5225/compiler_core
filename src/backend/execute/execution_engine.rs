extern crate llvm_sys as llvm;

use llvm::ir_reader::LLVMParseIRInContext;
use llvm::prelude::LLVMBool;
use llvm::prelude::LLVMContextRef;
use llvm::core::*;
use llvm::execution_engine::*;
use llvm::target::*;
use std::ffi::CStr;
use std::ffi::CString;

pub struct ExecutionEngine {
    engine: LLVMExecutionEngineRef,
    context: LLVMContextRef,
}

use std::fs;
use std::path::PathBuf;

impl ExecutionEngine {
    fn new() -> Self {
        // initialize LLVM components
        unsafe {
            LLVM_InitializeAllTargetInfos();
            LLVM_InitializeAllTargets();
            LLVM_InitializeAllTargetMCs();
            LLVM_InitializeAllAsmParsers();
            LLVM_InitializeAllAsmPrinters();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmParser();
            LLVM_InitializeNativeAsmPrinter();
            LLVMLinkInMCJIT();
        }

        let context: *mut llvm::LLVMContext = unsafe { LLVMContextCreate() };
        let mut engine: *mut LLVMOpaqueExecutionEngine = std::ptr::null_mut();
        unsafe {
            let mut tmp_module: *mut llvm::LLVMModule = std::ptr::null_mut();
            tmp_module = LLVMModuleCreateWithNameInContext(CString::new("temp").unwrap().as_ptr(), context);
            if tmp_module.is_null() {
                panic!("Failed to create temporary module");
            }

            let mut out_error: *mut i8 = std::ptr::null_mut();
            let result = LLVMCreateExecutionEngineForModule(&mut engine, tmp_module, &mut out_error);
            if result != 0 {
                let error_str = CStr::from_ptr(out_error).to_str().unwrap_or("Unknown error").to_owned();
                LLVMDisposeMessage(out_error);
                panic!("{}", error_str)
            }
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
        let ir = CString::new(ir_code).unwrap();
        let buffer_name = CString::new("abc").expect("Cstring failed");
        if ir.as_ptr().is_null() {
            panic!("Null pointer encountered in ir.as_ptr()");
        }
        let mut module: *mut llvm::LLVMModule = std::ptr::null_mut();
        unsafe {
            // parse the LLVM IR
            let memory_buffer: *mut llvm::LLVMMemoryBuffer = 
                LLVMCreateMemoryBufferWithMemoryRange(
                ir.as_ptr(),
                ir.as_bytes().len(),
                buffer_name.as_ptr(),
                0 as LLVMBool,
            );
            LLVMParseIRInContext(self.context, memory_buffer, &mut module, std::ptr::null_mut());

            // // create an execution engine
            // error happens here
            // Ensure 'module' is not null or invalid
            if module.is_null() {
                panic!("Module is null");
            }
            if self.engine.is_null() {
                panic!("Engine is null");
            }
            // Attempt to create the execution engine
            if LLVMCreateExecutionEngineForModule(&mut self.engine, module, std::ptr::null_mut()) != 0 {
                return Err("Failed to create execution engine".to_string());
            }

            // // lookup the function
            let function_name_c = CString::new(function_name).unwrap();
            let function_address = LLVMGetFunctionAddress(self.engine, function_name_c.as_ptr());
            if function_address == 0 {
                return Err("Function not found".to_string());
            }

            // define the function type
            let add_function: extern "C" fn(i64, i64) -> i64 = std::mem::transmute(function_address);

            // execute the function
            let result = add_function(args[0], args[1]);

            Ok(result)
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


