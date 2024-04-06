use std::sync::{Arc, Mutex};

use crate::{
    backend::{
        codegen::{ir::ir_codegen_core::IRGenerator, store::Store}, 
        llvm_lib::ir_lib::{
            element::{create_break_statement, create_continue_statement}, ops, return_type::{nonvoid_return, void_return}, 
            var

        }
    }, frontend::{ast::{
        ast_struct::ASTNode, syntax_element::SyntaxElement}, 
        symbol_table::symbol_table_struct::{SymbolTableStack, SymbolValue}} 
};

use llvm::prelude::LLVMValueRef;
use llvm::LLVMValue;
use llvm::LLVMBasicBlock;
use llvm::LLVMBuilder;

impl IRGenerator {
    /// Generates LLVM IR for a binary expression 
    pub fn generate_binary_exp_ir(&mut self, node: &ASTNode)-> LLVMValueRef {
        // let left_val: *mut LLVMValue = self.ir_router(left, symbol_table_stack);
        // let right_val: *mut LLVMValue = self.ir_router(right, symbol_table_stack); 

        // match operator.as_str() {
        //     "+" => {
        //         let tmp_name: CString = CString::new("addtmp").expect("Failed to create CString for add");
        //         ops::build_add(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "-" => {
        //         let tmp_name: CString = CString::new("subtmp").expect("Failed to create CString for minus");
        //         ops::build_sub(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "/" => {
        //         let tmp_name: CString = CString::new("divtmp").expect("Failed to create CString for divide");
        //         ops::build_div(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "*" => {
        //         let tmp_name: CString = CString::new("multmp").expect("Failed to create CString for divide");
        //         ops::build_mul(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "%" => {
        //         let tmp_name: CString = CString::new("remtmp").expect("Failed to create CString for divide");
        //         ops::build_rem(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "&&" => {
        //         let tmp_name: CString = CString::new("andtmp").expect("Failed to create CString for divide");
        //         ops::build_and(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "|" => {
        //         let tmp_name: CString = CString::new("ortmp").expect("Failed to create CString for divide");
        //         ops::build_or(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "^" => {
        //         let tmp_name: CString = CString::new("xortmp").expect("Failed to create CString for divide");
        //         ops::build_xor(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "<<" => {
        //         let tmp_name: CString = CString::new("shltmp").expect("Failed to create CString for shift left");
        //         ops::build_shl(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     ">>" => {
        //         let tmp_name: CString = CString::new("shrtmp").expect("Failed to create CString for shift right");
        //         ops::build_shr(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     ">" => {
        //         let tmp_name: CString = CString::new("gttmp").expect("Failed to create CString for greater than");
        //         ops::build_icmp_gt(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "<" => {
        //         let tmp_name: CString = CString::new("lttmp").expect("Failed to create CString for less than");
        //         ops::build_icmp_lt(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     "==" => {
        //         let tmp_name: CString = CString::new("eqtmp").expect("Failed to create CString for equal");
        //         ops::build_icmp_eq(self.get_builder(), left_val, right_val, tmp_name)
        //     }
        //     _ => panic!("Unrecognized binops operator{:?}", operator.as_str()) 
        // } 
        std::ptr::null_mut()
    }

    /// TODO
    pub fn generate_match_ir(&mut self, node: &ASTNode)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    /// TODO
    pub fn generate_fn_call_ir(&mut self, node: &ASTNode)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    /// TODO
    pub fn generate_initialization_ir(&mut self, node: &ASTNode)-> LLVMValueRef {
        std::ptr::null_mut()

    }

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
    pub fn generate_break_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        create_break_statement(self.get_builder(), self.get_current_block());
        std::ptr::null_mut()
    }

    pub fn generate_continue_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        create_continue_statement(self.get_builder(), self.get_current_block());
        std::ptr::null_mut()
    }

    /// Generates LLVM IR for a unary operation 
    pub fn generate_unary_ir(&mut self, node: &ASTNode)-> LLVMValueRef {
        // let operand_ir: *mut LLVMValue = self.ir_router(operand, symbol_table_stack);
        // match operator.as_str() {
        //     "-" => {
        //         let tmp_name = CString::new("negtmp").expect("Failed to create CString for negation");
        //         ops::build_negation(self.get_builder(), operand_ir, tmp_name)
        //     },
        //     "~" => {
        //         let tmp_name = CString::new("nottmp").expect("Failed to create CString for bitwise not");
        //         ops::generate_bitwise_not(self.get_builder(), operand_ir, tmp_name)
        //     },
        //     "!" => {
        //         let tmp_name = CString::new("lognotmp").expect("Failed to create CString for logical not");
        //         ops::generate_logical_not(self.get_builder(), self.get_context(), operand_ir, tmp_name)
        //     },
        //     _ => panic!("Unknown unary operator: {}", operator),
        // }
        std::ptr::null_mut()
    }

    pub fn generate_return_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        let children: Vec<ASTNode> = node.get_children();
    
        let value: Option<&ASTNode> = children.iter().find(|child| matches!(child.get_element(), SyntaxElement::AssignedValue));
    
        match value {
            Some(value_node) => {
                let llvm_val = self.ir_router(value_node);
                nonvoid_return(self.get_builder(), llvm_val)
            },
            None => {
                void_return(self.get_builder())
            }
        }
    }
    
}