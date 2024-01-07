extern crate llvm_sys as llvm;
use std::fmt;
use crate::frontend::syntax::{ syntax_element::SyntaxElement,
                               data_type::DataType, };

#[derive(Debug, PartialEq)] // may need to implement partialeq or eq at some point
pub struct AST {
    pub root: ASTNode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTNode {
    pub element: SyntaxElement,
    pub children: Vec<ASTNode>,
}

impl AST {
    pub fn new(root: ASTNode) -> Self {
        AST { 
            root 
        }
    }
}

impl ASTNode {
    pub fn new(element: SyntaxElement) -> Self {
        ASTNode {
            element,
            children: Vec::new(),
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ASTNode: {}", self.element)
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