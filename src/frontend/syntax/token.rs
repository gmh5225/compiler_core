/*
Defines acceptable tokens in the program
 */

#[derive(PartialEq, Debug)]
pub enum Token {
    /// Misc
    EOF,
    LET,
    RETURN,

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
    IF,
    ELSE,
    FUNCTION,

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

pub fn get_token(raw_text: &Vec<char>) -> Result<Token, String> {
    let identifier: String = raw_text.into_iter().collect();
    match &identifier[..] {
        "let" => Ok(Token::LET),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "return" => Ok(Token::RETURN),
        "fn" => Ok(Token::FUNCTION),
        "Integer" => Ok(Token::TINTEGER),
        "Float" => Ok(Token::TFLOAT),
        "Boolean" => Ok(Token::TBOOLEAN),
        _ => Err(String::from("Unexpected keyword"))
    }
}
