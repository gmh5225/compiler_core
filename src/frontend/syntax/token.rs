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
    FUNCTION,

    /// Binary operations
    DIVIDE,
    FLOORDIVISION,
    MINUS,
    PLUS,
    EQUAL,

    /// Scope changing
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

pub fn is_primitive(token: Token) -> bool {
    match token {
        Token::INT(_) => true,
        Token::TRUE => true,
        Token::FALSE => true,
        Token::IDENTIFIER(_) => true,
        _ => false,
    }
}
pub fn is_operator(token: Token) -> bool {
    match token {
        Token::PLUS => true,
        Token::MINUS => true,
        Token::DIVIDE => true,
        Token::FLOORDIVISION => true,
        Token::GREATERTHAN => true,
        Token::LESSTHAN => true,
        Token::LOGICALAND => true,
        _ => false,
    }
}

pub fn is_type_notation(token: Token) -> bool {
    match token {
        Token::TINTEGER => true,
        Token::TFLOAT => true,
        Token::TBOOLEAN => true,
        _ => false,
    }
}

pub fn is_delimiter(token: Token) -> bool {
    match token {
        Token::SEMICOLON => true,
        Token::RBRACKET => true,
        _ => false,
    }
}