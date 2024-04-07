use std::{collections::{HashMap, HashSet}, fmt, sync::{Arc, Mutex}};
use crate::frontend::{ast::{
    ast_struct::ASTNode, data_type::DataType,
}, utils::error::ErrorType};

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
    Node(ASTNode),

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
        parameters: Vec<(String, DataType)>, 
        /// Return type of the function
        return_type: Option<DataType>,
    },
}

/// Information on a symbol in a symboltable
#[derive(Clone, Debug)]
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

    pub fn get_element(&self, index: usize) -> Result<Arc<Mutex<SymbolTable>>, ErrorType> {
        if index < self.elements.len() {
            match self.elements.get(index) {
                Some(arc_mutex_symbol_table) => {
                    return Ok(arc_mutex_symbol_table.clone());
                },
                None => {
                    return Err(ErrorType::DevError{});
                }
            }
        } 
        panic!("Invalid index: {:} for size {:}", index, self.elements.len())
    }
}

impl fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_map();
        for (key, value) in &self.values {
            builder.entry(&key, &value);
        }
        builder.finish()
    }
}

impl fmt::Debug for SymbolTableStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SymbolTableStack")
         .field("size", &self.elements.len())
         .field("tables", &self.elements.iter().map(|table_arc| {
             match table_arc.lock() {
                 Ok(lock) => format!("{:?}", lock), 
                 Err(_) => "PoisonedLock".to_string(), 
             }
         }).collect::<Vec<String>>())
         .finish()
    }
}


impl PartialEq for SymbolTableStack {
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }

        for (self_table, other_table) in self.elements.iter().zip(&other.elements) {
            let self_table_lock = match self_table.lock() {
                Ok(lock) => lock,
                Err(_) => return false, 
            };
            let other_table_lock = match other_table.lock() {
                Ok(lock) => lock,
                Err(_) => return false, 
            };

            let self_keys: HashSet<_> = self_table_lock.values.keys().collect();
            let other_keys: HashSet<_> = other_table_lock.values.keys().collect();

            if self_keys != other_keys {
                return false;
            }
            // TODO make this better
        }

        true
    }
}

impl Eq for SymbolTableStack {}