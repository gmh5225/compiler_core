/*
Converts tokens into an AST
*/
                                 
use crate::frontend::{ 
    utils::{error::ErrorType, binop_precedence::binop_precedence},
    syntax::token::Token,
    ast::{
        ast_struct::{AST, ASTNode}, syntax_element::{
            SyntaxElement, FunctionParameter, MatchArm
        }, data_type::DataType
    }
};

/// Parses an input of tokens into an AST   
pub struct Parser<'a> {
    input: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a Vec<Token>) -> Self {
        Self {
            input,
            current: 0,
        }
    } 
    
    /// Parses an input of tokens into an AST, or returns a vector of errors
    pub fn parse(input: Vec<Token>) -> Result<AST, Vec<ErrorType>> {
        binop_precedence();

        let mut parser: Parser<'_> = Parser::new(&input);
        let mut root_children: Vec<ASTNode> = Vec::new();  
        let mut errors: Vec<ErrorType> = Vec::new();

        while parser.current < input.len() {
            match parser.parse_expression() { 
                Ok(Some(node)) => {
                    root_children.push(node);  
                }
                Ok(None) => {}
                Err(error_types) => {
                    errors.extend(error_types);
                }
            } 
        }

        let mut root: ASTNode = ASTNode::new(SyntaxElement::ModuleExpression);
        root.add_children(root_children);
        if errors.is_empty() {
            return Ok(AST::new(root));
        }
        Err(errors)
    }

    /// Parses top level expressions 
    fn parse_top_level(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        match self.input.get(self.current) {
            Some(Token::FUNCTION) => {
                self.consume_token(Token::FUNCTION)?;
    
                let (identifier, parameters, return_type) = self.parse_function_declaration()?;
                let function_body: Vec<ASTNode> = self.scope_changing(Token::RBRACKET)?;
                
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
    fn parse_function_declaration(&mut self) -> Result<(String, Vec<FunctionParameter>, Option<DataType>), Vec<ErrorType>> {
        if let Some(Token::IDENTIFIER(name_chars)) = self.input.get(self.current) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            let name: String = name_chars.iter().collect();
    
            self.consume_token(Token::LPAREN)?;
    
            let mut parameters: Vec<FunctionParameter> = Vec::new();
            while let Some(token) = self.input.get(self.current) {
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
    
                        if let Some(Token::COMMA) | Some(Token::RPAREN) = self.input.get(self.current) {
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
            if let Some(_) = self.input.get(self.current) {
                return_type = Some(self.consume_type()?);
            }
    
            self.consume_token(Token::LBRACKET)?;
    
            Ok((name, parameters, return_type))
        } else {
            Err(vec![ErrorType::DevError {}])
        }
    }
    
    fn parse_enum(&mut self) -> Result<(String, Vec<String>), Vec<ErrorType>> {
        self.consume_token(Token::ENUM)?;
    
        let enum_name = if let Some(Token::IDENTIFIER(name_chars)) = self.input.get(self.current) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect()
        } else {
            return Err(vec![ErrorType::DevError {  } ])
        };
    
        self.consume_token(Token::LBRACE)?;
    
        let mut variants = Vec::new();
        while self.current < self.input.len() && self.input.get(self.current) != Some(&Token::RBRACE) {
            if let Some(Token::IDENTIFIER(variant_chars)) = self.input.get(self.current) {
                self.consume_token(Token::IDENTIFIER(variant_chars.clone()))?;
                variants.push(variant_chars.iter().collect::<String>());
    
                if let Some(Token::COMMA) = self.input.get(self.current) {
                    self.consume_token(Token::COMMA)?;
                } else if self.input.get(self.current) != Some(&Token::RBRACE) {
                    return Err(vec![ErrorType::DevError {  } ])
                }
            } else {
                return Err(vec![ErrorType::DevError {  } ])
            }
        }
    
        self.consume_token(Token::RBRACE)?;
    
        Ok((enum_name, variants))
    }
    

    fn parse_struct(&mut self) -> Result<(String, Vec<(String, DataType)>), Vec<ErrorType>> {
        self.consume_token(Token::STRUCT)?;
    
        let struct_name = if let Some(Token::IDENTIFIER(name_chars)) = self.input.get(self.current) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect()
        } else {
            return Err(vec![ErrorType::DevError {  } ])
        };
    
        self.consume_token(Token::LBRACE)?;
    
        let mut fields = Vec::new();
        while self.current < self.input.len() && self.input.get(self.current) != Some(&Token::RBRACE) {
            if let Some(Token::IDENTIFIER(field_name_chars)) = self.input.get(self.current) {
                self.consume_token(Token::IDENTIFIER(field_name_chars.clone()))?;
                let field_name = field_name_chars.iter().collect::<String>();
    
                self.consume_token(Token::COLON)?;
    
                let field_type = self.consume_type()?;
    
                fields.push((field_name, field_type));
    
                if let Some(Token::COMMA) = self.input.get(self.current) {
                    self.consume_token(Token::COMMA)?;
                } else if self.input.get(self.current) != Some(&Token::RBRACE) {
                    return Err(vec![ErrorType::DevError {  } ])
                }
            } else {
                return Err(vec![ErrorType::DevError {  } ])
            }
        }
    
        self.consume_token(Token::RBRACE)?;
    
        Ok((struct_name, fields))
    }

    fn scope_changing(&mut self, stop_token: Token) -> Result<Vec<ASTNode>, Vec<ErrorType>> {
        let mut children: Vec<ASTNode> = Vec::new();
        self.consume_token(Token::LBRACKET)?;
    
        while self.current < self.input.len() && self.input.get(self.current) != Some(&stop_token) {
            match self.input.get(self.current) {
                Some(Token::IF) => {
                    match self.parse_if_statement(){
                        Ok(if_node) => {
                            children.push(if_node);
                        }
                        _ => panic!("scope_changing if_statement panic")
                    }   
                },
                Some(Token::FOR) => {
                    match self.parse_for_loop(){
                        Ok(for_node) => {
                            children.push(for_node);
                        }
                        _ => panic!("scope_changing for_loop panic")
                    }
                    
                },
                Some(Token::WHILE) => {
                    match self.parse_while_loop(){
                        Ok(while_node) => {
                            children.push(while_node);
                        }
                        _ => panic!("scope_changing while panic")
                    }
                },
                Some(Token::DO) => {
                    match self.parse_do_while_loop(){
                        Ok(do_while_node) => {
                            children.push(do_while_node);
                        }
                        _ => panic!("scope_changing do_while panic")
                    }
                },
                Some(Token::MATCH) => {
                    match self.parse_match_statement(){
                        Ok(match_node) => {
                            children.push(match_node);
                        }
                        _ => panic!("scope_changing match panic")
                    }
                },
                _ => {
                    match self.parse_expression() {
                        Ok(Some(expr_node)) => {
                            children.push(expr_node);
                        }
                        Ok(None) => {}
                        _ => panic!("scope_changing final parse")
                    }
                }
            }
        }
        if self.input.get(self.current) == Some(&stop_token) {
            self.consume_token(stop_token)?;
        }
        Ok(children)
    }
    
    fn parse_match_arms(&mut self) -> Result<Vec<MatchArm>, Vec<ErrorType>> {
        let mut arms: Vec<MatchArm> = Vec::new();
    
        self.consume_token(Token::LBRACE)?;
    
        while self.current < self.input.len() && self.input.get(self.current) != Some(&Token::RBRACE) {
            let variant: ASTNode = match self.parse_expression() {
                Ok(Some(value)) => value, 
                Ok(None) => {
                    panic!("Assignment value is missing");
                }
                Err(_) => {
                    panic!("Failed to parse assignment value");
                }
            };
    
            self.consume_token(Token::ARROW)?;  
    
            let action: ASTNode = match self.parse_expression() {
                Ok(Some(value)) => value, 
                Ok(None) => {
                    panic!("Assignment value is missing");
                }
                Err(_) => {
                    panic!("Failed to parse assignment value");
                }
            };
    
            arms.push(MatchArm::new(variant, action));
    
            if let Some(Token::COMMA) = self.input.get(self.current) {
                self.consume_token(Token::COMMA)?;
            } else if self.input.get(self.current) != Some(&Token::RBRACE) {
                return Err(vec![ErrorType::DevError {  } ])
            }
        }
    
        self.consume_token(Token::RBRACE)?;
    
        Ok(arms)
    }
    
    fn parse_if_statement(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::IF) => {
                    self.consume_token(Token::IF)?;
                    self.consume_token(Token::LPAREN)?;
                    
                    let condition: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => {value}
                        _ => panic!("if statement panic")
                    };
                    self.consume_token(Token::RPAREN)?;
                    let nodes: Vec<ASTNode> = self.scope_changing(Token::RBRACKET)?;
                    let then_branch: ASTNode = nodes.get(0).unwrap().clone();
                    let else_branch: Option<Box<ASTNode>> = if nodes.len() > 1 {
                        Some(Box::new(nodes[1].clone()))
                    } else {
                        None
                    };

                    let if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement { 
                        condition: Box::new(condition), 
                        then_branch: Box::new(then_branch), 
                        else_branch,
                    });
                    return Ok(if_node);
                }
                _ => panic!("Problem parsing in if statement"),
            }
        } panic!("Problem parsing if statement 2")
    }
    
    fn parse_for_loop(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::FOR) => {
                    self.consume_token(Token::FOR)?;
                    let value: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("for loop 1");
                        }
                        Err(_) => {
                            panic!("for lop 7");
                        }
                    };
                    let initializer = if let Some(Token::LET) = self.input.get(self.current) {
                        Some(Box::new(value))
                    } else {
                        None
                    };
                    let value: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("for loop 3");
                        }
                        Err(_) => {
                            panic!("for loop 4");
                        }
                    };
                    let condition: Box<ASTNode> = Box::new(value);
                    let value: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("for loop 6");
                        }
                        Err(_) => {
                            panic!("for loop 5");
                        }
                    };
                    let increment: Option<Box<ASTNode>> = if let Some(Token::SEMICOLON) = self.input.get(self.current) {
                        self.consume_token(Token::SEMICOLON)?;
                        Some(Box::new(value))
                    } else {
                        None
                    };
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing(Token::RBRACKET)?);
    
                    let for_node: ASTNode = ASTNode::new(SyntaxElement::ForLoop {
                        initializer,
                        condition,
                        increment,
                        body,
                    });
                    return Ok(for_node);
                }
                _ => panic!("Problem parsing in for loop"),
            }
        } panic!("Problem parsing for loop 2")
    }

    fn parse_while_loop(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::WHILE) => {
                    self.consume_token(Token::WHILE)?;
                    let value: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("Assignment value is missing");
                        }
                        Err(_) => {
                            panic!("Failed to parse assignment value");
                        }
                    };
                    let condition: Box<ASTNode> = Box::new(value);
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing(Token::RBRACKET)?);

                    let while_node = ASTNode::new(SyntaxElement::WhileLoop {
                        condition,
                        body,
                    });
                    return Ok(while_node);
                } 
                _ => panic!("problem while loop parsing"),
            }
        } panic!("problem while loop parsing 2")
    }

    fn parse_do_while_loop(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::DO) => {
                    self.consume_token(Token::DO)?;
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing(Token::WHILE)?);
                    let value: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("do while value is missing");
                        }
                        Err(_) => {
                            panic!("Failed to parse do while value");
                        }
                    };
                    let condition: Box<ASTNode> = Box::new(value);

                    let do_while_node = ASTNode::new(SyntaxElement::DoWhileLoop {
                        body,
                        condition,
                    });
                    return Ok(do_while_node);
                }
                _ => panic!("problem do_while parsing"),
            }
        } panic!("problem do_while parsing 2")
    }

    fn parse_match_statement(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::MATCH) => {
                    self.consume_token(Token::MATCH)?;
                    let value: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("Assignment value is missing");
                        }
                        Err(_) => {
                            panic!("Failed to parse assignment value");
                        }
                    };
                    let to_match: Box<ASTNode> = Box::new(value);
                    let arms: Vec<MatchArm> = self.parse_match_arms()?;
                    
                    let match_node = ASTNode::new(SyntaxElement::MatchStatement {
                        to_match,
                        arms,
                    });
                    return Ok(match_node);
                }
                _ => panic!("problem match parsing"),
            }
        } panic!("problem matach parsing 2")
    }

    fn parse_unary_expression(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::MINUS) | Some(Token::LOGICALNOT) => {
                    // Extract the operator
                    let operator = match self.input.get(self.current) {
                        Some(Token::MINUS) => "-",
                        Some(Token::LOGICALNOT) => "!",
                        _ => return Err(vec![ErrorType::DevError{}]),
                    }.to_string();
    
                    self.consume_token(self.input[self.current].clone())?;
    
                    let operand: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("unary is missing");
                        }
                        Err(_) => {
                            panic!("Failed to parse unary value");
                        }
                    };    
                    Ok(ASTNode::new(SyntaxElement::UnaryExpression {
                        operator,
                        operand: Box::new(operand),
                    }))
                },
                _ => self.parse_primitive(),
            }
        } else {
            Err(vec![ErrorType::DevError{}])
        }
    }
    

    fn parse_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::FUNCTION) | Some(Token::STRUCT) | Some(Token::ENUM) => Ok(Some(self.parse_top_level()?)),
                Some(Token::IF) => return Ok(Some(self.parse_if_statement()?)),  
                Some(Token::FOR) => return Ok(Some(self.parse_for_loop()?)),  
                Some(Token::DO) => return Ok(Some(self.parse_do_while_loop()?)),  
                Some(Token::WHILE) => return Ok(Some(self.parse_while_loop()?)),
                Some(Token::MATCH) => return Ok(Some(self.parse_match_statement()?)),
                Some(Token::LET) => return Ok(Some(self.parse_initialization()?)),
                Some(Token::PLUS) | 
                Some(Token::MINUS) | 
                Some(Token::MULTIPLY) | 
                Some(Token::DIVIDE) => return Ok(Some(self.parse_binary_expression()?)),
                Some(Token::LOGICALNOT) => return Ok(Some(self.parse_unary_expression()?)),
                Some(Token::INT(_)) |
                Some(Token::TRUE) | 
                Some(Token::FALSE) => return Ok(Some(self.parse_primitive()?)),
                Some(Token::IDENTIFIER(_)) => return Ok(Some(self.parse_assignment()?)),
                Some(Token::BREAK) => {
                    return Ok(Some(ASTNode::new(SyntaxElement::Break)));
                },
                Some(Token::CONTINUE) => {
                    return Ok(Some(ASTNode::new(SyntaxElement::Continue)));
                },
                Some(Token::RETURN) => {
                    self.consume_token(Token::RETURN)?;
                    let value = match self.parse_expression() {
                        Ok(Some(value)) => {value}
                        _ => panic!("return panic")
                    };
                    return Ok(Some(ASTNode::new(SyntaxElement::Return{value: Box::new(value)})))
                }
                Some(Token::SEMICOLON) => {
                    self.consume_token(Token::SEMICOLON)?;
                    Ok(None)
                },
                Some(Token::EOF) => {
                    self.consume_token(Token::EOF)?;
                    Ok(None) // this Result<Option double wrapping isn't ideal, consider refactoring
                }
                _ => panic!("{:?}", self.input.get(self.current)),

            }
        } else {
            panic!("parse expression 2")
        }
    }

    fn parse_initialization(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::LET) => {
                    self.consume_token(Token::LET)?;
    
                    let variable_name = if let Some(Token::IDENTIFIER(name_chars)) = self.input.get(self.current) {
                        self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
                        name_chars.iter().collect::<String>()
                    } else {
                        return Err(vec![ErrorType::DevError {  }]);
                    };
    
                    if let Some(Token::EQUAL) = self.input.get(self.current) {
                        self.consume_token(Token::EQUAL)?;
                    } else {
                        return Err(vec![ErrorType::DevError{}]);
                    }
    
                    let value: ASTNode = match self.parse_expression() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("initialization value is missing");
                        }
                        Err(_) => {
                            panic!("Failed to parse initialization value");
                        }
                    };    
                    Ok(ASTNode::new(SyntaxElement::Initialization {
                        variable: variable_name,
                        value: Box::new(value),
                    }))
                },
                _ => Err(vec![ErrorType::DevError {  }]),
            }
        } else {
            Err(vec![ErrorType::DevError {  }])
        }
    }
    
    fn parse_assignment(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        let variable_name = if let Some(Token::IDENTIFIER(name_chars)) = self.input.get(self.current) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect::<String>()
        } else {
            panic!("parse_assignment 1")
        };
    
        if let Some(Token::EQUAL) = self.input.get(self.current) {
            self.consume_token(Token::EQUAL)?;
        } else {
            panic!("parse_assignment 2")
        }
    
        let value: ASTNode = match self.parse_expression() {
            Ok(Some(value)) => value, 
            Ok(None) => {
                panic!("Assignment value is missing");
            }
            Err(_) => {
                panic!("Failed to parse assignment value");
            }
        };
        
        Ok(ASTNode::new(SyntaxElement::Assignment {
            variable: variable_name,
            value: Box::new(value),
        }))
        
    }
    
    fn parse_binary_expression(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        let mut left = self.parse_primitive()?; 
    
        while let Some(op_token) = self.input.get(self.current) {
            if let Some(&precedence) = binop_precedence().get(&&self.operator_to_char(op_token)) { 
                self.consume_token(op_token.clone())?;
    
                let mut right = self.parse_primitive()?;
                while let Some(next_op) = self.input.get(self.current) {
                    if let Some(&next_precedence) = binop_precedence().get(&self.operator_to_char(next_op)) {
                        if precedence < next_precedence {
                            right = self.parse_binary_expression()?;
                            break;
                        }
                    }
                    break;
                }
    
                let operator = self.operator_to_char(op_token).to_string();
                left = ASTNode::new(SyntaxElement::BinaryExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }
    
        Ok(left)
    }
    
    fn operator_to_char(&self, token: &Token) -> char {
        match token {
            Token::PLUS => '+',
            Token::MINUS => '-',
            Token::MULTIPLY => '*',
            Token::DIVIDE => '/',
            _ => panic!("not an operator")
        }
    }

    fn parse_primitive(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        match self.input.get(self.current) {
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
            _ => panic!("primitive")
        }
    }

    fn consume_type(&mut self) -> Result<DataType, ErrorType> {
        if let Some(token) = self.input.get(self.current) {
            match token {
                Token::TINTEGER => {
                    self.consume_token(Token::TINTEGER)?;
                    Ok(DataType::Integer)
                }
                Token::TBOOLEAN => {
                    self.consume_token(Token::TBOOLEAN)?;
                    Ok(DataType::Boolean)
                }  
                _ => Err(ErrorType::DevError {  }),
            }
        }
        else {
            return Err(ErrorType::DevError{});
        }
    }

    fn consume_token(&mut self, expected_token: Token) -> Result<(), ErrorType> {
        if let Some(token) = self.input.get(self.current) {
            if *token == expected_token {
                self.current += 1;
                Ok(())
            } else {
                Err(ErrorType::DevError {  })
            }
        } else {
            Err(ErrorType::DevError {  })
        }
    }
}


/// TESTS ///
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_empty_input() { // passes
            let tokens: Vec<Token> = vec![];
            let ast: AST = Parser::parse(tokens).expect("Failed to parse");
            assert_eq!(ast.get_root().get_element(), SyntaxElement::ModuleExpression);
            assert!(ast.get_root().get_children().is_empty());
        }

        #[test]
        fn test_single_function_declaration() {
            let tokens: Vec<Token> = vec![
                Token::FUNCTION,
                Token::IDENTIFIER(vec!['m', 'y', '_', 'f', 'u', 'n', 'c']),
                Token::LPAREN,
                Token::RPAREN,
                Token::LBRACKET,
                Token::RBRACKET
            ];
            let ast: AST = Parser::parse(tokens).expect("Failed to parse");
            match ast.get_root().get_children().first().unwrap().get_element() {
                SyntaxElement::FunctionDeclaration { ref name, .. } => {
                    assert_eq!(name, "my_func");
                },
                _ => panic!("Expected FunctionDeclaration"),
            }
        }
    
        #[test]
        fn test_function_with_parameters_and_return_type() { 
            let tokens: Vec<Token> = vec![
                Token::FUNCTION,
                Token::IDENTIFIER(vec!['c', 'a', 'l', 'c', 'u', 'l', 'a', 't', 'e']),
                Token::LPAREN,
                Token::IDENTIFIER(vec!['x']),
                Token::COLON,
                Token::TINTEGER,
                Token::COMMA,
                Token::IDENTIFIER(vec!['y']),
                Token::COLON,
                Token::TINTEGER,
                Token::RPAREN,
                Token::COLON,
                Token::TBOOLEAN,
                Token::LBRACKET,
                Token::RBRACKET
            ];
            let ast: AST = Parser::parse(tokens).expect("Failed to parse");
            match ast.get_root().get_children().first().unwrap().get_element() {
                SyntaxElement::FunctionDeclaration { ref name, ref parameters, .. } => {
                    assert_eq!(name, "calculate");
                    assert_eq!(parameters.len(), 2);
                },
                _ => panic!("Expected FunctionDeclaration with parameters"),
            }
        }
    
    #[test]
    fn test_function_with_body() {
        let tokens = vec![
            Token::FUNCTION,
            Token::IDENTIFIER(vec!['t', 'e', 's', 't']),
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACKET,
            Token::LET,
            Token::IDENTIFIER(vec!['x']),
            Token::EQUAL,
            Token::INT(vec!['1']),
            Token::SEMICOLON,
            Token::RBRACKET,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::FunctionDeclaration { ref name, .. } => {
                assert_eq!(name, "test");
                assert!(!ast.get_root().get_children().is_empty());
            },
            _ => panic!("Expected FunctionDeclaration with body"),
        }
        }
    }

    #[test]
    fn test_literal_parsing() {
        let tokens: Vec<Token> = vec![
            Token::INT(vec!['1', '2', '3']),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::Literal(DataType::Integer, value) => assert_eq!(value, "123"),
            _ => panic!("Expected Literal"),
        }
    }

    #[test]
    fn test_binary_expression_parsing() {
        let tokens = vec![
            Token::INT(vec!['1']),
            Token::PLUS,
            Token::INT(vec!['2']),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::BinaryExpression { left, operator, right } => {
                assert_eq!(*left, ASTNode::new(SyntaxElement::Literal(DataType::Integer, "1".to_string())));
                assert_eq!(*right, ASTNode::new(SyntaxElement::Literal(DataType::Integer, "2".to_string())));
                assert_eq!(operator, "+");
            },
            _ => panic!("Expected BinaryExpression"),
        }
    }

    #[test]
    fn test_if_statement_parsing() {
        let tokens = vec![
            Token::IF,
            Token::LPAREN,
            Token::TRUE,
            Token::RPAREN,
            Token::LBRACKET,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACKET,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::IfStatement { condition, then_branch, else_branch } => {
                assert_eq!(*condition, ASTNode::new(SyntaxElement::Literal(DataType::Boolean, "true".to_string())));
                assert!(else_branch.is_none());
            },
            _ => panic!("Expected IfStatement"),
        }
    }
    #[test]
    fn test_assignment_parsing() {
        let tokens = vec![
            Token::IDENTIFIER(vec!['x']),
            Token::EQUAL,
            Token::INT(vec!['1']),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::Assignment { variable, value } => {
                assert_eq!(variable, "x");
                assert_eq!(*value, ASTNode::new(SyntaxElement::Literal(DataType::Integer, "1".to_string())));
            },
            _ => panic!("Expected Assignment"),
        }
    }
    #[test]
    fn test_initialization_parsing() {
        let tokens = vec![
            Token::LET,
            Token::IDENTIFIER(vec!['x']),
            Token::EQUAL,
            Token::INT(vec!['1']),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::Initialization { variable, value } => {
                assert_eq!(variable, "x");
                assert_eq!(*value, ASTNode::new(SyntaxElement::Literal(DataType::Integer, "1".to_string())));
            },
            _ => panic!("Expected Initialization"),
        }
    }
    #[test]
    fn test_for_loop_parsing() {
        let tokens = vec![
            Token::FOR,
            Token::LPAREN,
            Token::LET,
            Token::IDENTIFIER(vec!['i']),
            Token::EQUAL,
            Token::INT(vec!['0']),
            Token::SEMICOLON,
            Token::IDENTIFIER(vec!['i']),
            Token::LESSTHAN,
            Token::INT(vec!['1', '0']),
            Token::SEMICOLON,
            Token::IDENTIFIER(vec!['i']),
            Token::PLUSASSIGN,
            Token::INT(vec!['1']),
            Token::RPAREN,
            Token::LBRACKET,
            Token::BREAK,
            Token::SEMICOLON,
            Token::RBRACKET,
            Token::EOF,
        ];
        let ast: AST = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::ForLoop { initializer, condition, increment, body } => {
                match initializer.as_ref().unwrap().get_element() {
                    SyntaxElement::Initialization { variable, value } => {
                        assert_eq!(variable, "i");
                        match value.get_element() {
                            SyntaxElement::Literal(DataType::Integer, val) => {
                                assert_eq!(val, "0");
                            },
                            _ => panic!("Expected Literal for initializer value"),
                        }
                    },
                    _ => panic!("Initializer is not an Initialization SyntaxElement"),
                }
                match condition.get_element() {
                    SyntaxElement::BinaryExpression { left, operator, right } => {
                        match left.get_element() {
                            SyntaxElement::Variable(name) => assert_eq!(name, "i"),
                            _ => panic!("Expected Variable in condition's left expression"),
                        }
                        assert_eq!(operator, "<");
                        match right.get_element() {
                            SyntaxElement::Literal(DataType::Integer, val) => assert_eq!(val, "10"),
                            _ => panic!("Expected Literal in condition's right expression"),
                        }
                    },
                    _ => panic!("Condition is not a BinaryExpression"),
                }
                match increment.as_ref().unwrap().get_element() {
                    SyntaxElement::Assignment { variable, value } => {
                        assert_eq!(variable, "i");
                        match value.get_element() {
                            SyntaxElement::Literal(DataType::Integer, val) => assert_eq!(val, "1"),
                            _ => panic!("Expected Literal for increment value"),
                        }
                    },
                    _ => panic!("Increment is not an Assignment"),
                }
                let body_nodes: &Vec<ASTNode> = &(*body);
                match body_nodes.first() {
                    Some(node) => {
                        match node.get_element() {
                            SyntaxElement::Break => {},
                            _ => panic!("Body's first node is not a BreakStatement"),
                        }
                    },
                    None => panic!("Body is empty"),
                }
            },
            _ => panic!("Expected ForLoop"),
        }
    }
    
    
    

    #[test]
    fn test_while_loop_parsing() {
        let tokens = vec![
            Token::WHILE,
            Token::LPAREN,
            Token::TRUE,
            Token::RPAREN,
            Token::LBRACKET,
            Token::BREAK,
            Token::SEMICOLON,
            Token::RBRACKET,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::WhileLoop { condition, body } => {
                match condition.get_element() {
                    SyntaxElement::Literal(DataType::Boolean, val) => {
                        assert_eq!(val, "true");
                    },
                    _ => panic!("Expected Boolean Literal in condition"),
                }
    
                let body_nodes = &(*body);
                match body_nodes.first() {
                    Some(node) => {
                        match node.get_element() {
                            SyntaxElement::Break => {},
                            _ => panic!("Body's first node is not a BreakStatement"),
                        }
                    },
                    None => panic!("Body is empty"),
                }
            },
            _ => panic!("Expected WhileLoop"),
        }
    }

    #[test]
    fn test_do_while_loop_parsing() {
        let tokens = vec![
            Token::DO,
            Token::LBRACKET,
            Token::BREAK,
            Token::SEMICOLON,
            Token::RBRACKET,
            Token::WHILE,
            Token::LPAREN,
            Token::TRUE,
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::DoWhileLoop { body, condition } => {
                let body_nodes = &(*body);
                match body_nodes.first() {
                    Some(node) => {
                        match node.get_element() {
                            SyntaxElement::Break => {},
                            _ => panic!("Body's first node is not a BreakStatement"),
                        }
                    },
                    None => panic!("Body is empty"),
                }
    
                match condition.get_element() {
                    SyntaxElement::Literal(DataType::Boolean, val) => {
                        assert_eq!(val, "true");
                    },
                    _ => panic!("Expected Boolean Literal in condition"),
                }
            },
            _ => panic!("Expected DoWhileLoop"),
        }
    }
    
    

    #[test]
    fn test_continue_statement_parsing() {
        let tokens = vec![
            Token::CONTINUE,
            Token::SEMICOLON,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::Continue => {  },
            _ => panic!("Expected ContinueStatement"),
        }
    }

    #[test]
    fn test_match_statement_parsing() {
        let tokens = vec![
            Token::MATCH,
            Token::LPAREN,
            Token::IDENTIFIER(vec!['x']),
            Token::RPAREN,
            Token::LBRACKET,
            Token::RBRACKET,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::MatchStatement { to_match, arms } => {
                match to_match.get_element() {
                    SyntaxElement::Variable(name) => assert_eq!(name, "x"),
                    _ => panic!("Expected Variable in to_match"),
                }
                assert!(arms.is_empty(), "Expected no match arms");
            },
            _ => panic!("Expected MatchStatement"),
        }
    }
    

    #[test]
    fn test_function_call_parsing() {
        let tokens = vec![
            Token::IDENTIFIER(vec!['f', 'u', 'n', 'c']),
            Token::LPAREN,
            Token::IDENTIFIER(vec!['a']),
            Token::COMMA,
            Token::IDENTIFIER(vec!['b']),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];
        let ast = Parser::parse(tokens).expect("Failed to parse");
        match ast.get_root().get_children().first().unwrap().get_element() {
            SyntaxElement::FunctionCall { name, arguments } => {
                assert_eq!(name, "func");
                assert_eq!(arguments.len(), 2, "Expected two arguments");
    
                match arguments[0].get_element() {
                    SyntaxElement::Variable(arg_name) => assert_eq!(arg_name, "a"),
                    _ => panic!("Expected first argument to be a variable 'a'"),
                }
                match arguments[1].get_element() {
                    SyntaxElement::Variable(arg_name) => assert_eq!(arg_name, "b"),
                    _ => panic!("Expected second argument to be a variable 'b'"),
                }
            },
            _ => panic!("Expected FunctionCall"),
        }
    }
    
}
