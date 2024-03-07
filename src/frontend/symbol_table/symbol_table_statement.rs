use crate::frontend::{
    ast::{
        ast_struct::ASTNode, 
        data_type::DataType
    }, 
    symbol_table::symbol_table_struct::SymbolTableStack, 
    utils::error::ErrorType,
    };

use super::symbol_table_struct::{SymbolInfo, SymbolValue};

impl SymbolTableStack {
    pub fn sym_table_init(&mut self, variable: String, value: Box<ASTNode>, data_type: DataType) -> Result<(), Vec<ErrorType>> {
        let current_table = match self.peek() {
            Some(table) => table,
            None => panic!("No symbol table on the stack."),
        };

        let mut current_table_lock = current_table.lock().expect("Failed to lock symbol table mutex.");

        let init_info = SymbolInfo::new(
            data_type,
            SymbolValue::Node(value)
        );

        current_table_lock.add(variable, init_info);

        Ok(())
    }
}