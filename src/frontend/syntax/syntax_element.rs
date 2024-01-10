/*
Defines acceptable syntax elements, as a part of an AST
 */

use crate::frontend::syntax::{ data_type::DataType, 
                               ast::ASTNode, };
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    name: String,
    data_type: DataType,
}

impl fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.data_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxElement {
    ModuleExpression,
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
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<FunctionParameter>,
        return_type: Option<DataType>,
    },
}

impl fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxElement::ModuleExpression => write!(f, "ModuleExpression"),
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
                write!(f, "Initialization({}, {})", variable, value),
                SyntaxElement::FunctionDeclaration { name, parameters, return_type } => {
                    let return_type_str = match return_type {
                        Some(rt) => rt.to_string(),
                        None => "None".to_string(),
                    };
                    write!(f, "FunctionDeclaration(name: {}, parameters: {:?}, return_type: {})", name, parameters, return_type_str)
                }
        }
    }
}