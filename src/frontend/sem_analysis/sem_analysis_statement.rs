use std::f32::consts::E;

use crate::frontend::{
    ast::{
        ast_struct::ASTNode, syntax_element::SyntaxElement, 
    }, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::symbol_table_struct::SymbolTable, 
    utils::error::ErrorType,
};

impl SemAnalysis {
    /// Completes semantic analysis of variable assignment
    pub fn sem_assignment(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        // let mut errors: Vec<ErrorType> = Vec::new();
        
        // match self.get_current_sym_table() {
        //     Ok(table) => {
        //         let locked_table = table.lock().unwrap();
                    
        //         if let SyntaxElement::Assignment = node.get_element() {
        //             for child in node.get_children() {
        //                 match child.get_element() {
        //                     SyntaxElement::Identifier(name) => {
        //                         if let Some(e) = self.check_if_assigned(&name, &locked_table) {
        //                             errors.push(e);
        //                         }
        //                     }
        //                     SyntaxElement::AssignedValue => {
        //                         if let Some(child_errors) = self.sem_analysis_router(&child) {
        //                             errors.extend(child_errors);
        //                         }
        //                     }
        //                     _ => errors.push(ErrorType::DevError{})
        //                 }
        //             }
        //         } else {
        //             errors.push(ErrorType::DevError{});
        //         }
            

        //     }
        //     Err(e) => {
        //         return Some(vec![e]);
        //     }
        // }
        // if !errors.is_empty() {
        //     return Some(errors);
        // }
        None
    }
    

    /// TODO
    pub fn sem_bin_exp(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_initialization(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_match_statement(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_function_call(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_unary_exp(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_return(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_break(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_continue(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

}