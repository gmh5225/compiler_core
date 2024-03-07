/*
Defines acceptable data types
 */

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    String,
    Function,
    Unknown,
    None,
    Struct,
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

