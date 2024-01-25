/*
Represents an abstract syntax tree
 */

use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Eq, PartialEq};
use std::sync::{Arc, Mutex};

use std::fmt;
use crate::frontend::{
    ast::syntax_element::SyntaxElement, 
    ast_pass::symbol_table::SymbolTableStack
};

pub struct ModAST {
    children: BinaryHeap<ModElement>,
}

impl ModAST {
    pub fn new() -> Self {
        ModAST {
            children: BinaryHeap::new(),
        }
    }

    pub fn add_child(&mut self, child: ModElement) {
        self.children.push(child);
    }

    pub fn get_child(&mut self) -> Option<ModElement> {
        self.children.pop()
    }

    pub fn get_children(&mut self) -> &mut BinaryHeap<ModElement> {
        &mut self.children
    }
}

#[derive(Clone)]
pub struct ModElement {
    ast: AST,
    sym_table_stack: Arc<Mutex<SymbolTableStack>>,
    priority: i32,
}

impl ModElement {
    pub fn new(ast: AST, sym_table_stack: Arc<Mutex<SymbolTableStack>>, priority: i32) -> Self {
        Self {
            ast,
            sym_table_stack,
            priority,
        }
    }
    pub fn get_ast(&self) -> AST {
        self.ast.clone()
    }
    pub fn get_sym_table_stack(&self) -> Arc<Mutex<SymbolTableStack>> {
        Arc::clone(&self.sym_table_stack)
    }
}

impl Ord for ModElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for ModElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ModElement {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for ModElement {} 

#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    root: ASTNode,
}

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
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
    pub fn is_return(&self) -> bool {
        match &self.element {
            SyntaxElement::Return { value: _ } => true,
            _ => false,
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ASTNode: {}", self.element)
    }
}