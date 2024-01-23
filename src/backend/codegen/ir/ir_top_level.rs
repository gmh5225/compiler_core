use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::{
            types::void_type, 
            element::{create_function_type, add_function_to_module}
        }
    }, 
    frontend::ast::{
        syntax_element::FunctionParameter, 
        data_type::DataType
    }, 
};

use llvm::prelude::LLVMValueRef;
use llvm::LLVMType;

impl IRGenerator {
    pub fn generate_fn_declaration_ir(&mut self, name: &String, parameters: &Vec<FunctionParameter>, return_type: &Option<DataType>) -> LLVMValueRef {
        let llvm_return_type = match return_type {
            Some(data_type) => self.map_data_type(data_type),
            None => void_type(self.get_context()),
        };
    
        let mut llvm_param_types = Vec::new();
        for param in parameters {
            let param_type = self.map_data_type(&param.get_data_type());
            llvm_param_types.push(param_type);
        }
    
        let function_type: *mut LLVMType = create_function_type(self.get_context(), llvm_return_type, &llvm_param_types, false);
    
        let function: LLVMValueRef = add_function_to_module(self.get_module(), name, function_type);
    
        self.set_current_function(function);
        
        function
    }
    

    pub fn generate_enum_declaration_ir(&mut self, name: &String, variants: &Vec<String>) -> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_struct_declaration_ir(&mut self, name: &String, fields: &Vec<(String, DataType)>) -> LLVMValueRef {
        std::ptr::null_mut()
   
    }
}