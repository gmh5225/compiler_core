use crate::frontend::syntax::{ data_type::DataType, 
                               ast::ASTNode, };
#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxElement {
    FileExpression,
    Literal(DataType, String),
    Variable(String),
    BinaryExpression {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    }, 
    Assignment {
        variable: String,
        value: Box<ASTNode>,
    },
    Initialization {
        variable: String,
        value: Box<ASTNode>
    }
}