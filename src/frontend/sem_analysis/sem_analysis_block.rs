use std::sync::{Arc, Mutex};

use crate::frontend::{
    ast::ast_struct::ASTNode, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::symbol_table_struct::SymbolTableStack, 
    utils::error::ErrorType
};

impl<'a> SemAnalysis {
    /// TODO
    pub fn sem_for_loop(&mut self, 
        _initializer: &Option<Box<ASTNode>>, 
        _condition: &Box<ASTNode>, 
        _increment: &Option<Box<ASTNode>>, 
        _body: &Box<Vec<ASTNode>>, 
        _symbol_table: &Arc<Mutex<SymbolTableStack>>,) 
    -> Option<Vec<ErrorType>> {

        unimplemented!("Sem analysis for loop unimplemented")
    }

    /// TODO
    pub fn sem_while_loop(&mut self, _condition: &Box<ASTNode>, _body: &Box<Vec<ASTNode>>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis while loop unimplemented")
    }

    /// TODO
    pub fn sem_do_while_loop(&mut self, _body: &Box<Vec<ASTNode>>, _condition: &Box<ASTNode>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis do while loop unimplemented")
    }

    /// TODO
    pub fn sem_if_statement(&mut self, 
        _condition: &Box<ASTNode>, 
        _then_branch: &Box<Vec<ASTNode>>, 
        _else_branch: &Option<Box<Vec<ASTNode>>>, 
            _symbol_table: &Arc<Mutex<SymbolTableStack>>,) 
    -> Option<Vec<ErrorType>> {
        None
    }
}

