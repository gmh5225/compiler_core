use crate::frontend::{ 
    utils::error::ErrorType,
    syntax::token::Token,
    ast::data_type::DataType,
    parser::parser_core::Parser,
};


impl Parser {
    pub fn consume_type(&mut self) -> Result<DataType, ErrorType> {
        if let Some(token) = self.get_input().get(self.get_current()) {
            match token {
                Token::TINTEGER => {
                    self.consume_token(Token::TINTEGER)?;
                    Ok(DataType::Integer)
                }
                Token::TBOOLEAN => {
                    self.consume_token(Token::TBOOLEAN)?;
                    Ok(DataType::Boolean)
                }  
                _ => panic!("not a type"),
            }
        }
        else {
            panic!("no type to consume");
        }
    }
    
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