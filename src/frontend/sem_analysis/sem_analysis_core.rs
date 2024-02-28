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
        sem_rule::RulesConfig
    },
    utils::error::ErrorType,
    symbol_table::symbol_table_core::SymbolTableStack,
};

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
            let arc_mutex_symbol_table_stack: Arc<Mutex<SymbolTableStack>> = mod_element.get_sym_table_stack();
            
            if let Some(e) = semantic_analysis.analyze_mod(ast, &arc_mutex_symbol_table_stack) {
                errors.extend(e);
            }
        }
    
        if errors.is_empty() {
            return Ok(semantic_analysis.output());
        }
        Err(errors)
    }
    

    fn analyze_mod(&mut self, ast: AST, symbol_table_stack: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        let mut errors: Vec<ErrorType> = Vec::new();
    
        for child in ast.get_root().get_children() {
            let root_element = ast.get_root().get_element();

            if root_element == SyntaxElement::ModuleExpression || root_element == SyntaxElement::TopLevelExpression {
                if let Some(e) = self.sem_analysis_router(&child, symbol_table_stack) {
                    errors.extend(e);
                }
                    
            }
        }
    
        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }
    
    
    /// analyzes each node, recursively, until it has checked all nodes, and appends errors
    fn sem_analysis_router(&mut self, node: &ASTNode, symbol_table: &Arc<Mutex<SymbolTableStack>>) -> Option<Vec<ErrorType>> {
        let mut acc_errors: Vec<ErrorType> = Vec::new();

        let syn_errors: Option<Vec<ErrorType>> = match &node.get_element() {
            SyntaxElement::NoExpression
            | SyntaxElement::ModuleExpression
            | SyntaxElement::TopLevelExpression => { None },
    
            // top level
            SyntaxElement::FunctionDeclaration { name, parameters, return_type } => {
                self.sem_function_dec(name, parameters, return_type, symbol_table)
            },
            SyntaxElement::StructDeclaration { name, fields } => {
                self.sem_struct_dec(name, fields, symbol_table)
            },
            SyntaxElement::EnumDeclaration { name, variants } => {
                self.sem_enum_dec(name, variants, symbol_table)
            },

            // block
            SyntaxElement::ForLoop { initializer, condition, increment, body } => {
                self.sem_for_loop(initializer, condition, increment, body, symbol_table)
            },
            SyntaxElement::WhileLoop { condition, body } => {
                self.sem_while_loop(condition, body, symbol_table)
            },
            SyntaxElement::DoWhileLoop { body, condition } => {
                self.sem_do_while_loop(body, condition, symbol_table)
            },
            SyntaxElement::IfStatement { condition, then_branch, else_branch } => {
                self.sem_if_statement(condition, then_branch, else_branch, symbol_table)
            },

            // statement
            SyntaxElement::BinaryExpression { left, operator, right } => {
                self.sem_bin_exp(left, operator, right, symbol_table)
            },
            SyntaxElement::Assignment { variable, value } => {
                self.sem_assignment(variable, value, symbol_table)
            },
            SyntaxElement::Initialization { variable, data_type, value } => {
                self.sem_initialization(variable, data_type, value, symbol_table)
            },
            SyntaxElement::MatchStatement { to_match, arms } => {
                self.sem_match_statement(to_match, arms, symbol_table)
            },
            SyntaxElement::FunctionCall { name, arguments } => {
                self.sem_function_call(name, arguments, symbol_table)
            },
            SyntaxElement::UnaryExpression { operator, operand } => {
                self.sem_unary_exp(operator, operand, symbol_table)
            },
            SyntaxElement::Return { value } => {
                self.sem_return(value, symbol_table)
            },
            SyntaxElement::Break => {
                self.sem_break(symbol_table)
            },
            SyntaxElement::Continue => {
                self.sem_continue(symbol_table)
            },

            // these might not be necessary or maybe they should be errors if hitting here? not sure
            // SyntaxElement::Literal { data_type, value } => {
            //     self.sem_literal(data_type, value, symbol_table)
            // },
            // SyntaxElement::Variable { data_type, name } => {
            //     self.sem_variable(data_type, name, symbol_table)
            // },
            _ => {panic!("Should this be hit?")}
        };
    
        match syn_errors {
            Some(err) => {
                acc_errors.extend(err)
            }
            _ => {}
        }
        // Recursively check each child and accumulate errors
        for child in &node.get_children() {
            let syn_errors = self.sem_analysis_router(child, symbol_table);
            match syn_errors {
                Some(err) => {
                    acc_errors.extend(err)
                }
                _ => {}
            }
        }
    
        if !acc_errors.is_empty() {
            Some(acc_errors.clone()) 
        } else {
            None
        }
    }
    

}
