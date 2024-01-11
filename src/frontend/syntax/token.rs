/*
Defines acceptable tokens in the program
 */


use crate::frontend::utils::error::ErrorType;

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

/// retrieves a token if text matches, else error
pub fn get_token(raw_text: &Vec<char>) -> Result<Token, ErrorType> {
    let identifier: String = raw_text.into_iter().collect();
    match &identifier[..] {
        "let" => Ok(Token::LET),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "return" => Ok(Token::RETURN),
        "Integer" => Ok(Token::TINTEGER),
        "Float" => Ok(Token::TFLOAT),
        "Boolean" => Ok(Token::TBOOLEAN),
        _ => Err(ErrorType::UnrecognizedToken { token: String::from("Unrecognized token") }),
    }
}