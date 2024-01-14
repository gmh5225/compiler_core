use crate::frontend::{ 
    utils::{error::ErrorType, binop_precedence::binop_precedence},
    syntax::token::Token,
    ast::{
        ast_struct::ASTNode, 
        syntax_element::SyntaxElement,
    },
    parser::parser_core::Parser,
};

impl<'a> Parser<'a> {
    pub fn parse_element(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
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
                Some(Token::LOGICALNOT) => return Ok(Some(self.parse_unary_expression()?)), // add other unary expressions here
                Some(Token::INT(_)) |
                Some(Token::TRUE) | 
                Some(Token::FALSE) => return Ok(Some(self.parse_primitive()?)),
                Some(Token::IDENTIFIER(fun_call)) => {
                    match self.next_token() { // check next token
                        Some(token) => { // if token
                            match token {
                                Token::LPAREN => { // either left parenthesis (function call)
                                    self.consume_token(Token::LPAREN)?;
                                    match self.get_input().get(self.get_current()) {
                                        Some(Token::IDENTIFIER(name_chars)) => {
                                            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
                                            let name: String = name_chars.iter().collect();
                                            let arguments = self.parse_function_call_params();
                                            return Ok(Some(ASTNode::new(SyntaxElement::FunctionCall { name, arguments })))    
                                            
                                        },
                                        Some(Token::RPAREN) => {
                                            self.consume_token(Token::IDENTIFIER(fun_call.clone()))?;
                                            let name: String = fun_call.iter().collect();
                                            Ok(Some(ASTNode::new(SyntaxElement::FunctionCall { name, arguments: Vec::new() })))
                                        }
                                        _ => panic!("{:?}", self.get_input().get(self.get_current())),
                                    }
                                }
                                _ => return Ok(Some(self.parse_assignment()?)) // or assignment

                            }

                        }
                        _ => panic!("next_token problem")
                    }
                }
                Some(Token::BREAK) => return Ok(Some(ASTNode::new(SyntaxElement::Break))),
                Some(Token::CONTINUE) => return Ok(Some(ASTNode::new(SyntaxElement::Continue))),
                Some(Token::RETURN) => {
                    self.consume_token(Token::RETURN)?;
                    let value = match self.parse_element() {
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
                _ => panic!("Are you sure this is an expression: {:?}", self.get_input().get(self.get_current())),

            }
        } else {
            panic!("parse expression 2")
        }
    }

    fn parse_unary_expression(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::MINUS) | Some(Token::LOGICALNOT) => {
                    // Extract the operator
                    
                    let operator = match self.get_input().get(self.get_current()) {
                        Some(Token::MINUS) => {
                            self.consume_token(Token::MINUS)?;
                            "-"
                        },
                        Some(Token::LOGICALNOT) => {
                            self.consume_token(Token::LOGICALNOT)?;
                            "!"
                        },
                        _ => return Err(vec![ErrorType::DevError{}]),
                    }.to_string();
        
                    let operand: ASTNode = match self.parse_element() {
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
    fn parse_initialization(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::LET) => {
                    self.consume_token(Token::LET)?;
    
                    let variable_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
                        self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
                        name_chars.iter().collect::<String>()
                    } else {
                        return Err(vec![ErrorType::DevError {  }]);
                    };
    
                    if let Some(Token::EQUAL) = self.get_input().get(self.get_current()) {
                        self.consume_token(Token::EQUAL)?;
                    } else {
                        return Err(vec![ErrorType::DevError{}]);
                    }
    
                    let value: ASTNode = match self.parse_element() {
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
    
    fn parse_function_call_params(&mut self) -> Vec<ASTNode> {
        if self.get_current() >= self.get_input().len() {
            panic!("function_call_params panic 1");
        }
    
        let mut parameters = Vec::new();
    
        while self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::RPAREN) => {
                    self.consume_token(Token::RPAREN);
                    return parameters;
                },
                Some(Token::IDENTIFIER(name_chars)) => {
                    self.consume_token(Token::IDENTIFIER(name_chars.clone()));
                    let param_name = name_chars.iter().collect::<String>();
                    parameters.push(ASTNode::new(SyntaxElement::Variable(param_name)));
    
                    match self.get_input().get(self.get_current()) {
                        Some(Token::COMMA) => match self.consume_token(Token::COMMA) {
                            Ok(_) => continue,
                            Err(_) => panic!("function_call_params panic 5"),
                        },                        
                        Some(Token::RPAREN) => {
                            self.consume_token(Token::RPAREN);
                            return parameters;
                        },
                        _ => panic!("function_call_params panic 3"),
                    }
                },
                Some(Token::COMMA) => {
                    self.consume_token(Token::COMMA);
                    continue;
                }
                _ => panic!("function_call_params panic 4"),
            }
        }
    
        panic!("function_call_params panic 5");
    }
    
    

    fn parse_assignment(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        let variable_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
            name_chars.iter().collect::<String>()
        } else {
            panic!("parse_assignment 1")
        };
    
        if let Some(Token::EQUAL) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::EQUAL)?;
        } else {
            panic!("parse_assignment 2")
        }
    
        let value: ASTNode = match self.parse_element() {
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
    
        while let Some(op_token) = self.get_input().get(self.get_current()) {
            if let Some(&precedence) = binop_precedence().get(&&self.operator_to_char(op_token)) { 
                self.consume_token(op_token.clone())?;
    
                let mut right = self.parse_primitive()?;
                while let Some(next_op) = self.get_input().get(self.get_current()) {
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
}