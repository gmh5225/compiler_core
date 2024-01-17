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
    /// Base level of the parser
    pub fn parse_token(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::INT(_)) | 
                Some(Token::TRUE) |
                Some(Token::FALSE) => return self.parse_primitive(), 
                Some(Token::IDENTIFIER(_)) => return self.parse_identifier(),
                Some(Token::TBOOLEAN) |
                Some(Token::TCHAR) |
                Some(Token::TFLOAT) |
                Some(Token::TINTEGER) |
                Some(Token::TSTRING) |
                Some(Token::TVOID) => panic!("Shouldn't be getting types at parse_token. Types are consumed as parts of expressions"),
                _ => return self.parse_protected_keyword(),
            }
        } else {
            panic!("parse_token panic")
        }
    }

    fn parse_primitive(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::INT(value)) => {
                    self.consume_token(Token::INT(value.clone()))?;
                    let value_str = value.iter().collect::<String>();
                    return Ok(Some(ASTNode::new(SyntaxElement::Literal(DataType::Integer, value_str))));
                },
                Some(Token::TRUE) => {
                    self.consume_token(Token::TRUE)?;
                    return Ok(Some(ASTNode::new(SyntaxElement::Literal(DataType::Boolean, "true".to_string()))))
                },
                Some(Token::FALSE) => {
                    self.consume_token(Token::FALSE)?;
                    return Ok(Some(ASTNode::new(SyntaxElement::Literal(DataType::Boolean, "false".to_string()))))
                },
                _ => panic!("{:?}", self.get_input().get(self.get_current()))
            }
        } panic!("parse_primitive panic")
    }

    fn parse_identifier(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::IDENTIFIER(name_chars)) => {
                    // match self.peek_token() {
                    //     Some(Token::LESSTHAN) => {
                    //         return self.parse_binary_expression();
                    //     }
                    //     _ => {}
                    // }
                    self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
                    let name: String = name_chars.iter().collect();
                    match self.get_input().get(self.get_current()) {
                        Some(Token::EQUAL) => {
                            return self.parse_assignment();
                        }
                        _ => return Ok(Some(ASTNode::new(SyntaxElement::Variable(DataType::String, name)))),
                    }
                }
                _ => panic!("how'd you even hit this?")
            }
        } panic!("parse_identifier panic")
    }

    fn parse_protected_keyword(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::BREAK) => {
                    self.consume_token(Token::BREAK)?;
                    self.consume_token(Token::SEMICOLON)?;
                    return Ok(Some(ASTNode::new(SyntaxElement::Break)));
                }
                Some(Token::CONTINUE) => {
                    self.consume_token(Token::CONTINUE)?;
                    self.consume_token(Token::SEMICOLON)?;
                    return Ok(Some(ASTNode::new(SyntaxElement::Continue)));
                }
                Some(Token::RETURN) => {
                    self.consume_token(Token::RETURN)?;
                    let value = match self.parse_element() {
                        Ok(Some(value)) => {value}
                        _ => panic!("return panic")
                    };
                    self.consume_token(Token::SEMICOLON)?;
                    return Ok(Some(ASTNode::new(SyntaxElement::Return{value: Box::new(value)})))
                }
                Some(Token::SEMICOLON) => {
                    self.consume_token(Token::SEMICOLON)?;
                    return Ok(None);
                },
                Some(Token::EOF) => {
                    self.consume_token(Token::EOF)?;
                    return Ok(None);
                }
                _ => panic!("Are you sure this is a protected keyword? {:?}", self.get_input().get(self.get_current()))
            }
        } panic!("parse_protected_keyword panic")
    }
}