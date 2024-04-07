use crate::frontend::{ 
    lexer::token::Token,
    parser::parser_core::Parser,
};


impl Parser {
    /// Converts an operator to a string representation
    pub fn operator_to_char(&self, token: &Token) -> char {
        match token {
            Token::PLUS => '+',
            Token::MINUS => '-',
            Token::MULTIPLY => '*',
            Token::DIVIDE => '/',
            _ => panic!("not an operator")
        }
    }
}