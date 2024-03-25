use crate::frontend::ast::data_type::DataType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
/// Defines acceptable syntax elements, as part of an AST
pub enum SyntaxElement {
    /// No expression
    #[default]
    NoExpression,

    /// --- BASE EXPRESSION SECTION --- ///
    /// Literal
    Literal {
        /// Value of the literal
        value: String,
    },

    /// Mutable Literal
    MutLiteral {
        /// Value of the literal
        value: String,
    },

    /// Variable
    Variable {
        /// Mutability
        is_mutable: bool,
    },

    /// Binary expression
    BinaryExpression,

    /// Unary expression
    UnaryExpression,

    /// Identifier
    Identifier(String),

    /// Function call
    FunctionCall,

    /// Operator
    Operator(&'static str),

    /// Operand
    Operand,

    /// Type
    Type(DataType),


    /// --- CONTROL FLOW SECTION --- ///
    /// If statement
    IfStatement,

    /// Else if statement
    ElifStatement,

    /// Else statement
    ElseStatement,

    /// For loop
    ForLoop,

    /// While loop
    WhileLoop,

    /// Do while loop
    DoWhileLoop,

    /// Break statement
    Break,

    /// Continue statement
    Continue,

    /// Match statement
    MatchStatement,

    /// Match arm
    MatchArm,

    /// Return statement
    Return,


    /// --- DECLARATION SECTION --- ///
    /// Assignment of an existing variable
    Assignment,

    /// Initialization of a variable
    Initialization,

    /// Function declaration
    FunctionDeclaration,

    /// Struct declaration
    StructDeclaration,

    /// Enum declaration
    EnumDeclaration,


    /// --- MODULE & SCOPING SECTION --- ///
    /// Module expression
    ModuleExpression,

    /// Top level expression
    TopLevelExpression,

    /// Block
    BlockExpression,

    /// --- LOOP CONTROL SECTION --- ///
    /// Initializer to a loop
    LoopInitializer,

    /// Incrementer on a loop
    LoopIncrement,


    /// --- MISC SECTION --- ///
    /// Condition,
    Condition,

    /// Action,
    Action,

    /// Variant,
    Variant,

    /// Assigned value (used in initialization, assignment, and return)
    AssignedValue,

    /// Field of a struct
    Field,

    /// Parameter
    Parameter,
}


impl fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxElement::NoExpression => write!(f, "NoExpression"),
            SyntaxElement::Literal { value } => write!(f, "Literal({})", value),
            SyntaxElement::MutLiteral { value } => write!(f, "MutLiteral({})", value),
            SyntaxElement::Variable { is_mutable } => write!(f, "Variable(is_mutable: {})", is_mutable),
            SyntaxElement::BinaryExpression => write!(f, "BinaryExpression"),
            SyntaxElement::UnaryExpression => write!(f, "UnaryExpression"),
            SyntaxElement::Identifier(id) => write!(f, "Identifier({})", id),
            SyntaxElement::FunctionCall => write!(f, "FunctionCall"),
            SyntaxElement::Operator(op) => write!(f, "Operator({})", op),
            SyntaxElement::Operand => write!(f, "Operand"),
            SyntaxElement::Type(data_type) => write!(f, "Type({})", data_type),
            SyntaxElement::IfStatement => write!(f, "IfStatement"),
            SyntaxElement::ElifStatement => write!(f, "ElifStatement"),
            SyntaxElement::ElseStatement => write!(f, "ElseStatement"),
            SyntaxElement::ForLoop => write!(f, "ForLoop"),
            SyntaxElement::WhileLoop => write!(f, "WhileLoop"),
            SyntaxElement::DoWhileLoop => write!(f, "DoWhileLoop"),
            SyntaxElement::Break => write!(f, "Break"),
            SyntaxElement::Continue => write!(f, "Continue"),
            SyntaxElement::MatchStatement => write!(f, "MatchStatement"),
            SyntaxElement::Return => write!(f, "Return"),
            SyntaxElement::Assignment => write!(f, "Assignment"),
            SyntaxElement::Initialization => write!(f, "Initialization"),
            SyntaxElement::FunctionDeclaration => write!(f, "FunctionDeclaration"),
            SyntaxElement::Parameter => write!(f, "Parameter"),
            SyntaxElement::StructDeclaration => write!(f, "StructDeclaration"),
            SyntaxElement::EnumDeclaration => write!(f, "EnumDeclaration"),
            SyntaxElement::ModuleExpression => write!(f, "ModuleExpression"),
            SyntaxElement::TopLevelExpression => write!(f, "TopLevelExpression"),
            SyntaxElement::BlockExpression => write!(f, "BlockExpression"),
            SyntaxElement::LoopInitializer => write!(f, "LoopInitializer"),
            SyntaxElement::LoopIncrement => write!(f, "LoopIncrement"),
            SyntaxElement::Condition => write!(f, "Condition"),
            SyntaxElement::Action => write!(f, "Action"),
            SyntaxElement::Variant => write!(f, "Variant"),
            SyntaxElement::AssignedValue => write!(f, "AssignedValue"),
            SyntaxElement::Field => write!(f, "Field"),
            SyntaxElement::MatchArm => write!(f, "MatchArm"),
        }
    }
}