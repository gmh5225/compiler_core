use compiler_core::frontend::{
    ast::{
        ast_struct::*,
        data_type::*,
        syntax_element::*,
    },
    lexer::token::*,
    parser::parser_core::*,
};

/// --- UTILITIES SECTION --- ///
/// cargo test --test parser_tests
/// panic!("{:?}", self.get_input().get(self.get_current()));

/// --- BASELINE SECTION --- ///

#[test]
fn test_empty_input() { 
    let tokens: Vec<Token> = vec![];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_element(), SyntaxElement::TopLevelExpression);
    assert!(ast.get_root().get_children().is_empty());
}

/// --- TOP LEVEL EXPRESSION SECTION --- ///

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

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);

    let identifier_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("my_func".to_string()));
    let block_expression_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

    function_declaration_node.add_child(identifier_node);
    function_declaration_node.add_child(block_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_declaration_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
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

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let identifier_node = ASTNode::new(SyntaxElement::Identifier("calculate".to_string()));
    function_declaration_node.add_child(identifier_node);

    let mut parameter_x_node = ASTNode::new(SyntaxElement::Parameter);
    parameter_x_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    parameter_x_node.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut parameter_y_node = ASTNode::new(SyntaxElement::Parameter);
    parameter_y_node.add_child(ASTNode::new(SyntaxElement::Identifier("y".to_string())));
    parameter_y_node.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    function_declaration_node.add_child(parameter_x_node);
    function_declaration_node.add_child(parameter_y_node);

    let return_type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Boolean));
    function_declaration_node.add_child(return_type_node);

    let block_expression_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

    function_declaration_node.add_child(block_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_declaration_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
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

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut function_declaration_node: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let identifier_node = ASTNode::new(SyntaxElement::Identifier("test".to_string()));
    function_declaration_node.add_child(identifier_node);

    let mut block_expression_node = ASTNode::new(SyntaxElement::BlockExpression);
    let mut initialization_node = ASTNode::new(SyntaxElement::Initialization);
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    let mut variable_node: ASTNode = ASTNode::new(SyntaxElement::Variable);

    let variable_id_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    let type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    let value_node: ASTNode = ASTNode::new(SyntaxElement::Literal("1".to_string()));

    variable_node.add_child(variable_id_node);
    variable_node.add_child(type_node);

    assigned_value_node.add_child(value_node);

    initialization_node.add_child(variable_node);
    initialization_node.add_child(assigned_value_node);

    block_expression_node.add_child(initialization_node);

    function_declaration_node.add_child(block_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_declaration_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_function_with_if_else_statement() {
    let tokens: Vec<Token> = vec![
        Token::FUNCTION,
        Token::IDENTIFIER(vec!['f', 'o', 'o']),
        Token::LPAREN,
        Token::IDENTIFIER(vec!['a']),
        Token::COLON,
        Token::TINTEGER,
        Token::COMMA,
        Token::IDENTIFIER(vec!['b']),
        Token::COLON,
        Token::TINTEGER,
        Token::RPAREN,
        Token::COLON,
        Token::TBOOLEAN,
        Token::LBRACKET,
        Token::IF,
        Token::LPAREN,
        Token::FALSE,
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::FALSE,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::ELSE,
        Token::LBRACKET,
        Token::RETURN,
        Token::TRUE,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let identifier_node = ASTNode::new(SyntaxElement::Identifier("foo".to_string()));
    function_declaration_node.add_child(identifier_node);

    let mut parameter_a_node = ASTNode::new(SyntaxElement::Parameter);
    parameter_a_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
    parameter_a_node.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));
    function_declaration_node.add_child(parameter_a_node);

    let mut parameter_b_node = ASTNode::new(SyntaxElement::Parameter);
    parameter_b_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));
    parameter_b_node.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));
    function_declaration_node.add_child(parameter_b_node);

    let return_type_node = ASTNode::new(SyntaxElement::Type(DataType::Boolean));
    function_declaration_node.add_child(return_type_node);

    let mut if_statement_node = ASTNode::new(SyntaxElement::IfStatement);
    let condition_node = ASTNode::new(SyntaxElement::Literal("false".to_string() ));

    let mut then_branch_node = ASTNode::new(SyntaxElement::BlockExpression);
    let mut then_return_node = ASTNode::new(SyntaxElement::Return);
    let mut then_return_value = ASTNode::new(SyntaxElement::AssignedValue);
    let then_return_value_node = ASTNode::new(SyntaxElement::Literal("false".to_string()));
    then_return_value.add_child(then_return_value_node);

    then_return_node.add_child(then_return_value);
    then_branch_node.add_child(then_return_node);

    let mut else_branch_node = ASTNode::new(SyntaxElement::BlockExpression);
    let mut else_return_node = ASTNode::new(SyntaxElement::Return);
    let mut else_return_value: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    let else_return_value_node = ASTNode::new(SyntaxElement::Literal("true".to_string() ));
    else_return_value.add_child(else_return_value_node);

    else_return_node.add_child(else_return_value);
    else_branch_node.add_child(else_return_node);

    if_statement_node.add_child(condition_node);
    if_statement_node.add_child(then_branch_node);
    if_statement_node.add_child(else_branch_node);

    let mut block_expression_node = ASTNode::new(SyntaxElement::BlockExpression);
    block_expression_node.add_child(if_statement_node);

    function_declaration_node.add_child(block_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_declaration_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}


/// --- BLOCK EXPRESSION SECTION --- ///

#[test]
fn test_for_loop_parsing() {
    let tokens: Vec<Token> = vec![
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
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut for_loop_node: ASTNode = ASTNode::new(SyntaxElement::ForLoop);

    let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let condition_value_node: ASTNode = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    condition_node.add_child(condition_value_node);

    let mut body_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let break_node: ASTNode = ASTNode::new(SyntaxElement::Break);

    body_node.add_child(break_node);

    for_loop_node.add_child(condition_node);
    for_loop_node.add_child(body_node);

    let mut top_level_expr: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(for_loop_node);

    let expected_ast: AST = AST::new(top_level_expr);
    assert_eq!(ast, expected_ast);
}

#[test]
fn test_while_loop_parsing() {
    let tokens: Vec<Token> = vec![
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
    let actual_ast = Parser::parse(tokens).expect("Failed to parse");

    let mut while_loop_node = ASTNode::new(SyntaxElement::WhileLoop);
    
    let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let condition_value_node = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    condition_node.add_child(condition_value_node);

    let mut body_node = ASTNode::new(SyntaxElement::BlockExpression);
    let break_node = ASTNode::new(SyntaxElement::Break);

    body_node.add_child(break_node);

    while_loop_node.add_child(condition_node); 
    while_loop_node.add_child(body_node); 

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(while_loop_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(actual_ast, expected_ast);
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
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut do_while_loop_node = ASTNode::new(SyntaxElement::DoWhileLoop);
    
    let mut body_node = ASTNode::new(SyntaxElement::BlockExpression);
    let break_node = ASTNode::new(SyntaxElement::Break);
    body_node.add_child(break_node);

    let mut condition_node = ASTNode::new(SyntaxElement::Condition);
    let condition_value_node = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    condition_node.add_child(condition_value_node);

    do_while_loop_node.add_child(body_node); 
    do_while_loop_node.add_child(condition_node); 

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(do_while_loop_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast)
}

/// --- STATEMENT SECTION --- ///

#[test]
fn test_if_statement_parsing() {
    let tokens: Vec<Token> = vec![
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

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut if_statement_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let condition_node: ASTNode = ASTNode::new(SyntaxElement::Literal("true".to_string()));

    let mut then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(SyntaxElement::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    then_branch_node.add_child(return_node);

    if_statement_node.add_child(condition_node);
    if_statement_node.add_child(then_branch_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(if_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
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
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut initialization_node: ASTNode = ASTNode::new(SyntaxElement::Initialization);

    let var_id_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    let type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Boolean));

    let mut variable_node: ASTNode = ASTNode::new(SyntaxElement::Variable);
    variable_node.add_child(var_id_node);
    variable_node.add_child(type_node);

    let value_node: ASTNode = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    assigned_value_node.add_child(value_node);

    initialization_node.add_child(variable_node);
    initialization_node.add_child(assigned_value_node);

    let mut top_level_expr: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(initialization_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_continue_statement_parsing() {
    let tokens = vec![
        Token::CONTINUE,
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast = Parser::parse(tokens).expect("Failed to parse");

    let continue_node = ASTNode::new(SyntaxElement::Continue);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(continue_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
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
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut match_statement_node: ASTNode = ASTNode::new(SyntaxElement::MatchStatement);
    let variable_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    let arms_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression); 

    match_statement_node.add_child(variable_node);
    match_statement_node.add_child(arms_node);

    let mut top_level_expr: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(match_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_ast_print() {
    let tokens = vec![
        Token::MATCH,
        Token::IDENTIFIER(vec!['x']),
        Token::LBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");
    println!("Printing parsed AST:");
    println!("{}", ast);
    println!("Printing parsed AST again:");
    println!("{}", ast);

    let mut match_statement_node: ASTNode = ASTNode::new(SyntaxElement::MatchStatement);
    let variable_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    let arms_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression); 

    match_statement_node.add_child(variable_node);
    match_statement_node.add_child(arms_node);

    let mut top_level_expr: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(match_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);
    println!("Printing expected AST:");
    println!("{}", expected_ast);
    assert_eq!(ast.to_string(),"AST: \nTopLevelExpression\n\tMatchStatement\n\t\tIdentifier(x)\n\t\tBlockExpression\n")
}