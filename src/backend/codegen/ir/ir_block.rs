use crate::{
    frontend::ast::ast_struct::ASTNode,
    backend::codegen::ir::ir_codegen_core::IRGenerator, 
};

use llvm::prelude::LLVMValueRef;

impl IRGenerator {
    pub fn generate_do_while_ir(&mut self, body: &Box<Vec<ASTNode>>, condition: &Box<ASTNode>)-> LLVMValueRef {
        std::ptr::null_mut()
    }

    pub fn generate_while_ir(&mut self, body: &Box<ASTNode>, condition: &Box<Vec<ASTNode>>)-> LLVMValueRef {
        std::ptr::null_mut()

    }

    pub fn generate_for_ir(&mut self, initializer: &Option<Box<ASTNode>>, condition: &Box<ASTNode>, increment: &Option<Box<ASTNode>>, body: &Box<Vec<ASTNode>>) -> LLVMValueRef{
        std::ptr::null_mut()

    }

    pub fn generate_if_ir(&mut self, condition: &Box<ASTNode>, then_branch: &Box<Vec<ASTNode>>, else_branch: &Option<Box<Vec<ASTNode>>>) -> LLVMValueRef{
        std::ptr::null_mut()
        
    }

}