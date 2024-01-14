/*
Converts tokens into an AST
*/
                                 
use crate::frontend::{ 
    utils::error::ErrorType,
    syntax::token::Token,
    ast::{
        ast_struct::{AST, ASTNode}, 
        syntax_element::SyntaxElement, 
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
        let mut parser: Parser<'_> = Parser::new(&input);
        let mut root_children: Vec<ASTNode> = Vec::new();  
        let mut errors: Vec<ErrorType> = Vec::new();

        while parser.current < input.len() {
            match parser.parse_element() { 
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

    pub fn get_input(&mut self) -> Vec<Token> {
        self.input.clone()
    }
    pub fn get_current(&mut self) -> usize {
        self.current.clone()
    }
    pub fn consume_token(&mut self, expected_token: Token) -> Result<(), ErrorType> {
        if let Some(token) = self.get_input().get(self.get_current()) {
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
    
    pub fn next_token(&mut self) -> Option<Token> {
        self.current += 1;
        if self.get_current() < self.get_input().len() {
            self.get_input().get(self.get_current()).cloned()
        } else {
            None
        }
    }
}
