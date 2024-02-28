use std::sync::{Arc, Mutex};

use crate::frontend::{
    ast::ast_struct::{ModElement, AST}, 
    symbol_table::symbol_table_struct::SymbolTable, 
    utils::error::ErrorType,
};

/// A stack of symbol tables, used to represent different levels of scope
#[derive(Clone)]
pub struct SymbolTableStack {
    elements: Vec<Arc<Mutex<SymbolTable>>>,
}

impl SymbolTableStack {
    fn new() -> Self {
        SymbolTableStack {
            elements: Vec::new(),
        }
    }

    pub fn gen_sym_table_stack(ast: AST) -> Result<ModElement, Vec<ErrorType>> {
        
    }

    fn sym_table_router() {

    }

    pub fn push(&mut self, item: SymbolTable) {
        let wrapped_table = Arc::new(Mutex::new(item));
        self.elements.push(wrapped_table);
    }

    pub fn pop(&mut self) -> Option<Arc<Mutex<SymbolTable>>> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<Arc<Mutex<SymbolTable>>> {
        self.elements.last().cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn get_elements(&self) -> &Vec<Arc<Mutex<SymbolTable>>> {
        &self.elements
    }
}