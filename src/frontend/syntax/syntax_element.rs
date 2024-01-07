/*
Defines acceptable syntax elements, as a part of an AST
 */

use crate::frontend::syntax::{ data_type::DataType, 
                               ast::ASTNode, };
use std::fmt;

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

impl fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxElement::FileExpression => write!(f, "FileExpression"),
            SyntaxElement::Literal(data_type, value) => write!(f, "Literal({:?}, {})", data_type, value),
            SyntaxElement::Variable(name) => write!(f, "Variable({})", name),
            SyntaxElement::BinaryExpression { left, operator, right } => 
                write!(f, "BinaryExpression({}, {}, {})", left, operator, right),
            SyntaxElement::IfStatement { condition, then_branch, else_branch } => {
                write!(f, "IfStatement({}, {}, ", condition, then_branch)?;
                if let Some(else_branch) = else_branch {
                    write!(f, "{}", else_branch)?;
                } else {
                    write!(f, "None")?;
                }
                Ok(())
            },
            SyntaxElement::Assignment { variable, value } => 
                write!(f, "Assignment({}, {})", variable, value),
            SyntaxElement::Initialization { variable, value } => 
                write!(f, "Assignment({}, {})", variable, value),
        }
    }
}