/*
Converts tokens into an AST
*/

use crate::frontend::{ error::ErrorType, 
                       syntax::{ token::Token,
                                 ast::{ AST, ASTNode }, 
                                 syntax_element::SyntaxElement, 
                                 data_type::DataType } }; // generally avoid wild card importing

use super::syntax::{syntax_element::FunctionParameter, binop_precedence}; 

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
        binop_precedence::binop_precedence(); // this is probably important. use it somewhere

        let mut parser: Parser<'_> = Parser::new(&input);
        let mut root_children: Vec<ASTNode> = Vec::new();  
        let mut errors: Vec<ErrorType> = Vec::new();

        while parser.current < input.len() {
            match parser.parse_top_level() { // parses top level expression and recursively parses ALL inner expressions
                Ok(node) => {
                    root_children.push(node);  
                }
                Err(error_types) => {
                    errors.extend(error_types);
                }
            } 
        }

        let mut root = ASTNode::new(SyntaxElement::ModuleExpression);
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
                self.current += 1; // consume function token

                match self.parse_function_declaration() {
                    Ok((identifier, parameters, return_type)) => {
                        let root_element: SyntaxElement = SyntaxElement::FunctionDeclaration { 
                            name: identifier, parameters, return_type: return_type.or(None)
                        };
                        // recurse on children until reaching right bracket, push to root's children, then return
                        // placeholder:
                        self.current += 1; //right bracket
                        let root = ASTNode::new(root_element);
                        Ok(root)
                    },
                    Err(_) => {
                        Err(vec![ErrorType::DevError {}])
                    }
                }
            }
            _ => Err(vec![ErrorType::DevError {  }]),
        }
    }

    /// After reading a function token, consumes the function declaration
    fn parse_function_declaration(&mut self) -> Result<(String, Vec<FunctionParameter>, Option<DataType>), ErrorType> {
        if let Some(Token::IDENTIFIER(name_chars)) = self.input.get(self.current) {
            self.current += 1; // Consume function name
            let name = name_chars.iter().collect();
    
            if let Some(Token::LPAREN) = self.input.get(self.current) {
                self.current += 1; // Consume left paren
    
                let mut parameters: Vec<FunctionParameter> = Vec::new();
    
                while let Some(token) = self.input.get(self.current) {
                    match token {
                        Token::RPAREN => {
                            self.current += 1; // Consume right paren
                            break; // End of parameters
                        },
                        Token::IDENTIFIER(param_name_chars) => {
                            self.current += 1; // Consume parameter name
                            let param_name = param_name_chars.iter().collect();
    
                            if let Some(Token::COLON) = self.input.get(self.current) {
                                self.current += 1; // Consume colon
    
                                // Consume type
                                match self.input.get(self.current) {
                                    Some(Token::TINTEGER) => {
                                        self.current += 1; 
                                        parameters.push(FunctionParameter::new(param_name, DataType::Integer));
                                    },
                                    Some(Token::TFLOAT) => {
                                        self.current += 1; 
                                        parameters.push(FunctionParameter::new(param_name, DataType::Float));
                                    },
                                    Some(Token::TBOOLEAN) => {
                                        self.current += 1; 
                                        parameters.push(FunctionParameter::new(param_name, DataType::Boolean));
                                    },
                                    _ => return Err(ErrorType::DevError{}),
                                }
    
                                match self.input.get(self.current) {
                                    Some(Token::COMMA) => self.current += 1, // Consume comma
                                    Some(Token::RPAREN) => continue, // Handle ')' in next iteration
                                    _ => return Err(ErrorType::DevError{}), 
                                }
                            } else {
                                return Err(ErrorType::DevError{}); 
                            }
                        },
                        _ => return Err(ErrorType::DevError{}), 
                    }
                }
                let mut return_type: Option<DataType> = None;
                if let Some(token) = self.input.get(self.current) {
                    match token {
                        Token::TINTEGER => {
                            self.current += 1;
                            return_type = Some(DataType::Integer);
                        }
                        Token::TBOOLEAN => {
                            self.current += 1;
                            return_type = Some(DataType::Boolean);
                        }  
                        _ => (),

                    }
                } else {
                    return Err(ErrorType::DevError {});
                }
                self.current += 1; // consume left bracket
                Ok((name, parameters, return_type)) 
            } else {
                Err(ErrorType::DevError{}) 
            }
        } else {
            Err(ErrorType::DevError{}) 
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
        fn test_empty_input() {
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
    
    //     #[test]
    //     fn test_function_with_parameters_and_return_type() {
    //         let tokens: Vec<Token> = vec![
    //             Token::FUNCTION,
    //             Token::IDENTIFIER(vec!['c', 'a', 'l', 'c', 'u', 'l', 'a', 't', 'e']),
    //             Token::LPAREN,
    //             Token::IDENTIFIER(vec!['x']),
    //             Token::COLON,
    //             Token::TINTEGER,
    //             Token::COMMA,
    //             Token::IDENTIFIER(vec!['y']),
    //             Token::COLON,
    //             Token::TINTEGER,
    //             Token::RPAREN,
    //             Token::COLON,
    //             Token::TBOOLEAN,
    //             Token::LBRACKET,
    //             Token::RBRACKET
    //         ];
    //         let ast: AST = Parser::parse(tokens).expect("Failed to parse");
    //         match ast.get_root().get_children().first().unwrap().get_element() {
    //             SyntaxElement::FunctionDeclaration { ref name, ref parameters, .. } => {
    //                 assert_eq!(name, "calculate");
    //                 assert_eq!(parameters.len(), 2);
    //             },
    //             _ => panic!("Expected FunctionDeclaration with parameters"),
    //         }
    //     }
    
    // }
    
    // #[test]
    // fn test_function_with_body() {
    //     let tokens = vec![
    //         Token::FUNCTION,
    //         Token::IDENTIFIER(vec!['t', 'e', 's', 't']),
    //         Token::LPAREN,
    //         Token::RPAREN,
    //         Token::LBRACKET,
    //         Token::LET,
    //         Token::IDENTIFIER(vec!['x']),
    //         Token::EQUAL,
    //         Token::INT(vec!['1']),
    //         Token::SEMICOLON,
    //         Token::RBRACKET,
    //     ];
    //     let ast = Parser::parse(tokens).expect("Failed to parse");
    //     match ast.get_root().get_children().first().unwrap().get_element() {
    //         SyntaxElement::FunctionDeclaration { ref name, .. } => {
    //             assert_eq!(name, "test");
    //             assert!(!ast.get_root().get_children().is_empty());
    //         },
    //         _ => panic!("Expected FunctionDeclaration with body"),
    //     }
    }
}

