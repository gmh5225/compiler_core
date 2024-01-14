use crate::frontend::{ 
    utils::error::ErrorType,
    syntax::token::Token,
    ast::{
        ast_struct::ASTNode, syntax_element::{
            SyntaxElement, MatchArm
        }, 
    },
    parser::parser_core::Parser,
};

impl<'a> Parser<'a> {
    pub fn scope_changing_until(&mut self, stop_token: Token) -> Result<Vec<ASTNode>, Vec<ErrorType>> {
        let mut children: Vec<ASTNode> = Vec::new();
        self.consume_token(Token::LBRACKET)?;

        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&stop_token) {
            match self.get_input().get(self.get_current()) {
                Some(Token::IF) => {
                    match self.parse_if_statement(){
                        Ok(if_node) => {
                            children.push(if_node);
                        }
                        _ => panic!("scope_changing_until if_statement panic")
                    }   
                },
                Some(Token::FOR) => {
                    match self.parse_for_loop(){
                        Ok(for_node) => {
                            children.push(for_node);
                        }
                        _ => panic!("scope_changing_until for_loop panic")
                    }
                    
                },
                Some(Token::WHILE) => {
                    match self.parse_while_loop(){
                        Ok(while_node) => {
                            children.push(while_node);
                        }
                        _ => panic!("scope_changing_until while panic")
                    }
                },
                Some(Token::DO) => {
                    match self.parse_do_while_loop(){
                        Ok(do_while_node) => {
                            children.push(do_while_node);
                        }
                        _ => panic!("scope_changing_until do_while panic")
                    }
                },
                Some(Token::MATCH) => {
                    match self.parse_match_statement(){
                        Ok(match_node) => {
                            children.push(match_node);
                        }
                        _ => panic!("scope_changing_until match panic")
                    }
                },
                _ => {
                    match self.parse_element() {
                        Ok(Some(expr_node)) => {
                            children.push(expr_node);
                        }
                        Ok(None) => {}
                        _ => panic!("scope_changing_until final parse")
                    }
                }
            }
        }
        if self.get_input().get(self.get_current()) == Some(&stop_token) {
            self.consume_token(stop_token)?;
        }
        Ok(children)
    }

    pub fn parse_match_arms(&mut self) -> Result<Vec<MatchArm>, Vec<ErrorType>> {
        let mut arms: Vec<MatchArm> = Vec::new();

        self.consume_token(Token::LBRACE)?;

        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
            let variant: ASTNode = match self.parse_element() {
                Ok(Some(value)) => value, 
                Ok(None) => {
                    panic!("Assignment value is missing");
                }
                Err(_) => {
                    panic!("Failed to parse assignment value");
                }
            };

            self.consume_token(Token::ARROW)?;  

            let action: ASTNode = match self.parse_element() {
                Ok(Some(value)) => value, 
                Ok(None) => {
                    panic!("Assignment value is missing");
                }
                Err(_) => {
                    panic!("Failed to parse assignment value");
                }
            };

            arms.push(MatchArm::new(variant, action));

            if let Some(Token::COMMA) = self.get_input().get(self.get_current()) {
                self.consume_token(Token::COMMA)?;
            } else if self.get_input().get(self.get_current()) != Some(&Token::RBRACE) {
                return Err(vec![ErrorType::DevError {  } ])
            }
        }

        self.consume_token(Token::RBRACE)?;

        Ok(arms)
    }

    pub fn parse_if_statement(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::IF) => {
                    self.consume_token(Token::IF)?;
                    self.consume_token(Token::LPAREN)?;
                    
                    let condition: ASTNode = match self.parse_element() {
                        Ok(Some(value)) => {value}
                        _ => panic!("if statement panic")
                    };
                    self.consume_token(Token::RPAREN)?;
                    let nodes: Vec<ASTNode> = self.scope_changing_until(Token::RBRACKET)?;
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

    pub fn parse_for_loop(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::FOR) => {
                    self.consume_token(Token::FOR)?;
                    let value: ASTNode = match self.parse_element() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("for loop 1");
                        }
                        Err(_) => {
                            panic!("for lop 7");
                        }
                    };
                    let initializer = if let Some(Token::LET) = self.get_input().get(self.get_current()) {
                        Some(Box::new(value))
                    } else {
                        None
                    };
                    let value: ASTNode = match self.parse_element() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("for loop 3");
                        }
                        Err(_) => {
                            panic!("for loop 4");
                        }
                    };
                    let condition: Box<ASTNode> = Box::new(value);
                    let value: ASTNode = match self.parse_element() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("for loop 6");
                        }
                        Err(_) => {
                            panic!("for loop 5");
                        }
                    };
                    let increment: Option<Box<ASTNode>> = if let Some(Token::SEMICOLON) = self.get_input().get(self.get_current()) {
                        self.consume_token(Token::SEMICOLON)?;
                        Some(Box::new(value))
                    } else {
                        None
                    };
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing_until(Token::RBRACKET)?);

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

    pub fn parse_while_loop(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::WHILE) => {
                    self.consume_token(Token::WHILE)?;
                    self.consume_token(Token::LPAREN)?;
                    let condition: Box<ASTNode> = Box::new(match self.parse_element() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("while1");
                        }
                        Err(_) => {
                            panic!("while2");
                        }
                    });
                    self.consume_token(Token::RPAREN)?;
                    self.consume_token(Token::LBRACKET)?;
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing_until(Token::RBRACKET)?);

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

    pub fn parse_do_while_loop(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::DO) => {
                    self.consume_token(Token::DO)?;
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing_until(Token::WHILE)?);
                    let value: ASTNode = match self.parse_element() {
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

    pub fn parse_match_statement(&mut self) -> Result<ASTNode, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::MATCH) => {
                    self.consume_token(Token::MATCH)?;
                    let value: ASTNode = match self.parse_element() {
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
}