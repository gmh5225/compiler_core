use crate::frontend::{ 
    utils::error::ErrorType,
    syntax::token::Token,
    ast::{
        ast_struct::ASTNode, syntax_element::{
            SyntaxElement, FunctionParameter
        }, data_type::DataType
    },
    parser::parser_core::Parser,
};

impl<'a> Parser<'a> {
    /// Parses top level expressions 
    pub fn parse_top_level(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        match self.get_input().get(self.get_current()) {
            Some(Token::FUNCTION) => {
                self.consume_token(Token::FUNCTION)?;

                let (identifier, parameters, return_type) = self.parse_function_declaration()?;
                let function_body: Vec<ASTNode> = self.scope_changing_until(Token::RBRACKET)?;
                
                let mut function_node: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration { 
                    name: identifier, parameters, return_type: return_type.or(None)
                });
                function_node.add_children(function_body);
                Ok(function_node)
            },
            Some(Token::ENUM) => {
                self.consume_token(Token::ENUM)?;

                let (name, variants) = self.parse_enum()?;
                let enum_node = ASTNode::new(SyntaxElement::EnumDeclaration { name, variants });
                Ok(enum_node)
            },
            Some(Token::STRUCT) => {
                self.consume_token(Token::STRUCT)?;

                let (name, fields) = self.parse_struct()?;
                let struct_node = ASTNode::new(SyntaxElement::StructDeclaration { name, fields });
                Ok(struct_node)
            },
            _ => Err(vec![ErrorType::DevError {  }]),
        }
    }

    /// After reading a function token, consumes the function declaration
    pub fn parse_function_declaration(&mut self) -> Result<(String, Vec<FunctionParameter>, Option<DataType>), Vec<ErrorType>> {
        if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            let name: String = name_chars.iter().collect();
    
            self.consume_token(Token::LPAREN)?;
    
            let mut parameters: Vec<FunctionParameter> = Vec::new();
            while let Some(token) = self.get_input().get(self.get_current()) {
                match token {
                    Token::RPAREN => {
                        self.consume_token(Token::RPAREN)?;
                        break;
                    },
                    Token::IDENTIFIER(param_name_chars) => {
                        self.consume_token(Token::IDENTIFIER(param_name_chars.clone()))?;
                        let param_name: String = param_name_chars.iter().collect();
    
                        self.consume_token(Token::COLON)?;
                        let param_type: DataType = self.consume_type()?;
    
                        parameters.push(FunctionParameter::new(param_name, param_type));
    
                        if let Some(Token::COMMA) | Some(Token::RPAREN) = self.get_input().get(self.get_current()) {
                            if let Token::COMMA = token {
                                self.consume_token(Token::COMMA)?;
                            }
                        } else {
                            return Err(vec![ErrorType::DevError {}]);
                        }
                    },
                    _ => return Err(vec![ErrorType::DevError {}]),
                }
            }
    
            let mut return_type: Option<DataType> = None;
            if let Some(_) = self.get_input().get(self.get_current()) {
                return_type = Some(self.consume_type()?);
            }
    
            self.consume_token(Token::LBRACKET)?;
    
            Ok((name, parameters, return_type))
        } else {
            Err(vec![ErrorType::DevError {}])
        }
    }
    
    pub fn parse_enum(&mut self) -> Result<(String, Vec<String>), Vec<ErrorType>> {
        self.consume_token(Token::ENUM)?;
    
        let enum_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect()
        } else {
            return Err(vec![ErrorType::DevError {  } ])
        };
    
        self.consume_token(Token::LBRACE)?;
    
        let mut variants = Vec::new();
        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
            if let Some(Token::IDENTIFIER(variant_chars)) = self.get_input().get(self.get_current()) {
                self.consume_token(Token::IDENTIFIER(variant_chars.clone()))?;
                variants.push(variant_chars.iter().collect::<String>());
    
                if let Some(Token::COMMA) = self.get_input().get(self.get_current()) {
                    self.consume_token(Token::COMMA)?;
                } else if self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
                    return Err(vec![ErrorType::DevError {  } ])
                }
            } else {
                return Err(vec![ErrorType::DevError {  } ])
            }
        }
    
        self.consume_token(Token::RBRACE)?;
    
        Ok((enum_name, variants))
    }
    

    pub fn parse_struct(&mut self) -> Result<(String, Vec<(String, DataType)>), Vec<ErrorType>> {
        self.consume_token(Token::STRUCT)?;
    
        let struct_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect()
        } else {
            return Err(vec![ErrorType::DevError {  } ])
        };
    
        self.consume_token(Token::LBRACE)?;
    
        let mut fields = Vec::new();
        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
            if let Some(Token::IDENTIFIER(field_name_chars)) = self.get_input().get(self.get_current()) {
                self.consume_token(Token::IDENTIFIER(field_name_chars.clone()))?;
                let field_name = field_name_chars.iter().collect::<String>();
    
                self.consume_token(Token::COLON)?;
    
                let field_type = self.consume_type()?;
    
                fields.push((field_name, field_type));
    
                if let Some(Token::COMMA) = self.get_input().get(self.get_current()) {
                    self.consume_token(Token::COMMA)?;
                } else if self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
                    return Err(vec![ErrorType::DevError {  } ])
                }
            } else {
                return Err(vec![ErrorType::DevError {  } ])
            }
        }
    
        self.consume_token(Token::RBRACE)?;
    
        Ok((struct_name, fields))
    }
}