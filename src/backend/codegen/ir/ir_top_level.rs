use std::sync::{Mutex, Arc};

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::{
            types::void_type, 
            element::{create_function_type, add_function_to_module}, init_ir::create_basic_block, utils::position_builder
        }
    }, 
    frontend::{ast::{
        syntax_element::FunctionParameter, 
        data_type::DataType, ast_struct::ASTNode
    }, 
    symbol_table::symbol_table_struct::SymbolTableStack}, 
};

use llvm::prelude::LLVMValueRef;

impl IRGenerator {
    /// Generates LLVM IR for a function declaration
    pub fn generate_fn_declaration_ir(
            &mut self, 
            name: &String, 
            parameters: &Vec<FunctionParameter>, 
            return_type: &Option<DataType>,
            body: &Vec<ASTNode>,
            symbol_table_stack: &Arc<Mutex<SymbolTableStack>>,
    ) -> LLVMValueRef {
        let llvm_return_type = match return_type {
            Some(data_type) => self.map_data_type(data_type),
            None => void_type(self.get_context()),
        };
    
        let mut llvm_param_types = Vec::new();
        for param in parameters {
            let param_type = self.map_data_type(&param.get_data_type());
            llvm_param_types.push(param_type);
        }
    
        let function_type = create_function_type(llvm_return_type, &llvm_param_types, false);
        let function = add_function_to_module(self.get_module(), name, function_type);
        let entry_bb = create_basic_block(self.get_context(), function, "entry");
        position_builder(self.get_builder(), entry_bb);
    
        self.set_current_function(function);
    
        for node in body.iter() {
            self.ir_router(node, symbol_table_stack);
        }
    
        function
    }
    
    /// TODO
    pub fn generate_enum_declaration_ir(&mut self, _name: &String, _variants: &Vec<String>) -> LLVMValueRef {
        std::ptr::null_mut()

    }

    /// TODO
    pub fn generate_struct_declaration_ir(&mut self, _name: &String, _fields: &Vec<(String, DataType)>) -> LLVMValueRef {
        std::ptr::null_mut()
   
    }
}