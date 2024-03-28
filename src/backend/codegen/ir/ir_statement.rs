use std::{ffi::CString, sync::{Arc, Mutex}};

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::{
            element::{create_break_statement, create_continue_statement}, ops, return_type::nonvoid_return, 
            var

        }
    }, frontend::{ast::{
        ast_struct::ASTNode, data_type::DataType, syntax_element::{MatchArm, SyntaxElement}
    }, 
    symbol_table::symbol_table::{SymbolTableStack, SymbolValue}} 
};

use llvm::prelude::LLVMValueRef;
use llvm::LLVMValue;
use llvm::LLVMBasicBlock;

impl IRGenerator {
    pub fn generate_binary_exp_ir(&mut self, left: &Box<ASTNode>, operator: &String, right: &Box<ASTNode>, 
            symbol_table_stack: &Arc<Mutex<SymbolTableStack>>)-> LLVMValueRef {
        let left_val: *mut LLVMValue = self.ir_router(left, symbol_table_stack);
        let right_val: *mut LLVMValue = self.ir_router(right, symbol_table_stack); 

        match operator.as_str() {
            "+" => {
                let tmp_name: CString = CString::new("addtmp").expect("Failed to create CString for add");
                ops::build_add(self.get_builder(), left_val, right_val, tmp_name)
            }
            "-" => {
                let tmp_name: CString = CString::new("subtmp").expect("Failed to create CString for minus");
                ops::build_sub(self.get_builder(), left_val, right_val, tmp_name)
            }
            "/" => {
                let tmp_name: CString = CString::new("divtmp").expect("Failed to create CString for divide");
                ops::build_div(self.get_builder(), left_val, right_val, tmp_name)
            }
            "*" => {
                let tmp_name: CString = CString::new("multmp").expect("Failed to create CString for divide");
                ops::build_mul(self.get_builder(), left_val, right_val, tmp_name)
            }
            "%" => {
                let tmp_name: CString = CString::new("remtmp").expect("Failed to create CString for divide");
                ops::build_rem(self.get_builder(), left_val, right_val, tmp_name)
            }
            "&&" => {
                let tmp_name: CString = CString::new("andtmp").expect("Failed to create CString for divide");
                ops::build_and(self.get_builder(), left_val, right_val, tmp_name)
            }
            "|" => {
                let tmp_name: CString = CString::new("ortmp").expect("Failed to create CString for divide");
                ops::build_or(self.get_builder(), left_val, right_val, tmp_name)
            }
            "^" => {
                let tmp_name: CString = CString::new("xortmp").expect("Failed to create CString for divide");
                ops::build_xor(self.get_builder(), left_val, right_val, tmp_name)
            }
            "<<" => {
                let tmp_name: CString = CString::new("shltmp").expect("Failed to create CString for shift left");
                ops::build_shl(self.get_builder(), left_val, right_val, tmp_name)
            }
            ">>" => {
                let tmp_name: CString = CString::new("shrtmp").expect("Failed to create CString for shift right");
                ops::build_shr(self.get_builder(), left_val, right_val, tmp_name)
            }
            ">" => {
                let tmp_name: CString = CString::new("gttmp").expect("Failed to create CString for greater than");
                ops::build_icmp_gt(self.get_builder(), left_val, right_val, tmp_name)
            }
            "<" => {
                let tmp_name: CString = CString::new("lttmp").expect("Failed to create CString for less than");
                ops::build_icmp_lt(self.get_builder(), left_val, right_val, tmp_name)
            }
            "==" => {
                let tmp_name: CString = CString::new("eqtmp").expect("Failed to create CString for equal");
                ops::build_icmp_eq(self.get_builder(), left_val, right_val, tmp_name)
            }
            _ => panic!("Unrecognized binops operator{:?}", operator.as_str()) 
        } 
    }

    pub fn generate_match_ir(&mut self, to_match: &Box<ASTNode>, arms: &Vec<MatchArm>)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_fn_call_ir(&mut self, name: &String, arguments: &Vec<ASTNode>)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_initialization_ir(&mut self, variable: &String, data_type: &DataType, value: &Box<ASTNode>)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_assignment_ir(&mut self, variable: &String, value: &Box<ASTNode>, symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) -> LLVMValueRef {
        let new_value_ir: *mut LLVMValue = self.ir_router(value, symbol_table_stack);

        let store = self.get_store();
        let store_locked = store.lock().unwrap();
        let variable_alloc: Option<&*mut LLVMValue> = store_locked.get_allocation(variable);
        
        match variable_alloc {
            Some(var) => {
                var::reassign_var(self.get_builder(), var.clone(), new_value_ir);
            }
            None => {
                panic!("Missing var alloc")
            }
        }

        new_value_ir
    }

    pub fn generate_break_ir(&mut self, break_block: *mut LLVMBasicBlock ) {
        create_break_statement(self.get_builder(), break_block)
    }

    pub fn generate_continue_ir(&mut self, continue_block: *mut LLVMBasicBlock) {
        create_continue_statement(self.get_builder(), continue_block)
    }

    pub fn generate_unary_ir(&mut self, operator: &String, operand: &Box<ASTNode>, symbol_table_stack: &Arc<Mutex<SymbolTableStack>>)-> LLVMValueRef {
        let operand_ir: *mut LLVMValue = self.ir_router(operand, symbol_table_stack);
        match operator.as_str() {
            "-" => {
                let tmp_name = CString::new("negtmp").expect("Failed to create CString for negation");
                ops::build_negation(self.get_builder(), operand_ir, tmp_name)
            },
            "~" => {
                let tmp_name = CString::new("nottmp").expect("Failed to create CString for bitwise not");
                ops::generate_bitwise_not(self.get_builder(), operand_ir, tmp_name)
            },
            "!" => {
                let tmp_name = CString::new("lognotmp").expect("Failed to create CString for logical not");
                ops::generate_logical_not(self.get_builder(), self.get_context(), operand_ir, tmp_name)
            },
            _ => panic!("Unknown unary operator: {}", operator),
        }
    }

    pub fn generate_return_ir(&mut self, value: &Box<ASTNode>, symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) -> LLVMValueRef {
        if let Some(symbol_table_arc) = symbol_table_stack.lock().unwrap().peek() {
            let symbol_table = symbol_table_arc.lock().unwrap();
            std::ptr::null_mut()
            // match value.get_element() {
            //     SyntaxElement::Variable => {
            //         match symbol_table.get(&name) {
            //             Some(symbol_info) => {
            //                 let llvm_val: LLVMValueRef = match &symbol_info.get_value() {
            //                     SymbolValue::StrValue(_) => {
            //                         unimplemented!("Need to add strvalues to implementation")
            //                     },
            //                     SymbolValue::Node(node_val) => {
            //                         self.ir_router(node_val, symbol_table_stack)
            //                     },
            //                 };
    
            //                 nonvoid_return(self.get_builder(), llvm_val)
            //             },
            //             None => panic!("Variable not found in symbol table: {}", name),
            //         }
            //     },
            //     _ => {
            //         let val: *mut LLVMValue = self.ir_router( value, symbol_table_stack);
            //         nonvoid_return(self.get_builder(), val)
            //     }
            // }
        } else {
            panic!("No symbol table found in the stack");
        }
    }
}