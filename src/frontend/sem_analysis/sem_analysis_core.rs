/*
Checks a Module for semantic correctness
 */

use std::sync::{Arc, Mutex, MutexGuard};

use crate::frontend::{ 
    ast::{ 
        ast_struct::{ 
            ASTNode, ModElement, Module, AST 
        }, rules_config::RulesConfig, syntax_element::SyntaxElement
    }, symbol_table::symbol_table_struct::{SymbolTable, SymbolTableStack}, utils::error::ErrorType
};

/// Structure for the semantic analysis phase
pub struct SemAnalysis{
    input: Module,
    current_sym_table_stack: Option<Arc<Mutex<SymbolTableStack>>>,
    rules: RulesConfig, 
    current_table_index: i64,
}

impl SemAnalysis {
    fn new(input: Module, rules: RulesConfig) -> Self {
        Self {
            input,
            current_sym_table_stack: None,
            rules,
            current_table_index: 0,
        }
    }

    /// Retrieves the input module for internal use
    fn get_input(&mut self) -> &mut Module {
        &mut self.input
    }

    /// Retrieves the configuration
    pub fn get_rules_config(&self) -> &RulesConfig {
        &self.rules
    }

    /// Retrieves the input module for exporting
    pub fn get_output(self) -> Module {
        self.input
    }

    /// Retrieves the current symbol table
    pub fn get_current_sym_table(&self) -> Result<Arc<Mutex<SymbolTable>>, ErrorType> {
        match &self.current_sym_table_stack {
            Some(stack) => {
                let locked_stack: MutexGuard<'_, SymbolTableStack> = stack.lock().unwrap();
                locked_stack.get_element(self.current_table_index.try_into().unwrap())
            }
            _ => {
                panic!("Unable to retrieve current symbol table stack")
            }
        }
    }

    pub fn set_current_sym_table_stack(&mut self, sym_table_stack: Arc<Mutex<SymbolTableStack>>) -> Result<(), ErrorType> {
        self.current_sym_table_stack = Some(sym_table_stack);
        Ok(())
    }    

    /// Increments the current symbol table stack pointer
    pub fn increment_sym_table_stack_pointer(&mut self) -> Result<(), ErrorType> {
        match &self.current_sym_table_stack {
            Some(stack) => {
                let stack_unlocked  = stack.lock().unwrap();
                if self.current_table_index < stack_unlocked.get_elements().len().try_into().unwrap() {
                    self.current_table_index = self.current_table_index + 1;
                    return Ok(());
                }
                return Err(ErrorType::DevError{});
            }
            _ => return Err(ErrorType::DevError{}),
        }
    }

    /// Decrements the current symbol table stack pointer
    pub fn decrement_sym_table_stack_pointer(&mut self) -> Result<(), ErrorType> {
        match &self.current_sym_table_stack {
            Some(stack) => {
                let stack_unlocked  = stack.lock().unwrap();
                if self.current_table_index < stack_unlocked.get_elements().len().try_into().unwrap() {
                    self.current_table_index = self.current_table_index - 1;
                    return Ok(());
                }
                return Err(ErrorType::DevError{});
            }
            _ => return Err(ErrorType::DevError{}),
        }
    }

    /// Checks if a module is well formed
    pub fn sem_analysis(input: Module, rules: RulesConfig) -> Result<Module, Vec<ErrorType>> { 
        let mut semantic_analysis: SemAnalysis = SemAnalysis::new(input, rules);
    
        let mut errors: Vec<ErrorType> = Vec::new();
    
        let elements: Vec<ModElement> = semantic_analysis.get_input().get_children().clone().into_sorted_vec();
    
        for mod_element in elements {
            let ast: AST = mod_element.get_ast();
            let arc_mutex_symbol_table_stack: Arc<Mutex<SymbolTableStack>> = mod_element.get_sym_table_stack();

            semantic_analysis.set_current_sym_table_stack(arc_mutex_symbol_table_stack);

            if let Some(e) = semantic_analysis.analyze_mod_element(ast) {
                errors.extend(e);
            }
        }
    
        if errors.is_empty() {
            return Ok(semantic_analysis.get_output());
        }
        Err(errors)
    }
    
    /// Checks if a mod element is well formed
    fn analyze_mod_element(&mut self, ast: AST) -> Option<Vec<ErrorType>> {
        let mut errors: Vec<ErrorType> = Vec::new();
    
        for child in ast.get_root().get_children() {
            let root_element = ast.get_root().get_element();

            if root_element == SyntaxElement::ModuleExpression || root_element == SyntaxElement::TopLevelExpression {
                if let Some(e) = self.sem_analysis_router(&child) {
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
    
    
    /// Analyzes each node, recursively, until it has checked all nodes, and appends errors
    pub fn sem_analysis_router(&mut self, node: &ASTNode) -> Option<Vec<ErrorType>> {
        let mut errors: Vec<ErrorType> = Vec::new();

        let syn_errors: Option<Vec<ErrorType>> = match &node.get_element() {
            // --- OPAQUE EXPRESSION SECTION --- //
            SyntaxElement::NoExpression
            | SyntaxElement::ModuleExpression
            | SyntaxElement::TopLevelExpression

            // TODO
            | SyntaxElement::LoopInitializer
            | SyntaxElement::LoopIncrement 
            | SyntaxElement::Condition 
            | SyntaxElement::Action 
            | SyntaxElement::Variant 
            | SyntaxElement::AssignedValue 
            | SyntaxElement::Field 
            | SyntaxElement::Parameter 
            | SyntaxElement::Operand => { None },

    
            // --- TOP LEVEL EXPRESSION SECTION --- //
            SyntaxElement::FunctionDeclaration => {
                self.sem_function_dec(&node)
            },
            SyntaxElement::StructDeclaration => {
                self.sem_struct_dec(&node)
            },
            SyntaxElement::EnumDeclaration => {
                self.sem_enum_dec(&node)
            },

            // --- BLOCK EXPRESSION SECTION --- //
            SyntaxElement::BlockExpression => todo!(),
            SyntaxElement::ForLoop => {
                self.sem_for_loop(&node)
            },
            SyntaxElement::WhileLoop => {
                self.sem_while_loop(&node)
            },
            SyntaxElement::DoWhileLoop => {
                self.sem_do_while_loop(&node)
            },
            SyntaxElement::IfStatement => {
                self.sem_if_statement(&node)
            },


            // --- STATEMENT SECTION --- //
            SyntaxElement::BinaryExpression => {
                self.sem_bin_exp(&node)
            },
            SyntaxElement::Assignment  => {
                self.sem_assignment(&node)
            },
            SyntaxElement::Initialization => {
                self.sem_initialization(&node)
            },
            SyntaxElement::MatchStatement  => {
                self.sem_match_statement(&node)
            },
            SyntaxElement::FunctionCall  => {
                self.sem_function_call(&node)
            },
            SyntaxElement::UnaryExpression => {
                self.sem_unary_exp(&node)
            },
            SyntaxElement::Return => {
                self.sem_return(&node)
            },
            SyntaxElement::Break => {
                self.sem_break(&node)
            },
            SyntaxElement::Continue => {
                self.sem_continue(&node)
            },
            SyntaxElement::ElifStatement => todo!(),
            SyntaxElement::ElseStatement => todo!(),
            SyntaxElement::MatchArm => todo!(),

            // --- PRIMITIVE SECTION TODO --- //
            SyntaxElement::Literal { value } => todo!(),
            SyntaxElement::MutLiteral { value } => todo!(),
            SyntaxElement::Variable { is_mutable } => todo!(),
            SyntaxElement::Identifier(_) => todo!(),
            SyntaxElement::Operator(_) => todo!(),
            SyntaxElement::Type(_) => todo!(),
        };
    
        match syn_errors {
            Some(err) => {
                errors.extend(err)
            }
            _ => {}
        }
        // Recursively check each child and accumulate errors
        for child in &node.get_children() {
            let sym_errors = self.sem_analysis_router(child);
            match sym_errors {
                Some(err) => {
                    errors.extend(err)
                }
                _ => {}
            }
        }
    
        if !errors.is_empty() {
            Some(errors) 
        } else {
            None
        }
    }
}