use crate::frontend::{ 
    utils::error::ErrorType,
    lexer::token::Token,
    ast::{
        ast_struct::ASTNode, syntax_element::SyntaxElement, data_type::DataType
    },
    parser::parser_core::Parser, 
};

impl Parser {
    /// Parses a function
    pub fn parse_function_declaration(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        match self.get_input().get(self.get_current()) {
            Some(Token::FUNCTION) => {
                self.consume_token(Token::FUNCTION)?;
                let mut function_node: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration);

                let (identifier, parameters, return_type) = self.parse_function_signature()?;
                let function_body: ASTNode;
                match self.parse_block() {
                    Ok(Some(block_exp )) => {
                        function_body = block_exp;
                    }
                    _ => {
                        panic!("missing fn body")
                    }
                }

                function_node.add_child(identifier);
                function_node.add_children(parameters);
                match return_type {
                    Some(ret_type) => {
                        function_node.add_child(ret_type);
                    }
                    _ => {}
                }
                function_node.add_child(function_body);

                Ok(Some(function_node))
            },
            _ => panic!("function")
        }
    }

    /// Parses a function declaration
    fn parse_function_signature(&mut self) -> Result<(ASTNode, Vec<ASTNode>, Option<ASTNode>), Vec<ErrorType>> {
        if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            let name: String = name_chars.iter().collect();
            let name_node: ASTNode = ASTNode::new(SyntaxElement::Identifier(name));

            self.consume_token(Token::LPAREN)?;
            let mut parameters: Vec<ASTNode> = Vec::new();

            while let Some(token) = self.get_input().get(self.get_current()) {
                match token {
                    Token::RPAREN => {
                        self.consume_token(Token::RPAREN)?;
                        break;
                    },
                    Token::IDENTIFIER(param_name_chars) => {
                        self.consume_token(Token::IDENTIFIER(param_name_chars.clone()))?;
                        let param_name: String = param_name_chars.iter().collect();
                        let param_name_node: ASTNode = ASTNode::new(SyntaxElement::Identifier(param_name));

                        self.consume_token(Token::COLON)?;
                        let data_type: DataType = self.consume_type()?;

                        let data_type_node: ASTNode = ASTNode::new(SyntaxElement::Type(data_type));

                        let mut param_node: ASTNode = ASTNode::new(SyntaxElement::Parameter);

                        param_node.add_child(param_name_node);
                        param_node.add_child(data_type_node);

                        parameters.push(param_node);
    
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
            let mut return_node: Option<ASTNode> = None;

            match self.get_input().get(self.get_current()) {
                Some(Token::COLON) => {
                    self.consume_token(Token::COLON)?;
            
                    match self.consume_type() {
                        Ok(data_type) => {
                            return_node = Some(ASTNode::new(SyntaxElement::Type(data_type)));
                        },
                        _ => panic!("missing return type")
                    }
                }
                _ => {}
            }
            
            
            Ok((name_node, parameters, return_node))
        } else {
            panic!("functions have names silly!")
        }
    }
    
    /// Parses an enum into a name and variants
    pub fn parse_enum_declaration(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        self.consume_token(Token::ENUM)?;
    
        let enum_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect()
        } else {
            panic!("enums have names silly!")
        };

        let mut enum_node_name = ASTNode::new(SyntaxElement::Identifier(enum_name));
    
        self.consume_token(Token::LBRACE)?;
    
        let mut variants: Vec<ASTNode> = Vec::new();

        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
            if let Some(Token::IDENTIFIER(variant_chars)) = self.get_input().get(self.get_current()) {
                self.consume_token(Token::IDENTIFIER(variant_chars.clone()))?;

                let variant_str: String = variant_chars.iter().collect::<String>();
                let mut variant_node: ASTNode = ASTNode::new(SyntaxElement::Variant);
                variant_node.add_child(ASTNode::new(SyntaxElement::Identifier(variant_str)));
                variants.push(variant_node);
    
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

        let mut enum_node: ASTNode = ASTNode::new(SyntaxElement::EnumDeclaration);
        enum_node.add_child(enum_node_name);
        enum_node.add_children(variants);

        Ok(Some(enum_node))
    }
    
    /// Parses a struct into a name and fields
    pub fn parse_struct_declaration(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        self.consume_token(Token::STRUCT)?;
    
        let struct_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect()
        } else {
            panic!("structs have names silly!")
        };
    
        self.consume_token(Token::LBRACE)?;
    
        let mut fields: Vec<ASTNode> = Vec::new();
        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
            if let Some(Token::IDENTIFIER(field_name_chars)) = self.get_input().get(self.get_current()) {
                self.consume_token(Token::IDENTIFIER(field_name_chars.clone()))?;
                let field_name = field_name_chars.iter().collect::<String>();
                let field_name_node: ASTNode = ASTNode::new(SyntaxElement::Literal{value: field_name});

                self.consume_token(Token::COLON)?;
                let field_type: DataType = self.consume_type()?;
                let field_type_node: ASTNode = ASTNode::new(SyntaxElement::Type(field_type));

                let mut field_node: ASTNode = ASTNode::new(SyntaxElement::Field);

                field_node.add_child(field_name_node);
                field_node.add_child(field_type_node);
                fields.push(field_node);
    
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
    
        let mut identifier_node: ASTNode = ASTNode::new(SyntaxElement::Identifier(struct_name));
        
        let mut struct_node: ASTNode = ASTNode::new(SyntaxElement::StructDeclaration);
        struct_node.add_child(identifier_node);
        struct_node.add_children(fields);

        Ok(Some(struct_node))
    }
}