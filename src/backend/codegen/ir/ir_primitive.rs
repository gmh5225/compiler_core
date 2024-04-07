extern crate llvm_sys as llvm;

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::{element, types}
    }, 
    frontend::ast::{
        ast_struct::ASTNode, 
        data_type::DataType, 
        syntax_element::SyntaxElement,
    }
};

use llvm::prelude::{LLVMTypeRef, LLVMValueRef};

impl<T> IRGenerator<T> {
    /// Generates LLVM IR for a data type
    pub fn generate_data_type_ir(&self, data_type: &DataType) -> LLVMTypeRef {
        match data_type {
            DataType::Integer => types::int_type(self.get_context()),
            DataType::Float => types::float_type(self.get_context()),
            DataType::Boolean => types::boolean_type(self.get_context()),
            _ => unimplemented!("Unimplemented data type mapping to LLVM IR"),
        }
    }

    /// Generates LLVM IR for a literal
    pub fn generate_literal_ir(&self, node: &ASTNode) -> LLVMValueRef {
        if let SyntaxElement::Literal(value) = node.get_element() {
            if let Ok(int_val) = value.parse::<i64>() {
                element::create_integer(int_val, self.get_context())
            } else if let Ok(float_val) = value.parse::<f64>() {
                element::create_float(float_val, self.get_context())
            } else if let Ok(bool_val) = value.parse::<bool>() {
                element::create_boolean(bool_val, self.get_context())
            } else {
                unimplemented!("Unimplemented literal type")
            }
        } else {
            panic!("Expected a literal node, found {:?}", node.get_element());
        }
    }
}