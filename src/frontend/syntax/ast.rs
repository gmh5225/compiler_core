/*
Represents an abstract syntax tree
 */

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