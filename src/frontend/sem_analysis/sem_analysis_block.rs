use std::sync::{Arc, Mutex};

use crate::frontend::{
    ast::ast_struct::ASTNode, sem_analysis::sem_analysis_core::SemAnalysis, symbol_table::symbol_table::SymbolTableStack, utils::error::ErrorType
};

impl<'a> SemAnalysis {
    pub fn sem_for_loop(&mut self, initializer: &Option<Box<ASTNode>>, condition: &Box<ASTNode>, increment: &Option<Box<ASTNode>>, body: &Box<Vec<ASTNode>>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis for loop unimplemented")
    }

    pub fn sem_while_loop(&mut self, condition: &Box<ASTNode>, body: &Box<Vec<ASTNode>>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis while loop unimplemented")
    }

    pub fn sem_do_while_loop(&mut self, body: &Box<Vec<ASTNode>>, condition: &Box<ASTNode>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis do while loop unimplemented")
    }

    pub fn sem_if_statement(&mut self, condition: &Box<ASTNode>, then_branch: &Box<Vec<ASTNode>>, else_branch: &Option<Box<Vec<ASTNode>>>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        None
    }
}

