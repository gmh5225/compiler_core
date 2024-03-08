/*
Defines acceptable syntax elements, as a part of an AST
 */

use crate::frontend::ast::{ 
    data_type::DataType, 
    ast_struct::ASTNode, 
};
use std::fmt;

/// Function parameter in an ast
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionParameter {
    name: String,
    data_type: DataType,
}
impl FunctionParameter {
    /// Creates a new function parameter
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name,
            data_type,
        }
    }

    /// Retrieves the function parameter's name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Retrieves the function parametr's data type
    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
}

impl fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.data_type)
    }
}

/// Match arm in an ast
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {
    variant: ASTNode,
    action: ASTNode,
}

impl MatchArm {
    /// Creates a new match arm
    pub fn new(variant: ASTNode, action: ASTNode) -> Self {
        Self {
            variant,
            action
        }
    }

    /// Retrieves the variant type of the match arm
    pub fn get_variant(&self) -> ASTNode {
        self.variant.clone()
    }

    /// Retrieves the action based on the variant
    pub fn get_action(&self) -> ASTNode {
        self.action.clone()
    }
}

/// Syntax element, an aspect of ASTNode's that make up an AST
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub enum SyntaxElement {
    /// No expression
    #[default]
    NoExpression,

    /// Module expression
    ModuleExpression,

    /// Top level expression
    TopLevelExpression,

    /// Literal
    Literal {
        /// Data type of the literal
        data_type: DataType, 
        /// Value of the literal
        value: String,
    }, 

    /// Variable
    Variable {
        /// Data type of the liberal
        data_type: DataType,
        /// Name of the variable
        name: String,
    }, 

    /// Binary expression
    BinaryExpression {
        /// Left hand side
        left: Box<ASTNode>,
        /// Operator
        operator: String,
        /// Right hand side
        right: Box<ASTNode>,
    },

    /// If statement
    IfStatement {
        /// Condition of the if statement
        condition: Box<ASTNode>,
        /// If true, then
        then_branch: Box<Vec<ASTNode>>,
        /// Else
        else_branch: Option<Box<Vec<ASTNode>>>,
    }, 

    /// Assignment of an existing variable
    Assignment {
        /// Variable name
        variable: String,
        /// Value to assign
        value: Box<ASTNode>,
    },

    /// Initialization of a variable
    Initialization {
        /// Variable name
        variable: String,
        /// Data type of the variable
        data_type: DataType,
        /// Initial value of the variable
        value: Box<ASTNode>
    },

    /// Function declaration
    FunctionDeclaration {
        /// Name of the function
        name: String,
        /// Parameters of the function
        parameters: Vec<FunctionParameter>,
        /// Return type of the function
        return_type: Option<DataType>,
    },

    /// For loop
    ForLoop {
        /// Initializer for the for loop
        initializer: Option<Box<ASTNode>>,
        /// Condition of the for loop
        condition: Box<ASTNode>,
        /// Increment of variable declared in the for loop definition
        increment: Option<Box<ASTNode>>,
        /// Body of the for loop
        body: Box<Vec<ASTNode>>,
    },

    /// While loop
    WhileLoop {
        /// Condition
        condition: Box<ASTNode>,
        /// Body
        body: Box<Vec<ASTNode>>,
    },

    /// Do while loop
    DoWhileLoop {
        /// Body 
        body: Box<Vec<ASTNode>>,
        /// Condition
        condition: Box<ASTNode>,
    },

    /// Break statement
    Break,

    /// Continue statement
    Continue,

    /// Match statement
    MatchStatement {
        /// Value to be matched
        to_match: Box<ASTNode>,
        /// Match arms
        arms: Vec<MatchArm>,
    },

    /// Function call
    FunctionCall {
        /// Name of function
        name: String,
        /// Aguments to function
        arguments: Vec<ASTNode>,
    },

    /// Struct declaration
    StructDeclaration {
        /// Name of struct
        name: String,
        /// Fields of struct
        fields: Vec<(String, DataType)> // change this to a hashmap?
    },

    /// Enum declaration
    EnumDeclaration {
        /// Name of enum
        name: String,
        /// Enum variants
        variants: Vec<String>,
    },

    /// Unary expression
    UnaryExpression { 
        /// Unary operator
        operator: String,
        /// Operand
        operand: Box<ASTNode>,
    },

    /// Return statement
    Return {
        /// Value of return
        value: Box<ASTNode>,
    }
}

impl fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxElement::NoExpression => panic!("No expression"),
            SyntaxElement::ModuleExpression => write!(f, "ModuleExpression"),
            SyntaxElement::TopLevelExpression => write!(f, "TopLevelExpression"),
            SyntaxElement::Literal{data_type, value} => write!(f, "Literal({:?}, {})", data_type, value),
            SyntaxElement::Variable{data_type, name } => write!(f, "Name of var({}), DataType: ({})", name, data_type),
            SyntaxElement::BinaryExpression { left, operator, right } => 
                write!(f, "BinaryExpression({}, {}, {})", left, operator, right),
                SyntaxElement::IfStatement { condition, then_branch, else_branch } => {
                    write!(f, "IfStatement({}, [", condition)?;
                    for (i, item) in then_branch.iter().enumerate() {
                        if i > 0 { write!(f, ", ")?; } 
                        write!(f, "{}", item)?;
                    }
                    write!(f, "]")?;
    
                    if let Some(else_branch) = else_branch {
                        write!(f, ", [")?;
                        for (i, item) in else_branch.iter().enumerate() {
                            if i > 0 { write!(f, ", ")?; }
                            write!(f, "{}", item)?;
                        }
                        write!(f, "]")?;
                    } else {
                        write!(f, ", None")?;
                    }
                    
                    Ok(())
                },
            SyntaxElement::Assignment { variable, value } => 
                write!(f, "Assignment({}, {})", variable, value),
            SyntaxElement::Initialization { variable, data_type: _, value } => 
                write!(f, "Initialization({}, {})", variable, value),
            SyntaxElement::FunctionDeclaration { name, parameters, return_type } => {
                let return_type_str = match return_type {
                    Some(rt) => rt.to_string(),
                    None => "None".to_string(),
                };
                write!(f, "FunctionDeclaration(name: {}, parameters: {:?}, return_type: {})", name, parameters, return_type_str)
            },
            SyntaxElement::ForLoop { initializer, condition, increment, body } => {
                write!(f, "ForLoop(")?;
                if let Some(init) = initializer {
                    write!(f, "initializer: {}, ", init)?;
                } else {
                    write!(f, "initializer: None, ")?;
                }
                write!(f, "condition: {}, ", condition)?;
                if let Some(inc) = increment {
                    write!(f, "increment: {}, ", inc)?;
                } else {
                    write!(f, "increment: None, ")?;
                }
                write!(f, "body: [")?;
                for (i, node) in body.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", node)?;
                }
                write!(f, "])")
            },
            SyntaxElement::WhileLoop { condition, body } => {
                write!(f, "WhileLoop(condition: {}, body: [", condition)?;
                for (i, node) in body.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", node)?;
                }
                write!(f, "])")
            },
            SyntaxElement::DoWhileLoop { body, condition } => {
                write!(f, "DoWhileLoop(body: [")?;
                for (i, node) in body.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", node)?;
                }
                write!(f, "], condition: {})", condition)
            },
            SyntaxElement::Break => write!(f, "BreakStatement"),
            SyntaxElement::Continue => write!(f, "ContinueStatement"),
            SyntaxElement::MatchStatement { to_match, arms } => 
                write!(f, "MatchStatement(to_match: {}, arms: {:?})", to_match, arms),
            SyntaxElement::FunctionCall { name, arguments } => 
                write!(f, "FunctionCall(name: {}, arguments: {:?})", name, arguments),
            SyntaxElement::StructDeclaration { name, fields } => 
                write!(f, "StructDeclaration(name: {}, fields: {:?})", name, fields),
            SyntaxElement::EnumDeclaration { name, variants } => 
                write!(f, "EnumDeclaration(name: {}, variants: {:?})", name, variants),
            SyntaxElement::UnaryExpression { operator, operand } => 
                write!(f, "UnaryExpression(operator: {}, operand: {})", operator, operand),
            SyntaxElement::Return { value} => 
                write!(f, "Return(value: {}),", value),
        }
    }
}