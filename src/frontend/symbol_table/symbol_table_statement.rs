use std::sync::{Arc, Mutex, MutexGuard};

use crate::{constants::DEFAULT_MUTABILITY_VARIABLES, frontend::{
    ast::{
        ast_struct::ASTNode, 
        data_type::DataType, syntax_element::SyntaxElement
    }, 
    symbol_table::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue}, 
    utils::error::ErrorType,
}};


impl SymbolTableStack {
    /// Adds a new variable into the current scope
    pub fn sym_table_init(&mut self, init_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let current_table: Arc<Mutex<SymbolTable>> = match self.peek() {
            Some(table) => table,
            None => panic!("No symbol table on the stack."),
        };

        let mut current_table_unlocked: MutexGuard<'_, SymbolTable> = current_table.lock().expect("Failed to lock symbol table mutex.");

        let children: Vec<ASTNode> = init_node.get_children();

        let mut var_name: Option<String> = None;
        let mut var_type: Option<DataType> = None;
        let mut initial_value: Option<ASTNode> = None;

        for child in children.iter() {
            match child.get_element() {
                SyntaxElement::Variable => {
                    for grandchild in child.get_children() {
                        match grandchild.get_element() {
                            SyntaxElement::Identifier(name) => {
                                var_name = Some(name);
                            },
                            SyntaxElement::Type(typed) => {
                                var_type = Some(typed);
                            },
                            _ => {
                                panic!("Unexpected node1: {:?}", grandchild)
                            }
                        }
                    }
                },
                SyntaxElement::AssignedValue => {
                    initial_value = Some(child.clone());
                }
                _ => {
                    panic!("Unexpected node: {:?}", child)
                }
            }
        } 

        let mut init_info: Option<SymbolInfo> = None;

        match var_type {
            Some(typed) => {
                match initial_value {
                    Some(init_value) => {
                        init_info = Some(SymbolInfo::new(
                            typed,
                            SymbolValue::Node(init_value)
                        ));
                    }
                    _ => {
                        panic!("Missing init value")
                    }
                }
            }
            _ => {
                panic!("Missing init type")
            }
        }

        match init_info {
            Some(node) => {
                match var_name {
                    Some(name) => {
                        current_table_unlocked.add(name, node);
                    }
                    _ => {
                        panic!("Problem with var name init node sts")
                    }
                }
            }
            _ => {
                panic!("Problem in sts init node")
            }
        }
        Ok(())
    }
}