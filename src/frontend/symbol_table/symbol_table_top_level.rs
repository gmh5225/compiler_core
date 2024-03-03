use crate::
    frontend::{
        ast::{data_type::DataType, syntax_element::FunctionParameter}, 
        utils::error::ErrorType,
        symbol_table::symbol_table_struct::SymbolTableStack,
};

impl SymbolTableStack {
    pub fn sym_table_fn(&mut self, name: String, parameters: Vec<FunctionParameter>, return_type: Option<DataType>) -> Result<(), Vec<ErrorType>> {
        Ok(())
    }

    pub fn sym_table_enum(&mut self) -> Result<(), Vec<ErrorType>> {  
        Ok(())
    }

    pub fn sym_table_struct(&mut self) -> Result<(), Vec<ErrorType>> {
        Ok(())
    }
}