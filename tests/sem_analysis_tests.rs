
use compiler_design::frontend::{ 
    ast::ast_struct::{ AST, ASTNode }, 
    ast::syntax_element::SyntaxElement, 
    ast::data_type::DataType,
    analysis::sem_analysis::*,
    utils::error::*,
};
#[test]
fn basic_test() {
    let left_node = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "5".to_string()));
    let right_node = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "0".to_string()));

    let division_expr = ASTNode::new(SyntaxElement::BinaryExpression {
        left: Box::new(left_node),
        operator: "/".to_string(),
        right: Box::new(right_node),
    });

    let mut root_node = ASTNode::new(SyntaxElement::ModuleExpression);
    root_node.add_child(division_expr);

    let ast = AST::new(root_node);

    let errors = SemAnalysis::sem_analysis(ast);

    assert!(errors.iter().any(|e| matches!(e, ErrorType::DivisionByZero { .. })),
            "Expected DivisionByZero error, but found {:?}", errors);
}
