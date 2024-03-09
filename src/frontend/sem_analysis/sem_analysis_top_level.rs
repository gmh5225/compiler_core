use std::{collections::HashSet, sync::{Arc, Mutex, MutexGuard}};

use crate::frontend::{
    ast::{
        data_type::DataType, 
        syntax_element::FunctionParameter
    }, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::symbol_table_struct::SymbolTableStack, 
    utils::error::ErrorType
};

impl<'a> SemAnalysis {
    /// Completes semantic analysis on a function declaration
    pub fn sem_function_dec(&mut self, 
        name: &String, 
        parameters: &Vec<FunctionParameter>, 
        return_type: &Option<DataType>, 
        symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) 
    -> Option<Vec<ErrorType>> {

        let mut errors: Vec<ErrorType> = Vec::new();

        let stack: MutexGuard<'_, SymbolTableStack> = symbol_table_stack.lock().unwrap();

        // deny functional polymorphism
        if let Some(current_symbol_table) = stack.peek() {
            let table = current_symbol_table.lock().unwrap();
            if table.get(name).is_some() {
                errors.push(ErrorType::DevError{})
            }
        }

        // ensure unique parameter names
        let mut param_names: HashSet<String> = HashSet::new();
        for param in parameters {
            if !param_names.insert(param.get_name()) {
                errors.push(ErrorType::DevError{})
            }
        }

        // deny unknown return types
        if let Some(return_type) = return_type{
            match return_type {
                DataType::Unknown => {
                    errors.push(ErrorType::DevError{})
                }
                _ => {}
            }
        }

        if !errors.is_empty() {
            return Some(errors);
        }
        None
    }
    
    /// TODO
    pub fn sem_struct_dec(&mut self, _name: &String, _fields: &Vec<(String, DataType)>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis of structs unimplemented")
    }

    /// TODO
    pub fn sem_enum_dec(&mut self, _name: &String, _variants: &Vec<String>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis of enums unimplemented")
    }
}