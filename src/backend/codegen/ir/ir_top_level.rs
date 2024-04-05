use std::sync::{Mutex, Arc};

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::{
            block::position_builder, element::{
                add_function_to_module, create_function_type
            }, init_ir::create_basic_block, types::void_type
        }
    }, 
    frontend::{
        ast::{
            ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement,
    }, 
    symbol_table::symbol_table_struct::{SymbolTable, SymbolValue}
}, 
};

use llvm::prelude::LLVMValueRef;
use llvm::{LLVMBasicBlock, LLVMType, LLVMValue};

impl IRGenerator {
    pub fn generate_fn_declaration_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        if let SyntaxElement::FunctionDeclaration = node.get_element() {
            let children: Vec<ASTNode> = node.get_children();
    
            let mut fn_id: Option<String> = None;
            let mut fn_block: Option<&ASTNode> = None;

            for child in children.iter() {
                match child.get_element() {
                    SyntaxElement::Identifier(fn_name) => {
                        fn_id = Some(fn_name.clone());
                    },
                    SyntaxElement::BlockExpression => {
                        fn_block = Some(child);
                    },
                    SyntaxElement::Parameter => {
                        continue
                    },
                    SyntaxElement::Type(_) => {
                        continue
                    },
                    _ => panic!("Unexpected node: {:?}", child)
                }
            }
            let fn_id: String = fn_id.expect("Function name is missing");
    
            let (params, fn_type) = self.extract_function_signature(&fn_id);
    
            let llvm_return_type: *mut LLVMType = fn_type.map_or_else(
                || void_type(self.get_context()), 
                |data_type| self.map_data_type(&data_type),
            );
    
            let llvm_param_types: Vec<*mut LLVMType> = params.iter()
                .map(|(_, param_type)| self.map_data_type(param_type))
                .collect();
    
            let function_type: *mut LLVMType = create_function_type(llvm_return_type, &llvm_param_types, false);
            let function: *mut LLVMValue = add_function_to_module(self.get_module(), &fn_id, function_type);
            let entry_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "entry");

            position_builder(self.get_builder(), entry_bb);
            self.set_current_function(function);
    
            match fn_block {
                Some(block_exp) => {
                    self.ir_router(block_exp);
                }
                _ => panic!("Missing boxy exp")
            }
    
            function
        } else {
            panic!("Unexpected node: {:?}", node);
        }
    }
    
    fn extract_function_signature(&self, fn_name: &String) -> (Vec<(String, DataType)>, Option<DataType>) {
        if let Some(stack) = self.get_stack() {
            let locked_stack = stack.lock().unwrap();
            let table: Arc<Mutex<SymbolTable>> = locked_stack.get_element(self.get_stack_pointer()).expect("Unexpected error");
            let locked_table = table.lock().unwrap();
    
            if let Some(symbol_value) = locked_table.get(fn_name) {
                if let SymbolValue::FunctionValue { parameters, return_type } = symbol_value.get_value() {
                    (parameters.clone(), return_type.clone())
                } else {
                    panic!("Unexpected node: {:?}", symbol_value.get_value());
                }
            } else {
                panic!("Function name missing");
            }
        } else {
            panic!("Missing stack");
        }
    }
    

    pub fn generate_enum_declaration_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        // let enum_type = int_type(self.get_context()); 
        // let mut variant_values = Vec::new();

        // for (index, variant) in variants.iter().enumerate() {
        //     let variant_value = add_constant_to_module(self.get_module(), &enum_type, index as i64, variant);
        //     variant_values.push(variant_value);
        // }
        std::ptr::null_mut()
    }

    /// TODO
    pub fn generate_struct_declaration_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        std::ptr::null_mut()
   
    }
}