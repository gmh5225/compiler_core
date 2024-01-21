/*
Checks an AST for semantic correctness
 */

use std::sync::{Mutex, Arc};

use crate::frontend::{ 
    ast::{ 
        ast_struct::{ 
            ASTNode, 
            ModAST, AST, ModElement, 
        }, 
        syntax_element::SyntaxElement, 
        sem_rule::{RulesConfig, SemanticRule} 
    },
    utils::error::ErrorType,
    analysis::symbol_table::SymbolTableStack,
};

use super::symbol_table::SymbolTable;

pub struct SemAnalysis{
    input: ModAST,
    rules: RulesConfig,
}

impl<'a> SemAnalysis {
    fn new(input: ModAST, rules: RulesConfig) -> Self {
        Self {
            input,
            rules,
        }
    }

    pub fn get_input(&mut self) -> &mut ModAST {
        &mut self.input
    }
    pub fn get_rules_config(&self) -> &RulesConfig {
        &self.rules
    }

    pub fn output(self) -> ModAST {
        self.input
    }

    /// checks an ast for semantic correctness
    pub fn sem_analysis(input: ModAST, rules: RulesConfig) -> Result<ModAST, Vec<ErrorType>> { 
        let mut semantic_analysis: SemAnalysis = SemAnalysis::new(input, rules);
    
        let mut errors: Vec<ErrorType> = Vec::new();
    
        let elements: Vec<ModElement> = semantic_analysis.get_input().get_children().clone().into_sorted_vec();
    
        for mod_element in elements {
            let ast: AST = mod_element.get_ast();
            let arc_mutex_symbol_table_stack = mod_element.get_sym_table_stack();
            
            if let Some(e) = semantic_analysis.analyze_mod(ast, &arc_mutex_symbol_table_stack) {
                errors.extend(e);
            }
        }
    
        if errors.is_empty() {
            return Ok(semantic_analysis.output());
        }
        Err(errors)
    }
    

    fn analyze_mod(&mut self, ast: AST, arc_mutex_symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        let mut errors: Vec<ErrorType> = Vec::new();

        if let Ok(mut symbol_table_stack) = arc_mutex_symbol_table_stack.lock() {
            if ast.get_root().get_element() == SyntaxElement::ModuleExpression {
                for child in ast.get_root().get_children() {
                    if let Some(symbol_table_arc_mutex) = symbol_table_stack.pop() {
                        if let Ok(mut symbol_table) = symbol_table_arc_mutex.lock() {
                            if let Some(e) = self.sem_analysis_router(&child, &mut errors, &mut symbol_table) {
                                errors.extend(e);
                            }
                        }
                    }
                }
            }
        } else {
            panic!("failed to acquire lock in analyze_mod");
        }

        if errors.is_empty() { 
            return None;
        } 
        Some(errors)
    }
    
    
    /// analyzes each node, recursively, until it has checked all nodes, and appends errors
    fn sem_analysis_router(&mut self, node: &ASTNode, errors: &mut Vec<ErrorType>, symbol_table: &mut SymbolTable) -> Option<Vec<ErrorType>> {
        match &node.get_element() {
            SyntaxElement::BinaryExpression { left, operator, right } => {
                let def_ast: &SyntaxElement = &SyntaxElement::BinaryExpression { left: Box::new(ASTNode::default()), operator: operator.clone(), right: Box::new(ASTNode::default())};
                let rules: Vec<SemanticRule> = self.get_rules_config().get_rules(def_ast);
                for rule in rules {
                    match rule.apply_rule(&node.get_element(), symbol_table) {
                        Some(e) => {
                            errors.push(e)
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        for child in &node.get_children() {
            if let Some(child_errors) = self.sem_analysis_router(child, errors, symbol_table) {
                errors.extend(child_errors); 
            }
        }

        if !errors.is_empty() {
            return Some(errors.to_vec());
        }
        None
    }

}
