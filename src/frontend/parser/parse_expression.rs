use crate::frontend::{ 
    utils::error::ErrorType,
    lexer::token::Token,
    ast::{
        ast_struct::ASTNode, 
        syntax_element::SyntaxElement,
    },
    parser::parser_core::Parser,
    parser::binop_precedence::binop_precedence,
};

impl Parser {
    /// Parses a unary expression
    pub fn parse_unary_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::MINUS) | Some(Token::LOGICALNOT) => {                    
                    let operator: &str = match self.get_input().get(self.get_current()) {
                        Some(Token::MINUS) => {
                            self.consume_token(Token::MINUS)?;
                            "-"
                        },
                        Some(Token::LOGICALNOT) => {
                            self.consume_token(Token::LOGICALNOT)?;
                            "!"
                        },
                        _ => panic!("This was a hard panic to hit"),
                    };

                    let operator_node: ASTNode = ASTNode::new(SyntaxElement::Operator(operator));

                    let operand_node: ASTNode = match self.parse_router() {
                        Ok(Some(value)) => value, 
                        Ok(None) => {
                            panic!("unary is missing");
                        }
                        Err(_) => {
                            panic!("Failed to parse unary value");
                        }
                    };
                    
                    let mut unary_exp: ASTNode = ASTNode::new(SyntaxElement::UnaryExpression);
                    unary_exp.add_child(operator_node);
                    unary_exp.add_child(operand_node);

                    return Ok(Some(unary_exp));
                },
                _ => panic!("Is this part of a unary expression?{:?}", self.get_input().get(self.get_current())),
            }
        } panic!("how'd you hit this?")
    }
    
    /// Parses a variable reassignment
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
    
        let identifier_node: ASTNode = ASTNode::new(SyntaxElement::Identifier(variable_name));

        let value_node: ASTNode = match self.parse_router() { 
            Ok(Some(value)) => value, 
            Ok(None) => {
                panic!("Assignment value is missing");
            }
            Err(_) => {
                panic!("Failed to parse assignment value");
            }
        }; 


        let mut assignment: ASTNode = ASTNode::new(SyntaxElement::Assignment);
        assignment.add_child(identifier_node);
        assignment.add_child(value_node);

        Ok(Some(assignment))
        
    }
    
    /// Parses a binary expression
    pub fn parse_binary_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
    //     match self.parse_router() {
    //         Ok(Some(lhs)) => {
    //             while let Some(op_token) = self.get_input().get(self.get_current()) {
    //                 if let Some(&precedence) = binop_precedence().get(&self.operator_to_char(op_token)) {
    //                     self.consume_token(op_token.clone())?;
        
    //                     let mut rhs: Option<ASTNode> = self.parse_router()?; 
    //                     let operator: String = self.operator_to_char(op_token).to_string();
        
    //                     while let Some(next_op) = self.get_next_operator() {
    //                         if let Some(&next_precedence) = binop_precedence().get(&self.operator_to_char(next_op)) {
    //                             if next_precedence > precedence {
    //                                 rhs = self.parse_binary_expression()?;
    //                             } else {
    //                                 break; 
    //                             }
    //                         }
    //                     }
        
    //                     let mut binary_exp_node = ASTNode::new(SyntaxElement::BinaryExpression);
    //                     binary_exp_node.add_child(lhs);
    //                     binary_exp_node.add_child(SyntaxElement::Operator(self.operator_to_char(op_token)));
    //                     binary_exp_node.add_child(rhs);
        
    //                 } else {
    //                     break;
    //                 }
    //             }
    //             Ok(Some(lhs))
    //         }
    //         _ => {
    //             panic!("missing binary exp")
    //         }
    //     }
    return Ok(None);
    }
}