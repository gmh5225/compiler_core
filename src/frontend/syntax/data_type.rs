/*
Defines acceptable data types
 */

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
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
            }
        }
    }
}

