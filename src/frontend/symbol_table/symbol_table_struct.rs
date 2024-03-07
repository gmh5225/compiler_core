use std::{collections::HashMap, sync::{Arc, Mutex}};
use crate::frontend::ast::{
    ast_struct::ASTNode, data_type::DataType, syntax_element::FunctionParameter
};

/// Initialized values in a scope
#[derive(Clone)]
pub struct SymbolTable {
    values: HashMap<String, SymbolInfo>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolValue {
    StrValue(Box<str>),
    Node(Box<ASTNode>), // this means the symbolvalue is calculated
    EnumValue { variants: Vec<String> },
    StructValue { fields: Vec<(String, DataType)> },
    FunctionValue { parameters: Vec<FunctionParameter>, return_type: Option<DataType> },
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

/// A stack of symbol tables, used to represent different levels of scope
#[derive(Clone)]
pub struct SymbolTableStack {
    elements: Vec<Arc<Mutex<SymbolTable>>>,
}

impl SymbolTableStack {
    pub fn new() -> Self {
        SymbolTableStack {
            elements: Vec::new(),
        }
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