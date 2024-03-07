use compiler_core::frontend::{
    ast::{
        ast_struct::{ASTNode, AST}, data_type::DataType, syntax_element::SyntaxElement
    }, 
    symbol_table::symbol_table_struct::{SymbolTableStack, SymbolValue},
};

#[test]
fn test_function_declaration_sym_table() {
    let root: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration {
        name: "test_function".to_string(),
        parameters: vec![],
        return_type: None,
    });
    let fn_symbol_value: SymbolValue = SymbolValue::FunctionValue {
        parameters: vec![],
        return_type: None,
    };
    let ast: AST = AST::new(root);
    match SymbolTableStack::gen_sym_table_stack(ast) {
        Ok((_ast, mut sym_table_stack)) => {
            assert_eq!(sym_table_stack.size(), 2, "SymbolTableStack should have two members.");

            match sym_table_stack.pop() {
                Some(_fn_sym_table_locked) => {
                    match sym_table_stack.peek() {
                        Some(glob_sym_table_locked) => {
                            match glob_sym_table_locked.lock() {
                                Ok(sym_table) => {
                                    match sym_table.get("test_function") {
                                        Some(fn_sym) => {
                                            assert_eq!(fn_sym.get_value(), fn_symbol_value);
                                            assert_eq!(fn_sym.get_data_type(), DataType::Function);
                                        }
                                        _ => {
                                            panic!("Missing function in symbol table")
                                        }
                                    }
                                }
                                Err(e) => {
                                    panic!("PoisonError on MutexGuard: {}", e)
                                }
                            }
                        }
                        _ => {
                            panic!("No global symbol table.")
                        }
                    }
                }
                _ => {
                    panic!("No function scope symbol table found.")
                }
            }
        },
        Err(errors) => {
            assert!(errors.is_empty(), "Expected no errors, but got: {:?}", errors);
        },
    }
}

#[test]
fn test_struct_declaration_sym_table() {
    let root: ASTNode = ASTNode::new(SyntaxElement::StructDeclaration {
        name: "test_struct".to_string(),
        fields: vec![("field1".to_string(), DataType::Integer)],
    });
    let ast: AST = AST::new(root);
    match SymbolTableStack::gen_sym_table_stack(ast) {
        Ok((_ast, sym_table_stack)) => {
            assert_eq!(sym_table_stack.size(), 1, "SymbolTableStack should have one member for global scope.");

            match sym_table_stack.peek() {
                Some(glob_sym_table_locked) => {
                    match glob_sym_table_locked.lock() {
                        Ok(sym_table) => {
                            match sym_table.get("test_struct") {
                                Some(struct_sym) => {
                                    assert_eq!(struct_sym.get_data_type(), DataType::Struct);
                                }
                                _ => panic!("Missing struct in symbol table"),
                            }
                        }
                        Err(e) => panic!("PoisonError on MutexGuard: {}", e),
                    }
                }
                _ => panic!("No global symbol table."),
            }
        },
        Err(errors) => {
            assert!(errors.is_empty(), "Expected no errors, but got: {:?}", errors);
        },
    }
}

#[test]
fn test_enum_declaration_sym_table() {
    let root: ASTNode = ASTNode::new(SyntaxElement::EnumDeclaration {
        name: "test_enum".to_string(),
        variants: vec!["Variant1".to_string(), "Variant2".to_string()],
    });
    let ast: AST = AST::new(root);
    match SymbolTableStack::gen_sym_table_stack(ast) {
        Ok((_ast, sym_table_stack)) => {
            assert_eq!(sym_table_stack.size(), 1, "SymbolTableStack should have one member for global scope.");

            match sym_table_stack.peek() {
                Some(glob_sym_table_locked) => {
                    match glob_sym_table_locked.lock() {
                        Ok(sym_table) => {
                            match sym_table.get("test_enum") {
                                Some(enum_sym) => {
                                    assert_eq!(enum_sym.get_data_type(), DataType::Enum);
                                }
                                _ => panic!("Missing enum in symbol table"),
                            }
                        }
                        Err(e) => panic!("PoisonError on MutexGuard: {}", e),
                    }
                }
                _ => panic!("No global symbol table."),
            }
        },
        Err(errors) => {
            assert!(errors.is_empty(), "Expected no errors, but got: {:?}", errors);
        },
    }
}

#[test]
fn test_variable_initialization_sym_table() {
    let root: ASTNode = ASTNode::new(SyntaxElement::Initialization {
        variable: "test_var".to_string(),
        data_type: DataType::Boolean,
        value: Box::new(ASTNode::new(SyntaxElement::Literal {
            data_type: DataType::Boolean,
            value: "true".to_string(),
        })),
    });
    let ast: AST = AST::new(root);
    match SymbolTableStack::gen_sym_table_stack(ast) {
        Ok((_ast, sym_table_stack)) => {
            assert_eq!(sym_table_stack.size(), 1, "SymbolTableStack should have one member for global scope.");

            match sym_table_stack.peek() {
                Some(glob_sym_table_locked) => {
                    match glob_sym_table_locked.lock() {
                        Ok(sym_table) => {
                            match sym_table.get("test_var") {
                                Some(var_sym) => {
                                    assert_eq!(var_sym.get_data_type(), DataType::Boolean);
                                }
                                _ => panic!("Missing variable in symbol table"),
                            }
                        }
                        Err(e) => panic!("PoisonError on MutexGuard: {}", e),
                    }
                }
                _ => panic!("No global symbol table."),
            }
        },
        Err(errors) => {
            assert!(errors.is_empty(), "Expected no errors, but got: {:?}", errors);
        },
    }
}
