use std::{collections::HashSet, sync::{Arc, Mutex, MutexGuard}};

use crate::frontend::{
    ast::{
        data_type::DataType, 
        syntax_element::FunctionParameter
    }, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::symbol_table::SymbolTableStack, 
    utils::error::ErrorType
};

impl<'a> SemAnalysis {
    pub fn sem_function_dec(&mut self, 
                            name: &String, 
                            parameters: &Vec<FunctionParameter>, 
                            return_type: &Option<DataType>, 
                            symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) 
                                -> Option<Vec<ErrorType>> {
        let mut errors: Vec<ErrorType> = Vec::new();

        // can be mutable but doesn't need to be for now
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
    
    pub fn sem_struct_dec(&mut self, name: &String, fields: &Vec<(String, DataType)>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis of structs unimplemented")
    }
    pub fn sem_enum_dec(&mut self, name: &String, variants: &Vec<String>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis of enums unimplemented")
    }
}