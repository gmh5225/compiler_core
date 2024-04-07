
use llvm::prelude::LLVMTypeRef;

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::types::{boolean_type, int_type, float_type}
    }, 
    frontend::ast::data_type::DataType
};

impl IRGenerator {
    /// Maps data types to LLVM IR
    pub fn map_data_type(&self, data_type: &DataType) -> LLVMTypeRef {
        match data_type {
            DataType::Boolean => boolean_type(self.get_context()),
            DataType::Integer => int_type(self.get_context()),
            DataType::Float => float_type(self.get_context()),
            _ => unimplemented!("Unimplemented ir data type")
        }
    }
}