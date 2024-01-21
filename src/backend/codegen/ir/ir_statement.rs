use crate::{
    frontend::ast::{
        ast_struct::ASTNode, 
        syntax_element::MatchArm, 
        data_type::DataType,
    },
    backend::codegen::ir::ir_codegen_core::IRGenerator, 
};

use llvm::prelude::LLVMValueRef;

impl IRGenerator {
    pub fn generate_binary_exp_ir(&mut self, left: &Box<ASTNode>, operator: &String, right: &Box<ASTNode>)-> LLVMValueRef {
        std::ptr::null_mut()

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

    pub fn generate_assignment_ir(&mut self, variable: &String, value: &Box<ASTNode>)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_break_ir(&mut self)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_continue_ir(&mut self)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_unary_ir(&mut self)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_return_ir(&mut self, value: &Box<ASTNode>)-> LLVMValueRef {
        std::ptr::null_mut()

    }

}