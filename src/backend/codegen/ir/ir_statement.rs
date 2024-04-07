extern crate llvm_sys as llvm;

use std::sync::{Arc, Mutex};

use crate::{
    backend::{
        codegen::{ir::ir_codegen_core::IRGenerator, store::Store}, 
        llvm_lib::ir_lib::{
            element, 
            return_type,
            var,
        }
    }, 
    frontend::ast::{
        ast_struct::ASTNode, syntax_element::SyntaxElement
    }, 
    
};

use llvm::{
    prelude::LLVMValueRef,
    LLVMValue, LLVMBuilder
};

impl IRGenerator {
    /// Generates LLVM IR for an assignment
    pub fn generate_assignment_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        if let SyntaxElement::Assignment = node.get_element() {
            let children: Vec<ASTNode> = node.get_children();
    
            let mut var_node: Option<&ASTNode> = None;
            let mut val_node: Option<&ASTNode> = None;
    
            for child in children.iter() {
                match child.get_element() {
                    SyntaxElement::Variable => {
                        var_node = Some(child);
                    },
                    SyntaxElement::AssignedValue => {
                        val_node = Some(child);
                    },
                    _ => panic!("Unexpected node: {:?}", child)
                }
            }
    
            let var_node: &ASTNode = var_node.expect("Assignment missing variable.");
            let val_node: &ASTNode = val_node.expect("Assignment missing value.");
    
            let mut var_id: Option<String> = None;
    
            for child in var_node.get_children() {
                match child.get_element() {
                    SyntaxElement::Identifier(name) => {
                        var_id = Some(name.clone());
                    }
                    _ => panic!("Unexpected node: {:?}", child)
                }
            }
    
            let var_id: String = var_id.expect("Variable name missing.");
    
            let new_value_ir: *mut LLVMValue = self.ir_router(val_node);
    
            let builder: *mut LLVMBuilder = self.get_builder();
    
            let store: &Arc<Mutex<Store>> = self.get_store();
            let store_locked = store.lock().unwrap();
    
            let variable_alloc: Option<&*mut LLVMValue> = Some(store_locked.get_allocation(&var_id).expect("Missing var alloc"));
    
            var::reassign_var(builder, *variable_alloc.unwrap(), new_value_ir);
    
            new_value_ir
        } else {
            panic!("Expected Assignment node, got: {:?}", node.get_element());
        }
    }
    

    /// Generates LLVM IR for a break statement
    pub fn generate_break_ir(&mut self, _node: &ASTNode) -> LLVMValueRef {
        element::create_break_statement(self.get_builder(), self.get_current_block());
        std::ptr::null_mut()
    }

    /// Generates LLVM IR for a continue statement
    pub fn generate_continue_ir(&mut self, _node: &ASTNode) -> LLVMValueRef {
        element::create_continue_statement(self.get_builder(), self.get_current_block());
        std::ptr::null_mut()
    }

    /// Generates LLVM IR for a return statement
    pub fn generate_return_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        let children: Vec<ASTNode> = node.get_children();
    
        let value: Option<&ASTNode> = children.iter().find(|child| matches!(child.get_element(), SyntaxElement::AssignedValue));
    
        match value {
            Some(value_node) => {
                let llvm_val = self.ir_router(value_node);
                return_type::nonvoid_return(self.get_builder(), llvm_val)
            },
            None => {
                return_type::void_return(self.get_builder())
            }
        }
    }
    
}