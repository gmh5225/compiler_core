extern crate llvm_sys as llvm;

use std::ffi::CString;
use std::path::PathBuf;
use compiler_core::backend::llvm_lib::ir_lib::{
    init_ir::{create_basic_block, create_builder, create_context, create_module},
    ops::build_add,
    return_type::{nonvoid_return, void_return},
    element::create_function,
    types::{int_type, void_type},
    utils::{get_param, write_to_file, position_builder}
};
use compiler_core::backend::execute::execution_engine::ExecutionEngine;

fn init() -> (*mut llvm::LLVMContext, *mut llvm::LLVMModule, *mut llvm::LLVMBuilder) {
    let context: *mut llvm::LLVMContext = create_context();
    let module: *mut llvm::LLVMModule = create_module("test_module", context);
    let builder: *mut llvm::LLVMBuilder = create_builder(context);
    (context, module, builder)
}

fn generate_basic_block() {
    let (context, module, builder) = init();

    let return_type: *mut llvm::LLVMType = void_type(context);
    let function: *mut llvm::LLVMValue = create_function("test_function", Some(return_type), &[], false, module);

    let bb: *mut llvm::LLVMBasicBlock = create_basic_block(context, function, "entry");

    position_builder(builder, bb);

    void_return(builder);

    write_to_file(module, "output_basic.ll");
}

fn generate_add_expression() {
    let (context, module, builder) = init();

    let int_type: *mut llvm::LLVMType = int_type(context);
    let param_types: Vec<*mut llvm::LLVMType> = vec![int_type, int_type];

    let function: *mut llvm::LLVMValue = create_function("add", Some(int_type), &param_types, false, module);

    let bb: *mut llvm::LLVMBasicBlock = create_basic_block(context, function, "entry");
    position_builder(builder, bb);

    let param_a: *mut llvm::LLVMValue = get_param(function, 0);
    let param_b: *mut llvm::LLVMValue = get_param(function, 1);

    let sum: CString = CString::new("sum").expect("Failed to create sum name");

    let sum_build: *mut llvm::LLVMValue = build_add(builder, param_a, param_b, sum);

    nonvoid_return(builder, sum_build);

    write_to_file(module, "output_add.ll");
}

fn run_codegen_test() {
    let file: PathBuf = PathBuf::from("./src/backend/codegen/target/output_add.ll");

    assert!(file.exists(), "The specified LLVM IR file does not exist");

    let result: Result<i64, String> = ExecutionEngine::execute_ir(file.to_str().unwrap(), "add", &[10, 5]);

    assert_eq!(result.unwrap(), 15, "The add function did not execute correctly");
}

#[test]
fn test_llvm_ir_generation_and_execution() {
    generate_basic_block();
    generate_add_expression();

    run_codegen_test();
}
