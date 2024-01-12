/*
Defines acceptable tokens in the program
 */

#[derive(PartialEq, Debug)]
pub enum Token {
    /// Misc
    EOF,
    LET,

    /// Multi-char
    INT(Vec<char>),
    IDENTIFIER(Vec<char>),

    /// Binary operations
    DIVIDE,
    FLOORDIVISION,
    MINUS,
    PLUS,
    EQUAL,
    MOD,

    /// Scope changing
    FUNCTION,
    IF,
    ELSE,
    RETURN,

    /// Special chars
    RBRACKET, // }
    LBRACKET, // {
    LPAREN, // (
    RPAREN, // )
    SEMICOLON,
    COMMA,
    COLON,

    /// Boolean
    LOGICALAND,
    TRUE,
    FALSE,
    LESSTHAN,
    GREATERTHAN,

    /// Tokens for type annotations, not actual types. See data_type for acceptable types
    TINTEGER,
    TFLOAT,
    TBOOLEAN,
}