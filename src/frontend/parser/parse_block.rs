use crate::frontend::{ 
    utils::error::ErrorType,
    lexer::token::Token,
    ast::{
        ast_struct::ASTNode, syntax_element::SyntaxElement, 
    },
    parser::parser_core::Parser,
};

impl Parser {
    /// Creates the children of an expression that changes scope. Used for all scope changing expressions except structs and enums
    pub fn parse_block(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        self.consume_token(Token::LBRACKET)?; 
        let mut block_exp = ASTNode::new(SyntaxElement::BlockExpression);

        let mut children: Vec<ASTNode> = Vec::new();

        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACKET) {
            match self.parse_router() {
                Ok(Some(expr_node)) => {
                    children.push(expr_node);
                }
                Ok(None) => {}
                _ => panic!("parse_block parse problem")
            }
        }
        if self.get_input().get(self.get_current()) == Some(&Token::RBRACKET) {
            self.consume_token(Token::RBRACKET)?;
        } else {
            panic!("failed to reach stop token")
        }
        block_exp.add_children(children);

        Ok(Some(block_exp))
    }

    /// Parses the initalization of a variable
    pub fn parse_initialization(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
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
    
                    let value: ASTNode = match self.parse_router() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("Failed to find initialization value");
                        }
                        Err(_) => {
                            panic!("Failed to parse initialization value");
                        }
                    };   

                    let mut initialization_node: ASTNode = ASTNode::new(SyntaxElement::Initialization);

                    let identifer_node: ASTNode = ASTNode::new(SyntaxElement::Identifier(variable_name));
                    initialization_node.add_child(identifer_node);

                    let type_node: ASTNode = ASTNode::new(SyntaxElement::Type(data_type));
                    initialization_node.add_child(type_node);
                    initialization_node.add_child(value);

                    Ok(Some(initialization_node))
                },
                _ => panic!("how'd you reach this let panic? this is a tricky one"),
            }
        } else {
            panic!("parse_initilization")
        }
    }

    /// Parses a match statement
    pub fn parse_match_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::MATCH) => {
                    self.consume_token(Token::MATCH)?;

                    let value: ASTNode = match self.parse_router() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("Assignment value is missing");
                        }
                        Err(_) => {
                            panic!("Failed to parse assignment value");
                        }
                    };

                    let block_exp: ASTNode = self.parse_match_arms()?;

                    let mut match_node: ASTNode = ASTNode::new(SyntaxElement::MatchStatement);
                    match_node.add_child(value);
                    match_node.add_child(block_exp);

                    return Ok(Some(match_node));
                }
                _ => panic!("problem match parsing"),
            }
        } panic!("problem matach parsing 2")
    }

    /// Parses the match arms of a match statement
    pub fn parse_match_arms(&mut self) -> Result<ASTNode, Vec<ErrorType>> { 
        let mut arms: Vec<ASTNode> = Vec::new();

        let mut block_exp: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

        self.consume_token(Token::LBRACKET)?;

        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACKET) {
            let variant: ASTNode = match self.parse_router() {
                Ok(Some(value)) => value, 
                Ok(None) => {
                    panic!("Assignment value is missing");
                }
                Err(_) => {
                    panic!("Failed to parse assignment value");
                }
            };

            self.consume_token(Token::ARROW)?;  

            let action: ASTNode = match self.parse_router() {
                Ok(Some(value)) => value, 
                Ok(None) => {
                    panic!("Assignment value is missing");
                }
                Err(_) => {
                    panic!("Failed to parse assignment value");
                }
            };

            let mut variant_node: ASTNode = ASTNode::new(SyntaxElement::Variant);
            variant_node.add_child(variant);

            let mut action_node: ASTNode = ASTNode::new(SyntaxElement::Action);
            action_node.add_child(action);

            let mut arm_node: ASTNode = ASTNode::new(SyntaxElement::MatchArm);
            arm_node.add_child(variant_node);
            arm_node.add_child(action_node);

            arms.push(arm_node);
            
            if let Some(Token::COMMA) = self.get_input().get(self.get_current()) {
                self.consume_token(Token::COMMA)?;
            } else if self.get_input().get(self.get_current()) != Some(&Token::RBRACKET) {
                panic!("match arms problem")
            }
        }
        self.consume_token(Token::RBRACKET)?;

        block_exp.add_children(arms);
        Ok(block_exp)
    }

    /// Parses an if statement
    pub fn parse_if_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::IF) => {
                    self.consume_token(Token::IF)?;
                    self.consume_token(Token::LPAREN)?;
                    
                    let condition: ASTNode = match self.parse_router() {
                        Ok(Some(value)) => {value}
                        _ => panic!("if statement panic")
                    };
                    self.consume_token(Token::RPAREN)?;

                    let mut if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);
                    if_node.add_child(condition);

                    match self.parse_router() {
                        Ok(Some(node)) => {
                            if_node.add_child(node);
                        }
                        _ => {
                            panic!("Missing then branch")
                        }
                    }


                    if let Some(Token::ELSE) = self.get_input().get(self.get_current()) {
                        self.consume_token(Token::ELSE)?;
                        match self.parse_router() {
                            Ok(Some(node)) => {
                                if_node.add_child(node);
                            }
                            _ => {
                                panic!("Missing else block exp")
                            }
                        }
                    };

                    return Ok(Some(if_node));
                }
                _ => panic!("Problem parsing in if statement"),
            }
        } panic!("Problem parsing if statement 2")
    }

    /// Parses a for loop
    pub fn parse_for_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() >= self.get_input().len() {
            panic!("Problem parsing for loop: input exhausted");
        }
    
        if let Some(Token::FOR) = self.get_input().get(self.get_current()) {
            self.consume_token(Token::FOR)?;
            self.consume_token(Token::LPAREN)?;
    
            let initializer_node = if self.get_input().get(self.get_current()) == Some(&Token::LET) {
                if let Ok(Some(node)) = self.parse_router() {
                    let mut init_node = ASTNode::new(SyntaxElement::LoopInitializer);
                    init_node.add_child(node);
                    Some(init_node)
                } else {
                    panic!("Expected initializer after 'let'");
                }
            } else {
                None
            };
    
            let condition_node = if self.get_input().get(self.get_current()) != Some(&Token::RPAREN) {
                if let Ok(Some(node)) = self.parse_router() {
                    let mut cond_node = ASTNode::new(SyntaxElement::Condition);
                    cond_node.add_child(node);
                    Some(cond_node)
                } else {
                    panic!("Expected condition in for loop");
                }
            } else {
                None
            };
    
            let increment_node = if self.get_input().get(self.get_current()) != Some(&Token::RPAREN) {
                if let Ok(Some(node)) = self.parse_router() {
                    let mut inc_node = ASTNode::new(SyntaxElement::LoopIncrement);
                    inc_node.add_child(node);
                    Some(inc_node)
                } else {
                    None
                }
            } else {
                None
            };
    
            self.consume_token(Token::RPAREN)?;
    
            let body = if let Ok(Some(body)) = self.parse_block() {
                body
            } else {
                panic!("Expected for loop body");
            };
    
            let mut for_node = ASTNode::new(SyntaxElement::ForLoop);
            if let Some(init_node) = initializer_node {
                for_node.add_child(init_node);
            }
            if let Some(cond_node) = condition_node {
                for_node.add_child(cond_node);
            }
            if let Some(inc_node) = increment_node {
                for_node.add_child(inc_node);
            }
            for_node.add_child(body);
    
            Ok(Some(for_node))
        } else {
            panic!("Expected 'for' at the beginning of a for loop");
        }
    }
    

    /// Parses a while loop
    pub fn parse_while_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::WHILE) => {
                    self.consume_token(Token::WHILE)?;
                    self.consume_token(Token::LPAREN)?;

                    let condition: ASTNode = match self.parse_router() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("while1");
                        }
                        Err(_) => {
                            panic!("while2");
                        }
                    };
                    let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
                    condition_node.add_child(condition);

                    self.consume_token(Token::RPAREN)?;

                    match self.parse_block() {
                        Ok(Some(node)) => {
                            let mut while_node: ASTNode = ASTNode::new(SyntaxElement::WhileLoop);
                            while_node.add_child(condition_node);
                            while_node.add_child(node);
        
                            return Ok(Some(while_node));
                        }
                        _ => {
                            panic!("Missing while block")
                        }
                    }
                } 
                _ => panic!("problem while loop parsing"),
            }
        } panic!("problem while loop parsing 2")
    }

    /// Parses a do while loop
    pub fn parse_do_while_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::DO) => {
                    self.consume_token(Token::DO)?;

                    match self.parse_block() {
                        Ok(Some(body)) => {
                            self.consume_token(Token::WHILE)?;
                            self.consume_token(Token::LPAREN)?;
        
                            let condition: ASTNode = match self.parse_router() {
                                Ok(Some(node)) => node,
                                _ => panic!("missing while condition"), 
                            };
        
                            let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
                            condition_node.add_child(condition);
        
                            self.consume_token(Token::RPAREN)?;
        
                            let mut do_while_node = ASTNode::new(SyntaxElement::DoWhileLoop);
                            do_while_node.add_child(body);
                            do_while_node.add_child(condition_node);
        
                            return Ok(Some(do_while_node));
                        }
                        _ => {
                            panic!("missing do block")
                        }
                    }
                }
                _ => panic!("problem do_while parsing"),
            }
        } panic!("problem do_while parsing 2")
    }
}