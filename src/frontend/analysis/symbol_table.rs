/*
Stores values and their names, accounting for scope with a stack
 */

use std::collections::HashMap;
use crate::frontend::ast::data_type::DataType;

/// A stack of symbol tables, used to represent different levels of scope
pub struct SymbolTableStack {
    elements: Vec<SymbolTable>,
}

/// Initialized values in a scope
pub struct SymbolTable {
    values: HashMap<String, SymbolInfo>,
}

/// Information on a symbol in a symboltable
pub struct SymbolInfo {
    data_type: DataType,
}

impl SymbolTableStack {
    pub fn new() -> Self {
        SymbolTableStack {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: SymbolTable) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<SymbolTable> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&SymbolTable> {
        self.elements.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, info: SymbolInfo) {
        self.values.insert(name, info);
    }

    pub fn get(&self, name: &str) -> Option<&SymbolInfo> {
        self.values.get(name)
    }
}