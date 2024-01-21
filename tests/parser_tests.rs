use compiler_core::frontend::{
    ast::{
        ast_struct::*,
        data_type::*,
        syntax_element::*,
    },
    syntax::token::*,
    parser::parser_core::*,
};

/// cargo test --test parser_tests
/// panic!("{:?}", self.get_input().get(self.get_current()));

#[test]
fn test_empty_input() { 
    let tokens: Vec<Token> = vec![];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_element(), SyntaxElement::TopLevelExpression);
    assert_eq!(sym_table.size(), 1);
    assert!(ast.get_root().get_children().is_empty());
}

#[test]
fn test_single_function_declaration() {
    let tokens: Vec<Token> = vec![
        Token::FUNCTION,
        Token::IDENTIFIER(vec!['m', 'y', '_', 'f', 'u', 'n', 'c']),
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(sym_table.size(), 2);
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::FunctionDeclaration { ref name, .. } => {
            assert_eq!(name, "my_func");
        },
        _ => panic!("Expected FunctionDeclaration"),
    }
}

#[test]
fn test_function_with_parameters_and_return_type() { 
    //fn calculate (x:Integer,y:Integer):Boolean {}
    let tokens: Vec<Token> = vec![
        Token::FUNCTION,
        Token::IDENTIFIER(vec!['c', 'a', 'l', 'c', 'u', 'l', 'a', 't', 'e']),
        Token::LPAREN,
        Token::IDENTIFIER(vec!['x']),
        Token::COLON,
        Token::TINTEGER,
        Token::COMMA,
        Token::IDENTIFIER(vec!['y']),
        Token::COLON,
        Token::TINTEGER,
        Token::RPAREN,
        Token::COLON, 
        Token::TBOOLEAN,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::FunctionDeclaration { ref name, ref parameters, .. } => {
            assert_eq!(name, "calculate");
            assert_eq!(parameters.len(), 2);
        },
        _ => panic!("Expected FunctionDeclaration with parameters"),
    }
}

#[test]
fn test_function_with_body() {
    let tokens: Vec<Token> = vec![
        Token::FUNCTION,
        Token::IDENTIFIER(vec!['t', 'e', 's', 't']),
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACKET,
        Token::LET,
        Token::IDENTIFIER(vec!['x']),
        Token::COLON,
        Token::TINTEGER,
        Token::EQUAL,
        Token::INT(vec!['1']),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::FunctionDeclaration { ref name, .. } => {
            assert_eq!(name, "test");
            assert!(!ast.get_root().get_children().is_empty());
        },
        _ => panic!("Expected FunctionDeclaration with body"),
    }
}

#[test]
fn test_if_statement_parsing() {
    let tokens = vec![
        Token::IF,
        Token::LPAREN,
        Token::TRUE,
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::TRUE,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::IfStatement { condition, then_branch, else_branch } => {
            match condition.get_element() {
                SyntaxElement::Literal { data_type, value } => {
                    assert_eq!(data_type, DataType::Boolean);
                    assert_eq!(value, "true");
                },
                _ => panic!("Expected Literal"),
            }            assert!(else_branch.is_none());
        },
        _ => panic!("Expected IfStatement"),
    }
}

#[test]
fn test_initialization_parsing() {
    let tokens = vec![
        Token::LET,
        Token::IDENTIFIER(vec!['x']),
        Token::COLON,
        Token::TBOOLEAN,
        Token::EQUAL,
        Token::TRUE,
        Token::SEMICOLON,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::Initialization { variable, data_type, value } => {
            assert_eq!(variable, "x");
            match value.get_element() {
                SyntaxElement::Literal { data_type, value } => {
                    assert_eq!(data_type, DataType::Boolean);
                    assert_eq!(value, "true");
                },
                _ => panic!("Expected Literal"),
            }   
            },
        _ => panic!("Expected Initialization"),
    }
}

#[test]
fn test_for_loop_parsing() {
    let tokens = vec![
        Token::FOR,
        Token::LPAREN,
        Token::TRUE, 
        Token::SEMICOLON,
        Token::RPAREN,
        Token::LBRACKET,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];

    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::ForLoop { initializer, condition, increment, body } => {
            assert!(initializer.is_none(), "Expected no initializer in ForLoop");
            
            match condition.get_element() {
                SyntaxElement::Literal { data_type, value } => {
                    assert_eq!(data_type, DataType::Boolean);
                    assert_eq!(value, "true");
                },
                _ => panic!("Expected Literal"),
            }

            assert!(increment.is_none(), "Expected no increment in ForLoop");

            let body_nodes: &Vec<ASTNode> = &(*body);
            match body_nodes.first() {
                Some(node) => match node.get_element() {
                    SyntaxElement::Break => {},
                    _ => panic!("Body's first node is not a BreakStatement"),
                },
                None => panic!("Body is empty"),
            }
        },
        _ => panic!("Expected ForLoop"),
    }
}


#[test]
fn test_while_loop_parsing() {
    let tokens = vec![
        Token::WHILE,
        Token::LPAREN,
        Token::TRUE,
        Token::RPAREN,
        Token::LBRACKET,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::WhileLoop { condition, body } => {
            match condition.get_element() {
                SyntaxElement::Literal { data_type, value } => {
                    assert_eq!(data_type, DataType::Boolean);
                    assert_eq!(value, "true");
                },
                _ => panic!("Expected Literal"),
            }  

            let body_nodes = &(*body);
            match body_nodes.first() {
                Some(node) => {
                    match node.get_element() {
                        SyntaxElement::Break => {},
                        _ => panic!("Body's first node is not a BreakStatement"),
                    }
                },
                None => panic!("Body is empty"),
            }
        },
        _ => panic!("Expected WhileLoop"),
    }
}

#[test]
fn test_do_while_loop_parsing() {
    let tokens: Vec<Token> = vec![
        Token::DO,
        Token::LBRACKET,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::WHILE,
        Token::LPAREN,
        Token::TRUE,
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::DoWhileLoop { body, condition } => {
            let body_nodes = &(*body);
            match body_nodes.first() {
                Some(node) => {
                    match node.get_element() {
                        SyntaxElement::Break => {},
                        _ => panic!("Body's first node is not a BreakStatement"),
                    }
                },
                None => panic!("Body is empty"),
            }

            match condition.get_element() {
                SyntaxElement::Literal { data_type, value } => {
                    assert_eq!(data_type, DataType::Boolean);
                    assert_eq!(value, "true");
                },
                _ => panic!("Expected Literal"),
            }
        },
        _ => panic!("Expected DoWhileLoop"),
    }
}

#[test]
fn test_continue_statement_parsing() {
    let tokens = vec![
        Token::CONTINUE,
        Token::SEMICOLON,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::Continue => {  },
        _ => panic!("Expected ContinueStatement"),
    }
}

#[test]
fn test_match_statement_parsing() {
    let tokens = vec![
        Token::MATCH,
        Token::IDENTIFIER(vec!['x']),
        Token::LBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];
    let (ast, sym_table) = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::MatchStatement { to_match, arms } => {
            match to_match.get_element() {
                SyntaxElement::Variable{data_type, name} => {
                    assert_eq!(name, "x");
                    assert_eq!(data_type, DataType::Unknown);
                },
                _ => panic!("Expected Variable in to_match"),
            }
            assert!(arms.is_empty(), "Expected no match arms");
        },
        _ => panic!("Expected MatchStatement"),
    }
}