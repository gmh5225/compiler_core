use std::sync::{Mutex, Arc};

use crate::{
    frontend::{
        ast::ast_struct::ASTNode, 
        symbol_table::symbol_table_struct::SymbolTableStack
    },
    backend::{codegen::ir::ir_codegen_core::IRGenerator, llvm_lib::ir_lib::{init_ir::create_basic_block, utils::position_builder, element::{create_cond_br, create_br}}}, 
};

use llvm::prelude::LLVMValueRef;

impl IRGenerator {
    pub fn generate_do_while_ir(&mut self, body: &Box<Vec<ASTNode>>, condition: &Box<ASTNode>, symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) -> LLVMValueRef {
        let function = self.get_current_function();
        let do_body_bb = create_basic_block(self.get_context(), function, "do_body");
        let do_cond_bb = create_basic_block(self.get_context(), function, "do_cond");
        let do_end_bb = create_basic_block(self.get_context(), function, "do_end");
    
        let entry_bb = self.get_current_block();
        position_builder(self.get_builder(), entry_bb);
        create_br(self.get_builder(), do_body_bb);
    
        position_builder(self.get_builder(), do_body_bb);
        for node in body.iter() {
            self.ir_router(node, symbol_table_stack);
        }
        create_br(self.get_builder(), do_cond_bb);
    
        position_builder(self.get_builder(), do_cond_bb);
        let condition_val = self.ir_router(condition, symbol_table_stack);
        create_cond_br(self.get_builder(), condition_val, do_body_bb, do_end_bb);
    
        position_builder(self.get_builder(), do_end_bb);
    
        std::ptr::null_mut()
    }
    
    
    pub fn generate_while_ir(&mut self, condition: &Box<ASTNode>, body: &Box<Vec<ASTNode>>, symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) -> LLVMValueRef {
        let function = self.get_current_function();
        let while_cond_bb = create_basic_block(self.get_context(), function, "while_cond");
        let while_body_bb = create_basic_block(self.get_context(), function, "while_body");
        let while_end_bb = create_basic_block(self.get_context(), function, "while_end");
    
        let entry_bb = self.get_current_block();
        position_builder(self.get_builder(), entry_bb);
        create_br(self.get_builder(), while_cond_bb);
    
        position_builder(self.get_builder(), while_cond_bb);
        let condition_val = self.ir_router(condition, symbol_table_stack);
        create_cond_br(self.get_builder(), condition_val, while_body_bb, while_end_bb);
    
        let body_returns = body.last().map_or(false, |node| node.is_return());
    
        position_builder(self.get_builder(), while_body_bb);
        for node in body.iter() {
            self.ir_router(node, symbol_table_stack);
        }
        if !body_returns {
            create_br(self.get_builder(), while_cond_bb);
        }
    
        position_builder(self.get_builder(), while_end_bb);
    
        std::ptr::null_mut()
    }
    
    pub fn generate_for_ir(&mut self, initializer: &Option<Box<ASTNode>>, condition: &Box<ASTNode>, increment: &Option<Box<ASTNode>>, body: &Box<Vec<ASTNode>>, symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) -> LLVMValueRef {
        let function = self.get_current_function();
        let for_cond_bb = create_basic_block(self.get_context(), function, "for_cond");
        let for_body_bb = create_basic_block(self.get_context(), function, "for_body");
        let for_inc_bb = increment.as_ref().map(|_| create_basic_block(self.get_context(), function, "for_inc"));
        let for_end_bb = create_basic_block(self.get_context(), function, "for_end");

        let entry_bb = self.get_current_block();
        position_builder(self.get_builder(), entry_bb);

        if let Some(init_node) = initializer {
            self.ir_router(init_node, symbol_table_stack);
        }
        create_br(self.get_builder(), for_cond_bb);

        position_builder(self.get_builder(), for_cond_bb);
        let condition_val = self.ir_router(condition, symbol_table_stack);
        create_cond_br(self.get_builder(), condition_val, for_body_bb, for_end_bb);

        position_builder(self.get_builder(), for_body_bb);
        let body_returns = body.last().map_or(false, |node| node.is_return());
        for node in body.iter() {
            self.ir_router(node, symbol_table_stack);
        }

        if let Some(inc_bb) = for_inc_bb {
            if !body_returns {
                create_br(self.get_builder(), inc_bb);
            }
            position_builder(self.get_builder(), inc_bb);
            if let Some(inc_node) = increment {
                self.ir_router(inc_node, symbol_table_stack);
            }
            create_br(self.get_builder(), for_cond_bb);
        } else if !body_returns {
            create_br(self.get_builder(), for_cond_bb);
        }

        position_builder(self.get_builder(), for_end_bb);

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