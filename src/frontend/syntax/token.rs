/*
Defines acceptable tokens in the program
 */
#[derive(PartialEq, Debug)]
pub enum Token {
    ERROR,
    EOF,
    INT(Vec<char>),
    IDENTIFIER(Vec<char>),
    PLUS,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    MINUS,
    EQUAL,
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
        _ => Err(String::from("Unexpected keyword"))
    }
}
