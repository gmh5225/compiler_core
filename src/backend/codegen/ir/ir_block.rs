use std::sync::{Mutex, Arc};

use crate::{
    frontend::{
        ast::ast_struct::ASTNode, 
        analysis::symbol_table::SymbolTableStack
    },
    backend::{codegen::ir::ir_codegen_core::IRGenerator, llvm_lib::ir_lib::{init_ir::create_basic_block, utils::position_builder, element::{create_cond_br, create_br}}}, 
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
        let else_bb = if else_branch.is_some() {
            Some(create_basic_block(self.get_context(), function, "else"))
        } else {
            None
        };
    
        let then_returns = then_branch.last().map_or(false, |node| node.is_return());
        let else_returns = else_branch.as_ref().map_or(true, |branch| {
            branch.last().map_or(false, |node| node.is_return())
        });
    
        let merge_bb_needed = !then_returns || !else_returns;
        let merge_bb = if merge_bb_needed {
            Some(create_basic_block(self.get_context(), function, "merge"))
        } else {
            None
        };
    
        create_cond_br(self.get_builder(), condition_val, then_bb, else_bb.unwrap_or_else(|| merge_bb.unwrap_or(then_bb)));
    
        position_builder(self.get_builder(), then_bb);
        for node in then_branch.iter() {
            self.ir_router(node, symbol_table_stack);
        }
        if merge_bb_needed && !then_returns {
            create_br(self.get_builder(), merge_bb.unwrap());
        }
    
        if let Some(else_nodes) = else_branch {
            position_builder(self.get_builder(), else_bb.unwrap());
            for node in else_nodes.iter() {
                self.ir_router(node, symbol_table_stack);
            }
            if merge_bb_needed && !else_returns {
                create_br(self.get_builder(), merge_bb.unwrap());
            }
        }
    
        if let Some(merge) = merge_bb {
            position_builder(self.get_builder(), merge);
        }
    
        std::ptr::null_mut()
    }
    

} 