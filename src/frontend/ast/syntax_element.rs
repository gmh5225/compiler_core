/*
Defines acceptable syntax elements, as a part of an AST
 */

use crate::frontend::ast::{ 
    data_type::DataType, 
    ast_struct::ASTNode, 
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    name: String,
    data_type: DataType,
}
impl FunctionParameter {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name,
            data_type,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
}

impl fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.data_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    variant: ASTNode,
    action: ASTNode,
}

impl MatchArm {
    pub fn new(variant: ASTNode, action: ASTNode) -> Self {
        Self {
            variant,
            action
        }
    }

    pub fn get_variant(&self) -> ASTNode {
        self.variant.clone()
    }

    pub fn get_action(&self) -> ASTNode {
        self.action.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxElement {
    ModuleExpression,
    TopLevelExpression,
    Literal(DataType, String), // this is for literal things, eg: Boolean, "true"
    Variable(DataType, String), // this is for variable names and their types, eg: Boolean, "foo"
    BinaryExpression {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Box<Vec<ASTNode>>,
        else_branch: Option<Box<Vec<ASTNode>>>,
    }, 
    Assignment {
        variable: String,
        value: Box<ASTNode>,
    },
    Initialization {
        variable: String,
        data_type: DataType,
        value: Box<ASTNode>
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<FunctionParameter>,
        return_type: Option<DataType>,
    },
    ForLoop {
        initializer: Option<Box<ASTNode>>,
        condition: Box<ASTNode>,
        increment: Option<Box<ASTNode>>,
        body: Box<Vec<ASTNode>>,
    },
    WhileLoop {
        condition: Box<ASTNode>,
        body: Box<Vec<ASTNode>>,
    },
    DoWhileLoop {
        body: Box<Vec<ASTNode>>,
        condition: Box<ASTNode>,
    },
    Break,
    Continue,
    MatchStatement {
        to_match: Box<ASTNode>,
        arms: Vec<MatchArm>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>,
    },
    StructDeclaration {
        name: String,
        fields: Vec<(String, DataType)>
    },
    EnumDeclaration {
        name: String,
        variants: Vec<String>,
    },
    UnaryExpression { // for things like !x (operator and one operand)
        operator: String,
        operand: Box<ASTNode>,
    },
    Return {
        value: Box<ASTNode>,
    }
}

impl fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxElement::ModuleExpression => write!(f, "ModuleExpression"),
            SyntaxElement::TopLevelExpression => write!(f, "TopLevelExpression"),
            SyntaxElement::Literal(data_type, value) => write!(f, "Literal({:?}, {})", data_type, value),
            SyntaxElement::Variable(data_type, name) => write!(f, "Name of var({}), DataType: ({})", name, data_type),
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
            SyntaxElement::Initialization { variable, data_type, value } => 
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