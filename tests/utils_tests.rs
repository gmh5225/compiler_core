use compiler_core::frontend::{
    ast::{
        ast_struct::AST,
        ast_struct::ASTNode,
        syntax_element::SyntaxElement,
    },
    lexer::token::Token,
    parser::parser_core::Parser,
};

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