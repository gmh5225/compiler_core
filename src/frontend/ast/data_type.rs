use std::fmt;

/// Acceptable data types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Integer type
    Integer,
    /// Float type
    Float,
    /// Boolean type
    Boolean,
    /// String type
    String,
    /// Function type
    Function,
    /// Unknown type
    Unknown,
    /// No type
    None,
    /// Struct type
    Struct,
    /// Enum type
    Enum,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer => {
                write!(f, "Integer")
            },
            DataType::Float => {
                write!(f, "Float")
            },
            DataType::Boolean => {
                write!(f, "Boolean")
            },
            DataType::String => {
                write!(f, "String")
            },
            DataType::Unknown => {
                write!(f, "Unknown")
            },
            DataType::Function => {
                write!(f, "Function")
            },
            DataType::Struct => {
                write!(f, "Struct")
            },
            DataType::Enum => {
                write!(f, "Enum")
            },
            DataType::None => {
                write!(f, "None")
            }
        }
    }
}

