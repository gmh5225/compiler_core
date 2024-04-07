use crate::frontend::{
    ast::ast_struct::ASTNode, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    utils::error::ErrorType
};

impl SemAnalysis {
    /// TODO
    pub fn sem_for_loop(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_while_loop(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_do_while_loop(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_if_statement(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }
}

