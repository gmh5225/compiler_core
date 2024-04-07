use std::sync::MutexGuard;

use crate::frontend::{
    sem_analysis::sem_analysis_core::SemAnalysis, 
    symbol_table::symbol_table_struct::SymbolTable, 
    utils::error::ErrorType
};

impl SemAnalysis {
    pub fn check_if_assigned(&mut self, name: &String, sym_table: &MutexGuard<SymbolTable>) -> Option<ErrorType> {
        if !sym_table.get(name).is_some() {
            return Some(ErrorType::DevError{});
        }
        None
    }
}