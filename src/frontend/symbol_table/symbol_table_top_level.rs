use std::sync::{Arc, Mutex, MutexGuard};

use crate::
    frontend::{
        ast::{ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement}, 
        symbol_table::symbol_table_struct::SymbolTableStack, 
        utils::error::ErrorType
};

use super::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolValue};

impl SymbolTableStack {
    /// Adds a function type to the current scope
    pub fn sym_table_fn(&mut self, fn_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let current_table: Arc<Mutex<SymbolTable>> = match self.peek() { 
            Some(table) => table, 
            None => panic!("No symbol table on the stack."), 
        };

        let mut current_table_lock: MutexGuard<'_, SymbolTable> = current_table.lock().expect("Failed to lock symbol table mutex.");

        let children: Vec<ASTNode> = fn_node.get_children();

        let mut fn_name: Option<String> = None;
        let mut fn_type: Option<DataType> = None;
        let mut params: Vec<(String, DataType)> = Vec::new();
        let mut body: Option<&ASTNode> = None;
         
        for child in children.iter() {
            match child.get_element() {
                SyntaxElement::Identifier(name) => {
                    if fn_name.is_none() { 
                        fn_name = Some(name.clone());
                    }
                },
                SyntaxElement::Type(set_type) => {
                    if fn_type.is_none() { 
                        fn_type = Some(set_type.clone());
                    }
                },
                SyntaxElement::Parameter => {
                    let mut name: Option<String> = None;
                    let mut param_type: Option<DataType> = None;
                    for grandchild in child.get_children() {
                        match grandchild.get_element() {
                            SyntaxElement::Identifier(param_name) => {
                                name = Some(param_name.clone());
                            },
                            SyntaxElement::Type(grandchild_type) => {
                                param_type = Some(grandchild_type.clone());
                            },
                            _ => {}
                        }
                    }
                    match name {
                        Some(name) => {
                            match param_type {
                                Some(param_type) => {
                                    params.push((name, param_type))
                                }
                                _ => panic!("Missing param type")
                            }
                        }
                        _ => panic!("Missing name")
                    }

                },
                SyntaxElement::BlockExpression => {
                    body = Some(child);
                }

                _ => {}
            }
        }

        let fn_info: SymbolInfo = SymbolInfo::new(
            DataType::Function,
            SymbolValue::FunctionValue { parameters: params, return_type: fn_type}
        );

        match fn_name {
            Some(name) => {
                current_table_lock.add(name, fn_info);
            }
            _ => {
                panic!("Missing fn name")
            }
        }

        let fn_scope: SymbolTable = SymbolTable::new();
        self.push(fn_scope);
        
        match body {
            Some(node) => {
                self.sym_table_stack_router(node)?;
            }
            _ => {
                panic!("Missing fn body")
            }
        }

        Ok(())
    }

    /// Adds an enum type to the current scope
    pub fn sym_table_enum(&mut self, enum_node: &ASTNode) -> Result<(), Vec<ErrorType>> {  
        // let current_table = match self.peek() {
        //     Some(table) => table,
        //     None => panic!("No symbol table on the stack."),
        // };

        // let mut current_table_lock: MutexGuard<'_, SymbolTable> = current_table.lock().expect("Failed to lock symbol table mutex");

        // let mut variants: Vec<String> = Vec::new();
        // let mut enum_name: String;

        // for child in enum_node.get_children().iter() {
        //     match child.get_element() {
        //         SyntaxElement::Identifier(enum_name_str) => {
        //             enum_name = enum_name_str;
        //         }
        //         SyntaxElement::Variant => {
        //             for grandchild in child.get_children() {
        //                 let variant_id: String;
        //                 let variant_node: Option<ASTNode> = None;
        //                 match grandchild.get_element() {
        //                     SyntaxElement::Identifier(variant_str) => {
        //                         variant_id = variant_str;
        //                     }
        //                     _ => {}
        //                 }
        //                 variants.push(variant_id);
        //             }
        //         }
        //         _ => {}
        //     }   
        // }
        // let enum_info: SymbolInfo = SymbolInfo::new(
        //     DataType::Enum,
        //     SymbolValue::EnumValue { variants }
        // );

        // current_table_lock.add(enum_name, enum_info);
        Ok(())
    }


    /// Adds a struct type to the current scope
    pub fn sym_table_struct(&mut self, struct_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        // let current_table = match self.peek() {
        //     Some(table) => table,
        //     None => panic!("No symbol table on the stack."),
        // };
        
        // let mut current_table_lock: MutexGuard<'_, SymbolTable> = current_table.lock().expect("Failed to lock symbol table mutex.");
        // let mut fields: Vec<(String, DataType)> = Vec::new();
        // let mut struct_name: String;

        // for child in struct_node.get_children().iter() {
        //     match child.get_element() {
        //         SyntaxElement::Identifier(struct_name_str) => {
        //             struct_name = struct_name_str;
        //         }
        //         SyntaxElement::Field => {
        //             for grandchild in child.get_children() {
        //                 let field_id: String;
        //                 let field_type: DataType;
        //                 match grandchild.get_element() {
        //                     SyntaxElement::Identifier(field_str) => {
        //                         field_id = field_str;
        //                     }
        //                     SyntaxElement::Type(data_type) => {
        //                         field_type = data_type;
        //                     }
        //                     _ => {}
        //                 }
        //                 fields.push((field_id, field_type));
        //             }
        //         }
        //         _ => {}
        //     }
        // }
        // let struct_info = SymbolInfo::new(
        //     DataType::Struct, 
        //     SymbolValue::StructValue { fields },
        // );

        // current_table_lock.add(struct_name, struct_info);
        Ok(())   
    }
}