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
    /// Entry point to the main parsing logic. Serves as a way to match the current token type to the file/expression we want to parse
    /// All other parsing files have their own version of this, but this is the main one
    pub fn parse_element(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                // top level expressions
                Some(Token::FUNCTION) | 
                Some(Token::STRUCT) | 
                Some(Token::ENUM) => return self.parse_top_level(),

                // statements
                Some(Token::IF) |
                Some(Token::FOR) |
                Some(Token::DO) | 
                Some(Token::WHILE) |
                Some(Token::MATCH) |
                Some(Token::LET) => return self.parse_statement(),

                // binary operations
                Some(Token::PLUS) | 
                Some(Token::MINUS) | 
                Some(Token::MULTIPLY) | 
                Some(Token::DIVIDE) => return self.parse_binary_expression(),

                // unary operations
                Some(Token::LOGICALNOT) => return self.parse_unary_expression(), 

                // base elements like primitives, identifiers, and protected keywords
                Some(Token::INT(_)) |
                Some(Token::TRUE) | 
                Some(Token::FALSE) |
                Some(Token::IDENTIFIER(_)) |
                Some(Token::BREAK) |
                Some(Token::CONTINUE) |
                Some(Token::RETURN) |
                Some(Token::SEMICOLON) |
                Some(Token::EOF) => return self.parse_base(),
                _ => panic!("Are you sure this is an expression: {:?}", self.get_input().get(self.get_current())),

            }
        } else {
            panic!("You hooligan. You're out of tokens")
        }
    }

    /// Parses a unary expression
    fn parse_unary_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::MINUS) | Some(Token::LOGICALNOT) => {                    
                    let operator = match self.get_input().get(self.get_current()) {
                        Some(Token::MINUS) => {
                            self.consume_token(Token::MINUS)?;
                            "-"
                        },
                        Some(Token::LOGICALNOT) => {
                            self.consume_token(Token::LOGICALNOT)?;
                            "!"
                        },
                        _ => panic!("This was a hard panic to hit"),
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
                    return Ok(Some(ASTNode::new(SyntaxElement::UnaryExpression {
                        operator,
                        operand: Box::new(operand),
                    })));
                },
                _ => panic!("Is this part of a unary expression?{:?}", self.get_input().get(self.get_current())),
            }
        } panic!("how'd you hit this?")
    }
    
    /// Parses a variable reassignment (variable is already initialized)
    /// format: x = 10
    pub fn parse_assignment(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        let variable_name: String = 
            if let Some(Token::IDENTIFIER(name_chars)) = self.get_input().get(self.get_current()) {
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
        }; // value consumed with parse_element()
        
        Ok(Some(ASTNode::new(SyntaxElement::Assignment {
            variable: variable_name,
            value: Box::new(value),
        })))
        
    }
    
    /// Parses a binary expression
    /// format: expr operator expr
    pub fn parse_binary_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        let lhs: Option<ASTNode> = self.parse_base()?;
        if let Some(lhs_unwrapped) = lhs {
            let mut expr: Option<ASTNode> = None;
            while let Some(op_token) = self.get_input().get(self.get_current()) {
                if let Some(&precedence) = binop_precedence().get(&self.operator_to_char(op_token)) {
                    self.consume_token(op_token.clone())?;
    
                    let mut rhs: Option<ASTNode> = self.parse_base()?; // i think this is a bug
                    // actually the whole function idk this needs work
                    let operator: String = self.operator_to_char(op_token).to_string();
    
                    while let Some(_) = self.get_next_operator_with_higher_precedence(precedence.try_into().unwrap()) {
                        rhs = match self.parse_binary_expression()? {
                            Some(r) => Some(r),
                            None => break,
                        };
                    }
    
                    expr = match rhs {
                        Some(right_node) => {
                            Some(ASTNode::new(SyntaxElement::BinaryExpression {
                                left: Box::new(lhs_unwrapped.clone()),
                                operator,
                                right: Box::new(right_node),
                            }))
                        },
                        None => None,
                    };
                } else {
                    break;
                }
            }
            Ok(expr)
        } else {
            Ok(None)
        }
    }

    fn get_next_operator_with_higher_precedence(&mut self, current_precedence: usize) -> Option<Token> {
        if let Some(next_op) = self.get_input().get(self.get_current()) {
            if let Some(&next_precedence) = binop_precedence().get(&self.operator_to_char(next_op)) {
                if current_precedence < next_precedence.try_into().unwrap() {
                    return Some(next_op.clone());
                }
            }
        }
        None
    }
}