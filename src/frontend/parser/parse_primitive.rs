use crate::frontend::{ 
    utils::error::ErrorType,
    syntax::token::Token,
    ast::{
        ast_struct::ASTNode, 
        syntax_element::SyntaxElement, 
        data_type::DataType
    },
    parser::parser_core::Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_primitive(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        match self.get_input().get(self.get_current()) {
            Some(Token::INT(value)) => {
                self.consume_token(Token::INT(value.clone()))?;
                let value_str = value.iter().collect::<String>();
                Ok(ASTNode::new(SyntaxElement::Literal(DataType::Integer, value_str)))
            },
            Some(Token::TRUE) => {
                self.consume_token(Token::TRUE)?;
                Ok(ASTNode::new(SyntaxElement::Literal(DataType::Boolean, "true".to_string())))
            },
            Some(Token::FALSE) => {
                self.consume_token(Token::FALSE)?;
                Ok(ASTNode::new(SyntaxElement::Literal(DataType::Boolean, "false".to_string())))
            },
            Some(Token::IDENTIFIER(name_chars)) => {
                self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
                let name = name_chars.iter().collect::<String>();
                Ok(ASTNode::new(SyntaxElement::Variable(name)))
            },
            _ => panic!("{:?}", self.get_input().get(self.get_current()))
        }
    }
}