use std::collections::HashMap;
use crate::frontend::ast::{
    data_type::DataType, 
    ast_struct::ASTNode
};

/// Initialized values in a scope
#[derive(Clone)]
pub struct SymbolTable {
    values: HashMap<String, SymbolInfo>,
}

#[derive(Clone)]
pub enum SymbolValue {
    StrValue(Box<str>),
    Node(Box<ASTNode>),
}

/// Information on a symbol in a symboltable
#[derive(Clone)]
pub struct SymbolInfo {
    data_type: DataType,
    value: SymbolValue,
}

impl SymbolInfo {
    pub fn new(data_type: DataType, value: SymbolValue) -> Self {
        Self {
            data_type,
            value
        }
    }

    pub fn get_value(&self) -> SymbolValue {
        self.value.clone()
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            values: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, info: SymbolInfo) {
        self.values.insert(name, info);
    }

    pub fn get(&self, name: &str) -> Option<&SymbolInfo> {
        self.values.get(name)
    }
}