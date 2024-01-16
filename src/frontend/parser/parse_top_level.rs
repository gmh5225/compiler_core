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
    pub fn parse_top_level(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        match self.get_input().get(self.get_current()) {
            Some(Token::FUNCTION) => {
                self.consume_token(Token::FUNCTION)?;

                let (identifier, parameters, return_type) = self.parse_function_declaration()?;
                let function_body: Vec<ASTNode> = self.scope_changing_until(Token::RBRACKET)?;
                
                let mut function_node: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration { 
                    name: identifier, parameters, return_type: return_type.or(None)
                });
                function_node.add_children(function_body);
                Ok(Some(function_node))
            },
            Some(Token::ENUM) => {
                self.consume_token(Token::ENUM)?;

                let (name, variants) = self.parse_enum()?;
                let enum_node: ASTNode = ASTNode::new(SyntaxElement::EnumDeclaration { name, variants });
                Ok(Some(enum_node))
            },
            Some(Token::STRUCT) => {
                self.consume_token(Token::STRUCT)?;

                let (name, fields) = self.parse_struct()?;
                let struct_node: ASTNode = ASTNode::new(SyntaxElement::StructDeclaration { name, fields });
                Ok(Some(struct_node))
            },
            _ => panic!("Silly goose! This {:?} isn't a top level expression!", self.get_input().get(self.get_current())),
        }
    }

    /// After reading a function token, consumes the function declaration
    /// format of function declaration currently: fn foo(a: int, b: bool) {}
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
    
                        println!("{:?}", self.get_input().get(self.get_current()));

                        if self.get_current() < self.get_input().len() {
                            match self.get_input().get(self.get_current()) {
                                Some(Token::COMMA) => self.consume_token(Token::COMMA)?,
                                Some(Token::RPAREN) => {}
                                _ => panic!("unexpected parse_function")
                            }
                        }
                    },
                    _ => {
                        println!("{:?}", token);
                        panic!("problem in function_declaration")
                    },
                }
            }
    
            let mut return_type: Option<DataType> = None;
                match self.get_input().get(self.get_current()) {
                    Some(Token::COLON) => {
                        self.consume_token(Token::COLON)?;
                
                        match self.consume_type() {
                            Ok(data_type) => {
                                return_type = Some(data_type);
                            },
                            _ => panic!("missing return type")
                        }
                    }
                    _ => {}
                }
        
            Ok((name, parameters, return_type))
        } else {
            panic!("functions have names silly!")
        }
    }
    
    /// Parses an enum into a name and variants
    /// format of enum currently: enum foo {variant, variant2, variant3}
    pub fn parse_enum(&mut self) -> Result<(String, Vec<String>), Vec<ErrorType>> {
        self.consume_token(Token::ENUM)?;
    
        let enum_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect()
        } else {
            panic!("enums have names silly!")
        };
    
        self.consume_token(Token::LBRACE)?;
    
        let mut variants: Vec<String> = Vec::new();
        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
            if let Some(Token::IDENTIFIER(variant_chars)) = self.get_input().get(self.get_current()) {
                self.consume_token(Token::IDENTIFIER(variant_chars.clone()))?;
                variants.push(variant_chars.iter().collect::<String>());
    
                if let Some(Token::COMMA) = self.get_input().get(self.get_current()) {
                    self.consume_token(Token::COMMA)?;
                } else if self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
                    panic!("unexpected token in enum")
                }
            } else {
                panic!("enums have variants with names")
            }
        }
    
        self.consume_token(Token::RBRACE)?;
    
        Ok((enum_name, variants))
    }
    
    /// Parses a struct into a name and fields
    /// format of a struct currently: struct foo {field: type, field2: type2}
    pub fn parse_struct(&mut self) -> Result<(String, Vec<(String, DataType)>), Vec<ErrorType>> {
        self.consume_token(Token::STRUCT)?;
    
        let struct_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect()
        } else {
            panic!("structs have names silly!")
        };
    
        self.consume_token(Token::LBRACE)?;
    
        let mut fields: Vec<(String, DataType)> = Vec::new();
        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
            if let Some(Token::IDENTIFIER(field_name_chars)) = self.get_input().get(self.get_current()) {
                self.consume_token(Token::IDENTIFIER(field_name_chars.clone()))?;
                let field_name = field_name_chars.iter().collect::<String>();
    
                self.consume_token(Token::COLON)?;
    
                let field_type: DataType = self.consume_type()?;
    
                fields.push((field_name, field_type));
    
                if let Some(Token::COMMA) = self.get_input().get(self.get_current()) {
                    self.consume_token(Token::COMMA)?;
                } else if self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
                    panic!("unexpectd token in parse_struct")
                }
            } else {
                panic!("problem parse_struct")
            }
        }
    
        self.consume_token(Token::RBRACE)?;
    
        Ok((struct_name, fields))
    }
}