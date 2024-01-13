/*
Defines acceptable tokens in the program
 */

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    EOF,

    /// Assignment
    LET,
    PLUSASSIGN, // +=
    MINUSASSIGN,
    MULTIPLYASSIGN,
    DIVIDEASSIGN,
    MODASSIGN,

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
    MULTIPLY,
    EXPONENT,

    /// Scope changing
    FUNCTION,
    STRUCT,
    ENUM,
    IF,
    ELIF,
    ELSE,
    RETURN,
    FOR,
    WHILE,
    DO,
    BREAK,
    CONTINUE,
    MATCH,
    ARROW,

    /// Special chars
    RBRACKET, // }
    LBRACKET, // {
    LPAREN, // (
    RPAREN, // )
    SEMICOLON,
    COMMA,
    COLON,
    LBRACE, // [
    RBRACE, // ]
    DOT,
    COLONCOLON,

    /// Boolean
    LOGICALAND,
    LOGICALOR,
    LOGICALNOT,
    TRUE,
    FALSE,
    LESSTHAN,
    GREATERTHAN,
    NOTEQUAL,
    EQUALEQUAL, // ==
    LESSTHANEQUAL,
    GREATERTHANEQUAL,

    /// Tokens for type annotations, not actual types. See data_type for acceptable types
    TINTEGER,
    TFLOAT,
    TBOOLEAN,
    TSTRING,
    TCHAR,
    TVOID,
}