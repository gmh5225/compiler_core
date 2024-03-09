use std::sync::MutexGuard;

use crate::
    frontend::{
        ast::{data_type::DataType, syntax_element::FunctionParameter}, 
        utils::error::ErrorType,
        symbol_table::symbol_table_struct::SymbolTableStack,
};

use super::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolValue};

impl SymbolTableStack {
    /// Adds a function type to the current scope
    pub fn sym_table_fn(&mut self, name: String, parameters: Vec<FunctionParameter>, return_type: Option<DataType>) -> Result<(), Vec<ErrorType>> {
        let current_table = match self.peek() { 
            Some(table) => table, 
            None => panic!("No symbol table on the stack."), 
        };

        let mut current_table_lock: MutexGuard<'_, SymbolTable> = current_table.lock().expect("Failed to lock symbol table mutex.");

        let fn_info = SymbolInfo::new(
            DataType::Function,
            SymbolValue::FunctionValue { parameters, return_type}
        );

        current_table_lock.add(name, fn_info);

        let fn_scope: SymbolTable = SymbolTable::new();
        self.push(fn_scope);

        Ok(())
    }

    /// Adds an enum type to the current scope
    pub fn sym_table_enum(&mut self, name: String, variants: Vec<String>) -> Result<(), Vec<ErrorType>> {  
        let current_table = match self.peek() {
            Some(table) => table,
            None => panic!("No symbol table on the stack."),
        };

        let mut current_table_lock: MutexGuard<'_, SymbolTable> = current_table.lock().expect("Failed to lock symbol table mutex");

        let enum_info = SymbolInfo::new(
            DataType::Enum,
            SymbolValue::EnumValue { variants }
        );

        current_table_lock.add(name, enum_info);

        Ok(())
    }

    /// Adds a struct type to the current scope
    pub fn sym_table_struct(&mut self, name: String, fields: Vec<(String, DataType)>) -> Result<(), Vec<ErrorType>> {
        let current_table = match self.peek() {
            Some(table) => table,
            None => panic!("No symbol table on the stack."),
        };
        
        let mut current_table_lock: MutexGuard<'_, SymbolTable> = current_table.lock().expect("Failed to lock symbol table mutex.");

        let struct_info = SymbolInfo::new(
            DataType::Struct, 
            SymbolValue::StructValue { fields },
        );

        current_table_lock.add(name, struct_info);

        Ok(())   
    }
}