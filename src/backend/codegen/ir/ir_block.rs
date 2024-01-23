use std::sync::{Mutex, Arc};

use crate::{
    frontend::{
        ast::ast_struct::ASTNode, 
        analysis::symbol_table::SymbolTableStack
    },
    backend::{codegen::ir::ir_codegen_core::IRGenerator, llvm_lib::ir_lib::{init_ir::create_basic_block, utils::position_builder_at_end, element::{create_cond_br, create_br}}}, 
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

    pub fn generate_if_ir(&mut self, condition: &Box<ASTNode>, then_branch: &Box<Vec<ASTNode>>, else_branch: &Option<Box<Vec<ASTNode>>>, symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) -> LLVMValueRef {
        let condition_val = self.ir_router(condition, symbol_table_stack);

        let function = self.get_current_function();
        let then_bb = create_basic_block(self.get_context(), function, "then");
        let merge_bb = create_basic_block(self.get_context(), function, "merge");
        let else_bb = match else_branch {
            Some(_) => create_basic_block(self.get_context(), function, "else"),
            None => merge_bb, 
        };

        create_cond_br(self.get_builder(), condition_val, then_bb, if else_branch.is_some() { else_bb } else { merge_bb });

        position_builder_at_end(self.get_builder(), then_bb);
        for node in then_branch.iter() {
            self.ir_router(node, symbol_table_stack);
        }
        create_br(self.get_builder(), merge_bb); 

        if let Some(else_nodes) = else_branch {
            position_builder_at_end(self.get_builder(), else_bb);
            for node in else_nodes.iter() {
                self.ir_router(node, symbol_table_stack);
            }
            create_br(self.get_builder(), merge_bb); 
        }

        position_builder_at_end(self.get_builder(), merge_bb);

        std::ptr::null_mut()
    }

} 