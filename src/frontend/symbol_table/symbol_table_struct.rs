use std::{collections::HashMap, sync::{Arc, Mutex}};
use crate::frontend::ast::{
    ast_struct::ASTNode, data_type::DataType, syntax_element::FunctionParameter
};

/// Initialized values in a scope
#[derive(Clone)]
pub struct SymbolTable {
    values: HashMap<String, SymbolInfo>,
}

/// Types of symbol values in a symbol table
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolValue {
    /// Simple values that can be represented as a string
    StrValue(Box<str>),

    /// A calculated symbol value
    Node(Box<ASTNode>),

    /// An enum's value (variants)
    EnumValue { 
        /// Variants of the enum
        variants: Vec<String>,
    },

    /// A struct's value (fields)
    StructValue { 
        /// Fields of the struct
        fields: Vec<(String, DataType)>,
    },
    
    /// A function's value (params, return type)
    FunctionValue { 
        /// Parameters of the function
        parameters: Vec<FunctionParameter>, 
        /// Return type of the function
        return_type: Option<DataType>,
    },
}

/// Information on a symbol in a symboltable
#[derive(Clone)]
pub struct SymbolInfo {
    data_type: DataType,
    value: SymbolValue,
}

impl SymbolInfo {
    /// Creates a new symbol in a symbol table
    pub fn new(data_type: DataType, value: SymbolValue) -> Self {
        Self {
            data_type,
            value
        }
    }

    /// Retrieves the symbol info's value
    pub fn get_value(&self) -> SymbolValue {
        self.value.clone()
    }

    /// Retrieves the symbol info's data type
    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
}

impl SymbolTable {
    /// Creates a new symbol table
    pub fn new() -> Self {
        SymbolTable {
            values: HashMap::new(),
        }
    }

    /// Adds a new symbol info to the table
    pub fn add(&mut self, name: String, info: SymbolInfo) {
        self.values.insert(name, info);
    }

    /// Retrieves a value from the symbol table, else None
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
    /// Creates a new symbol table stack
    pub fn new() -> Self {
        SymbolTableStack {
            elements: Vec::new(),
        }
    }

    /// Pushes a new table onto the stack
    pub fn push(&mut self, item: SymbolTable) {
        let wrapped_table = Arc::new(Mutex::new(item));
        self.elements.push(wrapped_table);
    }

    /// Pops the topmost table off the stack
    pub fn pop(&mut self) -> Option<Arc<Mutex<SymbolTable>>> {
        self.elements.pop()
    }

    /// Retrieves the topmost table off the stack
    pub fn peek(&self) -> Option<Arc<Mutex<SymbolTable>>> {
        self.elements.last().cloned()
    }

    /// Checks if there are no tables on the stack
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Retrieves the size of the stack
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// Retreives all the tables off the stack
    pub fn get_elements(&self) -> &Vec<Arc<Mutex<SymbolTable>>> {
        &self.elements
    }
}