extern crate llvm_sys as llvm;

use std::sync::{Mutex, Arc};

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::{
            block, element, init_ir, types
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

impl<T> IRGenerator<T> {
    /// Generates LLVM IR for a function declaration
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
                || types::void_type(self.get_context()), 
                |data_type| self.map_data_type(&data_type),
            );
    
            let llvm_param_types: Vec<*mut LLVMType> = params.iter()
                .map(|(_, param_type)| self.map_data_type(param_type))
                .collect();
    
            let function_type: *mut LLVMType = element::create_function_type(llvm_return_type, &llvm_param_types, false);
            let function: *mut LLVMValue = element::add_function_to_module(self.get_module(), &fn_id, function_type);
            let entry_bb: *mut LLVMBasicBlock = init_ir::create_basic_block(self.get_context(), function, "entry");

            block::position_builder(self.get_builder(), entry_bb);
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
}