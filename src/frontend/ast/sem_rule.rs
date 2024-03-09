use std::{collections::HashMap, sync::Arc};

use crate::frontend::{
    ast::syntax_element::SyntaxElement, 
    symbol_table::symbol_table_struct::SymbolTable, 
    utils::error::ErrorType
};

/// Semantic rule
#[derive(Clone)]
pub struct SemanticRule {
    check: Arc<dyn Fn(&SyntaxElement, &mut SymbolTable) -> Option<ErrorType>>,
}

/// Rules configuration
pub struct RulesConfig {
    rules: HashMap<SyntaxElement, Vec<SemanticRule>>
}

impl RulesConfig {
    /// New configuration
    pub fn new(rules: HashMap<SyntaxElement, Vec<SemanticRule>>) -> Self {
        Self {
            rules
        }
    }

    /// Retrieve complete configuration of rules
    pub fn get_rules_config(&self) -> &HashMap<SyntaxElement, Vec<SemanticRule>> {
        &self.rules
    }

    /// Retrieve rules for syntaxelement
    pub fn get_rules(&self, element: &SyntaxElement) -> Vec<SemanticRule> {
        match self.rules.get(element) {
            Some(rules) => rules.to_vec(),
            None => Vec::new(),
        }
    }
}

impl SemanticRule {
    /// New semantic rule
    pub fn new<F>(check_fn: F) -> Self 
    where
        F: Fn(&SyntaxElement, &mut SymbolTable) -> Option<ErrorType> + 'static
    {
        Self {
            check: Arc::new(check_fn),
        }
    }

    /// Execute rule on element given symbol table
    pub fn apply_rule(&self, element: &SyntaxElement, symbol_table: &mut SymbolTable) -> Option<ErrorType> {
        (self.check)(element, symbol_table)
    }
}