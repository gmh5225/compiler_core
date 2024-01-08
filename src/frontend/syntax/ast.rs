/*
Represents an abstract syntax tree
 */

use std::fmt;
use crate::frontend::syntax::syntax_element::SyntaxElement;

#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    root: ASTNode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTNode {
    element: SyntaxElement,
    children: Vec<ASTNode>,
}

impl AST {
    pub fn new(root: ASTNode) -> Self {
        AST { 
            root 
        }
    }
    pub fn get_root(&self) -> ASTNode {
        self.root.clone()
    }
}

impl ASTNode {
    pub fn new(element: SyntaxElement) -> Self {
        ASTNode {
            element,
            children: Vec::new(),
        }
    }
    pub fn get_element(&self) -> SyntaxElement {
        self.element.clone()
    }
    pub fn get_children(&self) -> Vec<ASTNode> {
        self.children.clone()
    }
    pub fn add_child(&mut self, to_add: ASTNode) {
        self.children.push(to_add);
    }
    pub fn add_children(&mut self, to_add: Vec<ASTNode>) {
        self.children.extend(to_add);
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ASTNode: {}", self.element)
    }
}