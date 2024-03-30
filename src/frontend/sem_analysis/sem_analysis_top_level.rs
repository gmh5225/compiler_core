use crate::frontend::{
    ast::{ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement}, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::symbol_table_struct::SymbolTable, utils::error::ErrorType
};

impl SemAnalysis {
    /// Completes semantic analysis on a function declaration
    pub fn sem_function_dec(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        // let mut errors: Vec<ErrorType> = Vec::new();

        // let mut has_body: bool = false;

        // match self.get_current_sym_table() {
        //     Ok(table) => {
        //         let locked_table = table.lock().unwrap();

        //         for child in node.get_children() {
        //             match child.get_element() {
        //                 SyntaxElement::Parameter => {},
        //                 SyntaxElement::Type(_fn_type) => {},
        //                 SyntaxElement::FunctionDeclaration => {},

        //                 SyntaxElement::BlockExpression => {
        //                     has_body = true;

        //                     match self.increment_sym_table_stack_pointer() {
        //                         Ok(_) => {}
        //                         Err(e) => errors.push(e)
        //                     }

        //                     self.sem_analysis_router(&child);
        //                 }
        //                 _ => panic!("Unexpected node: {:?}", child)
        //             }
        //         }
        //         if !has_body {
        //             errors.push(ErrorType::DevError{})
        //         }
        
        //         if !errors.is_empty() {
        //             return Some(errors);
        //         }
        //     }
        //     Err(e) => {
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