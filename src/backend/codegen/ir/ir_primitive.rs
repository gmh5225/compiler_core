use llvm_sys::prelude::LLVMValueRef;

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::element
    }, 
    frontend::ast::{ast_struct::ASTNode, data_type::DataType}
};

impl IRGenerator {
    /// Generates LLVM IR for a literal
    pub fn generate_literal_ir(&self, node: &ASTNode) -> LLVMValueRef {
        // match data_type {
        //     DataType::Integer => {
        //         let val: i64 = match value.parse::<i64>() {
        //             Ok(val) => val,
        //             Err(e) => panic!("Failed to parse integer: {}", e),
        //         };
        //         element::create_integer(val, self.get_context()) 
        //     },
        //     DataType::Float => {
        //         let val: f64 = match value.parse::<f64>() {
        //             Ok(val) => val,
        //             Err(e) => panic!("Failed to parse floating point: {}", e),
        //         };
        //         element::create_float(val, self.get_context())
        //     },
        //     DataType::Boolean => {
        //         let val: bool = match value.parse::<bool>() {
        //             Ok(val) => val,
        //             Err(e) => panic!("Failed to parse boolean: {}", e),
        //         };
        //         element::create_boolean(val, self.get_context())
        //     },
        //     DataType::String => {
        //         let val: String = match value.parse::<String>() {
        //             Ok(val) => val,
        //             Err(e) => panic!("Failed to parse string: {}", e),
        //         };
        //         element::create_string(&val, self.get_builder())
        //     },
        //     DataType::Unknown => {
        //         std::ptr::null_mut() // this is intentional
        //     }
        //     _ => unimplemented!("Unimplemented data type in literal ir")
        // }
        std::ptr::null_mut()
    }

    /// TODO
    pub fn generate_var_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        // match data_type {
        //     DataType::Integer => {
        //         // let val: i64 = match value.parse::<i64>() {
        //         //     Ok(val) => val,
        //         //     Err(e) => panic!("Failed to parse integer: {}", e),
        //         // };
        //         // self.add_named_value(name, value)
        //         // create_element::create_integer(val, self.get_context())
        //         std::ptr::null_mut()
        //     }
        //     _ => unimplemented!("unimplemented var ir")
        // }
        std::ptr::null_mut()
    }
}