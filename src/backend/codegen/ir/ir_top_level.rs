use crate::{
    backend::codegen::ir::ir_codegen_core::IRGenerator, 
    frontend::ast::{
        syntax_element::FunctionParameter, 
        data_type::DataType
    }, 
};

use llvm::prelude::LLVMValueRef;

impl IRGenerator {
    pub fn generate_fn_declaration_ir(&mut self, name: &String, parameters: &Vec<FunctionParameter>, return_type: &Option<DataType>) -> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_enum_declaration_ir(&mut self, name: &String, variants: &Vec<String>) -> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_struct_declaration_ir(&mut self, name: &String, fields: &Vec<(String, DataType)>) -> LLVMValueRef {
        std::ptr::null_mut()
   
    }
}