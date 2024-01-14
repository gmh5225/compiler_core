use compiler_design::frontend::{
    ast::{
        ast_struct::*,
        data_type::*,
        syntax_element::*,
    },
    syntax::token::*,
    parser::parser_core::*,
};

#[test]
fn test_empty_input() { 
    let tokens: Vec<Token> = vec![];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_element(), SyntaxElement::ModuleExpression);
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
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::FunctionDeclaration { ref name, .. } => {
            assert_eq!(name, "my_func");
        },
        _ => panic!("Expected FunctionDeclaration"),
    }
}

#[test]
fn test_function_with_parameters_and_return_type() { 
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
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");
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
let tokens = vec![
    Token::FUNCTION,
    Token::IDENTIFIER(vec!['t', 'e', 's', 't']),
    Token::LPAREN,
    Token::RPAREN,
    Token::LBRACKET,
    Token::LET,
    Token::IDENTIFIER(vec!['x']),
    Token::EQUAL,
    Token::INT(vec!['1']),
    Token::SEMICOLON,
    Token::RBRACKET,
    Token::EOF,
];
let ast = Parser::parse(tokens).expect("Failed to parse");
match ast.get_root().get_children().first().unwrap().get_element() {
    SyntaxElement::FunctionDeclaration { ref name, .. } => {
        assert_eq!(name, "test");
        assert!(!ast.get_root().get_children().is_empty());
    },
    _ => panic!("Expected FunctionDeclaration with body"),
}
}

#[test]
fn test_literal_parsing() {
    let tokens: Vec<Token> = vec![
        Token::INT(vec!['1', '2', '3']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::Literal(DataType::Integer, value) => assert_eq!(value, "123"),
        _ => panic!("Expected Literal"),
    }
}

#[test]
fn test_binary_expression_parsing() {
    let tokens = vec![
        Token::INT(vec!['1']),
        Token::PLUS,
        Token::INT(vec!['2']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::BinaryExpression { left, operator, right } => {
            assert_eq!(*left, ASTNode::new(SyntaxElement::Literal(DataType::Integer, "1".to_string())));
            assert_eq!(*right, ASTNode::new(SyntaxElement::Literal(DataType::Integer, "2".to_string())));
            assert_eq!(operator, "+");
        },
        _ => panic!("Expected BinaryExpression"),
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
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::IfStatement { condition, then_branch, else_branch } => {
            assert_eq!(*condition, ASTNode::new(SyntaxElement::Literal(DataType::Boolean, "true".to_string())));
            assert!(else_branch.is_none());
        },
        _ => panic!("Expected IfStatement"),
    }
}
#[test]
fn test_assignment_parsing() {
    let tokens = vec![
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::INT(vec!['1']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::Assignment { variable, value } => {
            assert_eq!(variable, "x");
            assert_eq!(*value, ASTNode::new(SyntaxElement::Literal(DataType::Integer, "1".to_string())));
        },
        _ => panic!("Expected Assignment"),
    }
}
#[test]
fn test_initialization_parsing() {
    let tokens = vec![
        Token::LET,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::INT(vec!['1']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::Initialization { variable, value } => {
            assert_eq!(variable, "x");
            assert_eq!(*value, ASTNode::new(SyntaxElement::Literal(DataType::Integer, "1".to_string())));
        },
        _ => panic!("Expected Initialization"),
    }
}
#[test]
fn test_for_loop_parsing() {
    let tokens = vec![
        Token::FOR,
        Token::LPAREN,
        Token::LET,
        Token::IDENTIFIER(vec!['i']),
        Token::EQUAL,
        Token::INT(vec!['0']),
        Token::SEMICOLON,
        Token::IDENTIFIER(vec!['i']),
        Token::LESSTHAN,
        Token::INT(vec!['1', '0']),
        Token::SEMICOLON,
        Token::IDENTIFIER(vec!['i']),
        Token::PLUSASSIGN,
        Token::INT(vec!['1']),
        Token::RPAREN,
        Token::LBRACKET,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::ForLoop { initializer, condition, increment, body } => {
            match initializer.as_ref().unwrap().get_element() {
                SyntaxElement::Initialization { variable, value } => {
                    assert_eq!(variable, "i");
                    match value.get_element() {
                        SyntaxElement::Literal(DataType::Integer, val) => {
                            assert_eq!(val, "0");
                        },
                        _ => panic!("Expected Literal for initializer value"),
                    }
                },
                _ => panic!("Initializer is not an Initialization SyntaxElement"),
            }
            match condition.get_element() {
                SyntaxElement::BinaryExpression { left, operator, right } => {
                    match left.get_element() {
                        SyntaxElement::Variable(name) => assert_eq!(name, "i"),
                        _ => panic!("Expected Variable in condition's left expression"),
                    }
                    assert_eq!(operator, "<");
                    match right.get_element() {
                        SyntaxElement::Literal(DataType::Integer, val) => assert_eq!(val, "10"),
                        _ => panic!("Expected Literal in condition's right expression"),
                    }
                },
                _ => panic!("Condition is not a BinaryExpression"),
            }
            match increment.as_ref().unwrap().get_element() {
                SyntaxElement::Assignment { variable, value } => {
                    assert_eq!(variable, "i");
                    match value.get_element() {
                        SyntaxElement::Literal(DataType::Integer, val) => assert_eq!(val, "1"),
                        _ => panic!("Expected Literal for increment value"),
                    }
                },
                _ => panic!("Increment is not an Assignment"),
            }
            let body_nodes: &Vec<ASTNode> = &(*body);
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
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::WhileLoop { condition, body } => {
            match condition.get_element() {
                SyntaxElement::Literal(DataType::Boolean, val) => {
                    assert_eq!(val, "true");
                },
                _ => panic!("Expected Boolean Literal in condition"),
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
    let tokens = vec![
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
    let ast = Parser::parse(tokens).expect("Failed to parse");
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
                SyntaxElement::Literal(DataType::Boolean, val) => {
                    assert_eq!(val, "true");
                },
                _ => panic!("Expected Boolean Literal in condition"),
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
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::Continue => {  },
        _ => panic!("Expected ContinueStatement"),
    }
}

#[test]
fn test_match_statement_parsing() {
    let tokens = vec![
        Token::MATCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['x']),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::MatchStatement { to_match, arms } => {
            match to_match.get_element() {
                SyntaxElement::Variable(name) => assert_eq!(name, "x"),
                _ => panic!("Expected Variable in to_match"),
            }
            assert!(arms.is_empty(), "Expected no match arms");
        },
        _ => panic!("Expected MatchStatement"),
    }
}


#[test]
fn test_function_call_parsing() {
    let tokens = vec![
        Token::IDENTIFIER(vec!['f', 'o', 'o', 'o']),
        Token::LPAREN,
        Token::IDENTIFIER(vec!['a']),
        Token::COMMA,
        Token::IDENTIFIER(vec!['b']),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    match ast.get_root().get_children().first().unwrap().get_element() {
        SyntaxElement::FunctionCall { name, arguments } => {
            assert_eq!(name, "func");
            assert_eq!(arguments.len(), 2, "Expected two arguments");

            match arguments[0].get_element() {
                SyntaxElement::Variable(arg_name) => assert_eq!(arg_name, "a"),
                _ => panic!("Expected first argument to be a variable 'a'"),
            }
            match arguments[1].get_element() {
                SyntaxElement::Variable(arg_name) => assert_eq!(arg_name, "b"),
                _ => panic!("Expected second argument to be a variable 'b'"),
            }
        },
        _ => panic!("Expected FunctionCall"),
    }
}
