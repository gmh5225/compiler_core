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
    /// Parses statements
    pub fn parse_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::IF) => return self.parse_if_statement(),  
                Some(Token::FOR) => return self.parse_for_loop(),  
                Some(Token::DO) => return self.parse_do_while_loop(),  
                Some(Token::WHILE) => return self.parse_while_loop(),
                Some(Token::MATCH) => return self.parse_match_statement(),
                Some(Token::LET) => return self.parse_initialization(),
                _ => panic!("Are you sure this is an statement: {:?}", self.get_input().get(self.get_current())),
            }
        } else {
            panic!("parse_statement panic")
        }
    }

    /// Creates the children of an expression that changes scope. Used for all scope changing expressions except structs and enums
    pub fn scope_changing_until(&mut self, stop_token: Token) -> Result<Vec<ASTNode>, Vec<ErrorType>> {
        let mut children: Vec<ASTNode> = Vec::new();
        self.consume_token(Token::LBRACKET)?; // need to change this to a passed in start_token

        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&stop_token) {
            match self.parse_element() {
                Ok(Some(expr_node)) => {
                    children.push(expr_node);
                }
                Ok(None) => {}
                _ => panic!("scope_changing_until parse problem")
            }
        }
        if self.get_input().get(self.get_current()) == Some(&stop_token) {
            self.consume_token(stop_token)?;
        } else {
            panic!("failed to reach stop token")
        }
        Ok(children)
    }

    /// Parses the initalization of a variable
    /// format of initalization of variable currently: let a: bool = true;
    fn parse_initialization(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::LET) => {
                    self.consume_token(Token::LET)?;
    
                    let variable_name = if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
                        self.consume_token(Token::IDENTIFIER(name_chars.clone()))?;
                        name_chars.iter().collect::<String>()
                    } else {
                        panic!("wheres my variable name?")
                    };
                    self.consume_token(Token::COLON)?;
                
                    let data_type = self.consume_type()?;
                    if let Some(Token::EQUAL) = self.get_input().get(self.get_current()) {
                        self.consume_token(Token::EQUAL)?;
                    } else {
                        panic!("wheres my assignment operator?")
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
                    Ok(Some(ASTNode::new(SyntaxElement::Initialization {
                        variable: variable_name,
                        data_type,
                        value: Box::new(value),
                    })))
                },
                _ => panic!("how'd you reach this let panic? this is a tricky one"),
            }
        } else {
            panic!("parse_initilization")
        }
    }

    /// Parses a match statement
    /// current match statement format: match foo {a => action, b => actionb}
    fn parse_match_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
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
                    return Ok(Some(match_node));
                }
                _ => panic!("problem match parsing"),
            }
        } panic!("problem matach parsing 2")
    }

    /// Parses the match arms of a match statement
    fn parse_match_arms(&mut self) -> Result<Vec<MatchArm>, Vec<ErrorType>> {
        let mut arms: Vec<MatchArm> = Vec::new();

        self.consume_token(Token::LBRACKET)?;

        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACKET) {
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
            } else if self.get_input().get(self.get_current()) != Some(&Token::RBRACKET) {
                panic!("match arms problem")
            }
        }
        self.consume_token(Token::RBRACKET)?;

        Ok(arms)
    }

    /// Parses an if statement
    /// current if statement form: if(condition) {}
    fn parse_if_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
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

                    let then_branch: Vec<ASTNode> = self.scope_changing_until(Token::RBRACKET)?;
                    let else_branch: Option<Box<Vec<ASTNode>>> = match self.get_input().get(self.get_current()) {
                        Some(Token::LBRACE) => {
                            match self.scope_changing_until(Token::RBRACE) {
                                Ok(nodes) => Some(Box::new(nodes)),
                                _ => panic!("if statement panic")
                            }
                        }
                        _ => None
                    };

                    let if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement { 
                        condition: Box::new(condition), 
                        then_branch: Box::new(then_branch), 
                        else_branch,
                    });
                    return Ok(Some(if_node));
                }
                _ => panic!("Problem parsing in if statement"),
            }
        } panic!("Problem parsing if statement 2")
    }

    /// Parses a for loop
    /// current format: for (let i: int = 0; i < 1; i += 1;) {}
    fn parse_for_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::FOR) => {
                    self.consume_token(Token::FOR)?;
                    self.consume_token(Token::LPAREN)?;
        
                    // let initializer: Option<Box<ASTNode>> = if self.get_input().get(self.get_current()) != Some(&Token::RPAREN) {
                    //     match self.parse_element() {
                    //         Ok(node) => {
                    //             self.consume_token(Token::SEMICOLON)?;
                    //             node.map(Box::new)

                    //         },
                    //         _ => panic!("for loop")
                    //     }
                    // } else {
                    //     None
                    // };
                    let condition: Box<ASTNode> = if self.get_input().get(self.get_current()) != Some(&Token::RPAREN) {
                        match self.parse_element() {
                            Ok(Some(node)) => {
                                self.consume_token(Token::SEMICOLON)?;
                                Box::new(node)
                            },
                            _ => panic!("for loop condition?"),
                        }
                    } else {
                        panic!("For loop 9");
                    };
                    
                    
                    let increment: Option<Box<ASTNode>> = if self.get_input().get(self.get_current()) != Some(&Token::RPAREN) {
                        match self.parse_element() {
                            Ok(node) => {
                                self.consume_token(Token::SEMICOLON)?;
                                node.map(Box::new)
                            }
                            _ => panic!("for loop 2")
                        }
                    } else {
                        None
                    };
                    
                    self.consume_token(Token::RPAREN)?;
        
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing_until(Token::RBRACKET)?);

                    let for_node: ASTNode = ASTNode::new(SyntaxElement::ForLoop {
                        // initializer, 
                        initializer: None,
                        condition,   
                        increment,
                        body,
                    });
        
                    return Ok(Some(for_node));
                }
                _ => panic!("Problem parsing in for loop"),
            }
        } panic!("Problem parsing for loop 2")
    }

    /// Parses a while loop
    /// Current format: while(condition) {}
    fn parse_while_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
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
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing_until(Token::RBRACKET)?);

                    let while_node = ASTNode::new(SyntaxElement::WhileLoop {
                        condition,
                        body,
                    });
                    return Ok(Some(while_node));
                } 
                _ => panic!("problem while loop parsing"),
            }
        } panic!("problem while loop parsing 2")
    }

    /// Parses a do while loop
    /// current format: do{} while()
    fn parse_do_while_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::DO) => {
                    self.consume_token(Token::DO)?;
                    let body: Box<Vec<ASTNode>> = Box::new(self.scope_changing_until(Token::RBRACKET)?);
                    self.consume_token(Token::WHILE)?;
                    self.consume_token(Token::LPAREN)?;
                    let value: ASTNode = match self.parse_element() {
                        Ok(Some(node_option)) => node_option,
                        _ => panic!("missing while condition"), 
                    };
                    let condition: Box<ASTNode> = Box::new(value);
                    self.consume_token(Token::RPAREN)?;
                    let do_while_node = ASTNode::new(SyntaxElement::DoWhileLoop {
                        body,
                        condition,
                    });
                    return Ok(Some(do_while_node));
                }
                _ => panic!("problem do_while parsing"),
            }
        } panic!("problem do_while parsing 2")
    }
}