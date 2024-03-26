use crate::frontend::{ 
    utils::error::ErrorType,
    lexer::token::Token,
    ast::{
        ast_struct::ASTNode, 
        syntax_element::SyntaxElement, 
        data_type::DataType
    },
    parser::parser_core::Parser,
};

impl Parser {
    /// Parses a primitive value
    pub fn parse_primitive(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::INT(value)) => {
                    self.consume_token(Token::INT(value.clone()))?;
                    let value_str: String = value.iter().collect();
                    return Ok(Some(ASTNode::new(SyntaxElement::Literal(value_str))));
                },
                Some(Token::TRUE) => {
                    self.consume_token(Token::TRUE)?;
                    return Ok(Some(ASTNode::new(SyntaxElement::Literal("true".to_string()))))
                },
                Some(Token::FALSE) => {
                    self.consume_token(Token::FALSE)?;
                    return Ok(Some(ASTNode::new(SyntaxElement::Literal("false".to_string()))))
                },
                _ => panic!("{:?}", self.get_input().get(self.get_current()))
            }
        } panic!("parse_primitive panic")
    }

    /// Parses an identifier
    pub fn parse_identifier(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        let input = self.get_input();
        let name_chars: &Vec<char> = match input.get(self.get_current()) {
            Some(Token::IDENTIFIER(name_chars)) => {
                name_chars
            }
            _ => panic!("expected id")
        };
        self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
        match self.get_input().get(self.get_current()) {
            Some(Token::EQUAL) => {
                return self.parse_assignment();
            }
            _ => return Ok(Some(ASTNode::new(SyntaxElement::Identifier(name_chars.iter().collect())))),
        } 
    }

    /// Parses a protected keyword
    pub fn parse_protected_keyword(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
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
                    let value = match self.parse_router() {
                        Ok(Some(value)) => {value}
                        _ => panic!("return panic")
                    };

                    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
                    assigned_value_node.add_child(value);
                    
                    self.consume_token(Token::SEMICOLON)?;

                    let mut return_node: ASTNode = ASTNode::new(SyntaxElement::Return);
                    return_node.add_child(assigned_value_node);

                    return Ok(Some(return_node));
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

    /// Consumes a type token
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
}