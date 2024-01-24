use std::{collections::HashMap, sync::Arc};

use crate::frontend::{
    ast::syntax_element::SyntaxElement, 
    ast_pass::symbol_table::SymbolTable, 
    utils::error::ErrorType
};


#[derive(Clone)]
pub struct SemanticRule {
    check: Arc<dyn Fn(&SyntaxElement, &mut SymbolTable) -> Option<ErrorType>>,
}

pub struct RulesConfig {
    rules: HashMap<SyntaxElement, Vec<SemanticRule>>
}

impl RulesConfig {
    pub fn new(rules: HashMap<SyntaxElement, Vec<SemanticRule>>) -> Self {
        Self {
            rules
        }
    }

    pub fn get_rules_config(&self) -> &HashMap<SyntaxElement, Vec<SemanticRule>> {
        &self.rules
    }

    pub fn get_rules(&self, element: &SyntaxElement) -> Vec<SemanticRule> {
        match self.rules.get(element) {
            Some(rules) => rules.to_vec(),
            None => Vec::new(),
        }
    }
}

impl SemanticRule {
    pub fn new<F>(check_fn: F) -> Self 
    where
        F: Fn(&SyntaxElement, &mut SymbolTable) -> Option<ErrorType> + 'static
    {
        Self {
            check: Arc::new(check_fn),
        }
    }

    pub fn apply_rule(&self, element: &SyntaxElement, symbol_table: &mut SymbolTable) -> Option<ErrorType> {
        (self.check)(element, symbol_table)
    }
}