/*
Converts tokens into an AST
*/

use crate::frontend::syntax::token::Token;
use crate::frontend::syntax::ast::{AST, ASTNode, SyntaxElement, DataType};

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
    
    pub fn parse(input: Vec<Token>) -> Option<AST> {
        let mut parser = Parser::new(&input);
        let mut root_children = Vec::new();  

        while parser.current < input.len() {
            if let Some(expr) = parser.parse_expression() {
                root_children.push(expr);  
            } else {
                return None;  
            }
        }

        let mut root = ASTNode::new(SyntaxElement::FileExpression);
        root.children = root_children;
        Some(AST::new(root))  
    }

    fn parse_expression(&mut self) -> Option<ASTNode> {
        let left_expr = self.parse_primary();

        if self.current < self.input.len() {
            match self.input.get(self.current) {
                Some(Token::EQUAL) => {
                    self.current += 1; 
                    let right_expr = self.parse_expression(); 

                    if let Some(right) = right_expr {
                        if let Some(SyntaxElement::Variable(var_name)) = left_expr.clone().map(|node| node.element) {
                            return Some(ASTNode::new(SyntaxElement::Assignment {
                                variable: var_name,
                                value: Box::new(right),
                            }));
                        } else {
                            return None; 
                        }
                    }
                },
                Some(Token::PLUS) | Some(Token::MINUS) => {
                    let operator = match self.input.get(self.current) {
                        Some(Token::PLUS) => "+",
                        Some(Token::MINUS) => "-",
                        _ => "", 
                    };
                    self.current += 1; 
                    let right_expr = self.parse_expression(); 

                    if let (Some(left), Some(right)) = (left_expr.clone(), right_expr) {
                        return Some(ASTNode::new(SyntaxElement::BinaryExpression {
                            left: Box::new(left),
                            operator: operator.to_string(),
                            right: Box::new(right),
                        }));
                    }
                },
                _ => (),
            };
        }

        left_expr
    }


    fn parse_primary(&mut self) -> Option<ASTNode> {
        if self.current >= self.input.len() {
           return None;
        }
        if let Some(token) = self.input.get(self.current) {
            match token {
                Token::INT(chars) => {
                    let value_str: String = chars.iter().collect();
                    self.current += 1;
                    Some(ASTNode::new(SyntaxElement::Literal(DataType::Integer, value_str)))
                },
                Token::IDENTIFIER(chars) => {
                    let name: String = chars.iter().collect();
                    self.current += 1;
                    Some(ASTNode::new(SyntaxElement::Variable(name)))
                },
                Token::TRUE => {
                    self.current += 1;       
                    Some(ASTNode::new(SyntaxElement::Literal(DataType::Boolean, "true".to_string())))
                },
                Token::FALSE => { 
                    self.current += 1;     
                    Some(ASTNode::new(SyntaxElement::Literal(DataType::Boolean, "false".to_string())))
                }, 
                _ => None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let tokens = vec![
            Token::INT(vec!['1', '2', '3']),
            Token::PLUS,
            Token::INT(vec!['4', '5', '6']),
        ];
        let ast: Option<AST> = Parser::parse(tokens);

        let literal_1 = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "123".to_string()));
        let literal_2 = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "456".to_string()));

        let binary_expr = ASTNode::new(SyntaxElement::BinaryExpression {
            left: Box::new(literal_1),
            operator: "+".to_string(),
            right: Box::new(literal_2),
        });

        let mut expected_ast = AST {
            root: ASTNode::new(SyntaxElement::FileExpression),
        };
        expected_ast.root.children.push(binary_expr);

        assert!(ast.is_some(), "Parsed AST was None");
        assert_eq!(ast.unwrap(), expected_ast);
    }

}

