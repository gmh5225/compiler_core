use std::sync::{Mutex, Arc};

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::{
            element::{
                add_function_to_module, create_function_type
            }, 
            init_ir::create_basic_block, 
            types::void_type, 
            block::position_builder
        }
    }, 
    frontend::{
        ast::{
            ast_struct::ASTNode, data_type::DataType, syntax_element::FunctionParameter
    }, 
    symbol_table::symbol_table::SymbolTableStack}, 
};

use llvm::prelude::LLVMValueRef;

impl IRGenerator {
    pub fn generate_fn_declaration_ir(
            &mut self, 
            name: &String, 
            parameters: &Vec<FunctionParameter>, 
            return_type: &Option<DataType>,
            body: &Vec<ASTNode>,
            symbol_table_stack: &Arc<Mutex<SymbolTableStack>>,) -> LLVMValueRef {

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
    

    pub fn generate_enum_declaration_ir(&mut self, name: &String, variants: &Vec<String>) -> LLVMValueRef {
        // let enum_type = int_type(self.get_context()); 
        // let mut variant_values = Vec::new();

        // for (index, variant) in variants.iter().enumerate() {
        //     let variant_value = add_constant_to_module(self.get_module(), &enum_type, index as i64, variant);
        //     variant_values.push(variant_value);
        // }
        std::ptr::null_mut()
    }

    pub fn generate_struct_declaration_ir(&mut self, name: &String, fields: &Vec<(String, DataType)>) -> LLVMValueRef {
        std::ptr::null_mut()
   
    }
}