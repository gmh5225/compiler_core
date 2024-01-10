/*
Converts tokens into an AST
*/

use crate::frontend::{ error::ErrorType, 
                       syntax::{ token::Token,
                                 ast::{ AST, ASTNode }, 
                                 syntax_element::SyntaxElement, 
                                 data_type::DataType,
                                 binop_precedence::* } }; // generally avoid wild card importing

use super::syntax::syntax_element::FunctionParameter; 

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
        let mut parser = Parser::new(&input);
        let mut root_children = Vec::new();  
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
            Some(Token::FUNCTION(fn_name)) => {
                self.current += 1; // consume function token
                let identifier: String = fn_name.iter().collect();

                // delete this later and replace with recursive calls
                let dummy_params = Vec::new();

                let root_element: SyntaxElement = SyntaxElement::FunctionDeclaration { 
                    name: (identifier), parameters: (dummy_params), return_type: (None) 
                };
                let root: ASTNode = ASTNode::new(root_element);
                Ok(root)
            }
            _ => unimplemented!("Unimplemented top level expression"),
        }
    }

    fn parse_expression(&mut self) {
        // match self.input.get(self.current) {
        //     Some()
        // }

    }

    fn parse_initialization() {}

    fn parse_assignment() {}

    fn parse_function_declaration(&mut self) -> Result<(Vec<FunctionParameter>, DataType), ErrorType> {
        // match self.input.get(self.current) {
        //     Some(Token::LPAREN) => {
        //         self.current += 1; // consume left paren
        //         match self.input.get(self.current) {
        //             Some(Token::IDENTIFIER(name)) => {
                        
        //             }
        //             _ => unimplemented!("Add error type for reading params parser")
        //         }

        //     }
        //     _ => unimplemented!("Add errortype here for function declaration in parser")
        // }
        Err(ErrorType::DevError{})
    }

    fn children_until(root: ASTNode, stop_at: Token) {

    }

    fn parse_binary_operation() {}

}


/// TESTS ///
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
        let ast: Result<AST, Vec<ErrorType>> = Parser::parse(tokens);

        let literal_1 = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "123".to_string()));
        let literal_2 = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "456".to_string()));

        let binary_expr = ASTNode::new(SyntaxElement::BinaryExpression {
            left: Box::new(literal_1),
            operator: "+".to_string(),
            right: Box::new(literal_2),
        });
        let mut root = ASTNode::new(SyntaxElement::ModuleExpression);
        root.add_child(binary_expr);
        let expected_ast = AST::new(root);

        assert_eq!(ast.unwrap(), expected_ast);
    }

}

