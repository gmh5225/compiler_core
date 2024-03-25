use crate::frontend::{
    ast::{ast_struct::ASTNode, syntax_element::SyntaxElement}, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::symbol_table_struct::SymbolTable, utils::error::ErrorType
};

impl SemAnalysis {
    /// Completes semantic analysis on a function declaration
    pub fn sem_function_dec(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        // let mut errors: Vec<ErrorType> = Vec::new();
        // let mut has_body: bool = false;

        // let sym_table: Option<SymbolTable> = match self.get_current_sym_table() {
        //     Ok(table) => {
        //         let locked_table = table.lock().unwrap();
        //         Some(*locked_table)
        //     }
        //     Err(e) => {
        //         errors.push(e);
        //         None
        //     }
        // };

        // match sym_table {
        //     Some(table) => {
        //         match self.increment_sym_table_stack_pointer() {
        //             Ok(_) => {}
        //             Err(e) => errors.push(e)
        //         }
                
        //         for child in node.get_children() {
        //             match child.get_element() {
        //                 SyntaxElement::Parameter => {},
        
        //                 SyntaxElement::Type(fn_type) => {},
        
        //                 SyntaxElement::FunctionDeclaration => {
        //                 },
        //                 SyntaxElement::BlockExpression => {
        //                     has_body = true;
        //                     self.sem_analysis_router(&child);
        //                 }
        //                 // _ => errors.push(ErrorType::DevError{})
        //             }
        //         }
        //         if !has_body {
        //             errors.push(ErrorType::DevError{})
        //         }
        
        //         if !errors.is_empty() {
        //             return Some(errors);
        //         }
        
        //         match self.decrement_sym_table_stack_pointer() {
        //             Ok(_) => {}
        //             Err(e) => errors.push(e)
        //         }
        //         None
        //     }
        //     None => {
        //         panic!("Missing symbol table")
        //     }
        // }
        None
    }
    
    /// TODO
    pub fn sem_struct_dec(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }

    /// TODO
    pub fn sem_enum_dec(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        None
    }
}