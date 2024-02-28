use std::sync::{Arc, Mutex, MutexGuard};

use crate::frontend::{
    ast::{
        ast_struct::ASTNode, 
        data_type::DataType, 
        syntax_element::MatchArm
    }, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::{
        symbol_table_core::SymbolTableStack, 
        symbol_table_struct::SymbolTable,
    }, 
    utils::error::ErrorType,
};

impl<'a> SemAnalysis {
    pub fn sem_assignment(&mut self, 
                        variable: &String, 
                        value: &Box<ASTNode>, 
                        symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) 
                            -> Option<Vec<ErrorType>> {
        let mut errors: Vec<ErrorType> = Vec::new();
        let stack: MutexGuard<'_, SymbolTableStack> = symbol_table_stack.lock().unwrap();
        
        let mut var_found: bool = false;

        for symbol_table_arc in stack.get_elements().iter().rev() {
            let sym_table: MutexGuard<'_, SymbolTable> = symbol_table_arc.lock().unwrap();

            if sym_table.get(variable).is_some() {
                var_found = true;
                break;
            }
        }

        if !var_found {
            errors.push(ErrorType::DevError{})
        }
        if !errors.is_empty() {
            return Some(errors);
        }
        None
    }

    pub fn sem_bin_exp(&mut self, left: &Box<ASTNode>, operator: &String, right: &Box<ASTNode>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis bin exp unimplemented")
    }

    pub fn sem_initialization(&mut self, variable: &String, data_type: &DataType, value: &Box<ASTNode>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis init unimplemented")
    }

    pub fn sem_match_statement(&mut self, to_match: &Box<ASTNode>, arms: &Vec<MatchArm>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis match unimplemented")
    }

    pub fn sem_function_call(&mut self, name: &String, arguments: &Vec<ASTNode>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis fn call unimplemented")
    }

    pub fn sem_unary_exp(&mut self, operator: &String, operand: &Box<ASTNode>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis unary op unimplemented")
    }

    pub fn sem_return(&mut self, value: &Box<ASTNode>, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis return unimplemented")
    }

    pub fn sem_break(&mut self, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis break unimplemented")
    }

    pub fn sem_continue(&mut self, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis continue unimplemented")
    }

}