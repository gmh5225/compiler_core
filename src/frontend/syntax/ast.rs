extern crate llvm_sys as llvm;

// enum ASTNodeType {
//     NumericalExpr(NumExpr),
// }

// struct NumExpr {
//     val: f64,
// }

// impl NumExpr {
//     fn new(val : f64) -> Self {
//         NumExpr { val }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
}

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
        else_branch: Box<ASTNode>,
    }, 
    Assignment {
        variable: String,
        value: Box<ASTNode>,
    },
}

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