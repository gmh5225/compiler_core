use std::sync::{Arc, Mutex, MutexGuard};

use crate::frontend::{
    ast::{
        ast_struct::ASTNode, 
        data_type::DataType, 
        syntax_element::MatchArm
    }, 
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::{
        symbol_table_struct::SymbolTableStack, 
        symbol_table_struct::SymbolTable,
    }, 
    utils::error::ErrorType,
};

impl<'a> SemAnalysis {
    /// Completes semantic analysis of variable assignment
    pub fn sem_assignment(&mut self, 
        variable: &String, 
        _value: &Box<ASTNode>, 
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

    /// TODO
    pub fn sem_bin_exp(&mut self, _left: &Box<ASTNode>, _operator: &String, _right: &Box<ASTNode>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis bin exp unimplemented")
    }

    /// TODO
    pub fn sem_initialization(&mut self, _variable: &String, _data_type: &DataType, _value: &Box<ASTNode>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis init unimplemented")
    }

    /// TODO
    pub fn sem_match_statement(&mut self, _to_match: &Box<ASTNode>, _arms: &Vec<MatchArm>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis match unimplemented")
    }

    /// TODO
    pub fn sem_function_call(&mut self, _name: &String, _arguments: &Vec<ASTNode>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis fn call unimplemented")
    }

    /// TODO
    pub fn sem_unary_exp(&mut self, _operator: &String, _operand: &Box<ASTNode>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis unary op unimplemented")
    }

    /// TODO
    pub fn sem_return(&mut self, _value: &Box<ASTNode>, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis return unimplemented")
    }

    /// TODO
    pub fn sem_break(&mut self, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis break unimplemented")
    }

    /// TODO
    pub fn sem_continue(&mut self, _symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        unimplemented!("Sem analysis continue unimplemented")
    }

}