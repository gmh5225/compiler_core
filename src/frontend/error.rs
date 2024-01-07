#[derive(Debug)]
pub enum ErrorType {    
    /// Binary operation has incompatible types
    TypeMismatch {
        operation: String,
        left_type: String,
        right_type: String,
    },

    /// Variable used but not declared
    UndefinedVariable {
        variable_name: String,
    },

    /// Unsupported operator on types given
    UnsupportedOperator {
        operator: String,
        operand_type: String,
    },

    /// Divisor is zero
    DivisionByZero {
        operation: String,
    },

    /// Invalid assignment to a target 
    InvalidAssignment {
        target: String,
    },
}
