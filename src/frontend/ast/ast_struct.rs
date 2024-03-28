/*
Represents an abstract syntax tree
 */

use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Eq, PartialEq};
use std::sync::{Arc, Mutex};

use std::fmt;
use crate::frontend::{
    ast::syntax_element::SyntaxElement, 
    symbol_table::symbol_table_struct::SymbolTableStack
};

/// A Module TODO rename this to Module
pub struct Module {
    children: BinaryHeap<ModElement>,
}

impl Module {
    /// A new module
    pub fn new() -> Self {
        Module {
            children: BinaryHeap::new(),
        }
    }

    /// Adds a child to the module
    pub fn add_child(&mut self, child: ModElement) {
        self.children.push(child);
    }

    /// Retrieves the child with the highest priority
    pub fn get_child(&mut self) -> Option<ModElement> {
        self.children.pop()
    }

    /// Retrieves all children
    pub fn get_children(&mut self) -> &mut BinaryHeap<ModElement> {
        &mut self.children
    }
}

/// An element of a module
#[derive(Clone)]
pub struct ModElement {
    ast: AST,
    sym_table_stack: Arc<Mutex<SymbolTableStack>>,
    priority: i32,
}

impl ModElement {
    /// Creates a new module element
    pub fn new(ast: AST, sym_table_stack: Arc<Mutex<SymbolTableStack>>, priority: i32) -> Self {
        Self {
            ast,
            sym_table_stack,
            priority,
        }
    }
    
    /// Retrieves the ast of the mod element
    pub fn get_ast(&self) -> AST {
        self.ast.clone()
    }

    /// Retrieves the symbol table stack of the mod element
    pub fn get_sym_table_stack(&self) -> Arc<Mutex<SymbolTableStack>> {
        Arc::clone(&self.sym_table_stack)
    }

    /// Retrieves the priority of the mod element
    pub fn set_priority(&mut self, new: i32) {
        self.priority = new;
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

/// An Abstract Syntax Tree
#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    root: ASTNode,
}

/// A node of an abstract syntax tree
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct ASTNode {
    element: SyntaxElement,
    children: Vec<ASTNode>, 
}

impl AST {
    /// Create a new abstract syntax tree
    pub fn new(root: ASTNode) -> Self {
        AST { 
            root 
        }
    }

    /// Retrieves the root of the abstract syntax tree
    pub fn get_root(&self) -> ASTNode {
        self.root.clone()
    }

}

impl ASTNode {
    /// Creates a new ast node
    pub fn new(element: SyntaxElement) -> Self {
        ASTNode {
            element,
            children: Vec::new(),
        }
    }

    /// Gets the syntax element of the node
    pub fn get_element(&self) -> SyntaxElement {
        self.element.clone()
    }

    /// Gets the children of the node
    pub fn get_children(&self) -> Vec<ASTNode> {
        self.children.clone()
    }

    /// Adds a child node
    pub fn add_child(&mut self, to_add: ASTNode) {
        self.children.push(to_add);
    }

    /// Adds children nodes
    pub fn add_children(&mut self, to_add: Vec<ASTNode>) {
        self.children.extend(to_add);
    }
    pub fn is_return(&self) -> bool {
        // match &self.element {
        //     SyntaxElement::Return { value: _ } => true,
        //     _ => false,
        // }
        true
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ASTNode: {}", self.element)
    }
}