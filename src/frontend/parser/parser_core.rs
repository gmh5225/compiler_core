/*
Converts tokens into an AST and creates a symbol table stack
*/
                                 
use crate::frontend::{ 
    ast::{
        ast_struct::{ASTNode, AST}, 
        syntax_element::SyntaxElement, 
    }, 
    lexer::token::Token, 
    symbol_table::{
        symbol_table_core::SymbolTableStack, 
        symbol_table_struct::SymbolTable
    }, 
    utils::error::ErrorType,
};

/// Parses an input of tokens into an AST   
pub struct Parser {
    input: Vec<Token>,
    current: usize,
    symbol_table_stack: SymbolTableStack,
}

impl Parser {
    fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            current: 0,
            symbol_table_stack: SymbolTableStack::new()
        }
    } 
    
    /// Parses an input of tokens into an AST, or returns a vector of errors
    pub fn parse(input: Vec<Token>) -> Result<(AST, SymbolTableStack), Vec<ErrorType>> {
        let mut parser = Parser::new(input);
        let mut root_children: Vec<ASTNode> = Vec::new();  
        let mut errors: Vec<ErrorType> = Vec::new();

        parser.get_sym_table_stack().push(SymbolTable::new());

        while parser.get_current() < parser.get_input().len() {
            match parser.parse_router() { 
                Ok(Some(node)) => {
                    root_children.push(node);  
                }
                Ok(None) => {}
                Err(error_types) => {
                    errors.extend(error_types);
                }
            } 
        }

        let mut root: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
        root.add_children(root_children);
        let symbol_table: SymbolTableStack = parser.get_sym_table_stack().clone();
        if errors.is_empty() {
            return Ok((AST::new(root), symbol_table));
        }
        Err(errors)
    }  

    /// Gets the current input vector
    pub fn get_input(&mut self) -> Vec<Token> {
        self.input.clone()
    }

    /// Gets the current position in the input vector
    pub fn get_current(&mut self) -> usize {
        self.current.clone()
    }

    pub fn get_sym_table_stack(&mut self) -> &mut SymbolTableStack {
        &mut self.symbol_table_stack
    }

    /// Consumes a token if the expected token matches the token
    pub fn consume_token(&mut self, expected_token: Token) -> Result<(), ErrorType> {
        if let Some(token) = self.get_input().get(self.get_current()) {
            if *token == expected_token {
                self.current += 1;
                Ok(())
            } else {
                panic!("What is this? This is not the right token. Try again. Expected: {:?}, Actual: {:?}", expected_token, *token)
            }
        } else {
            panic!("You tried to consume a token that doesn't exist? Tsk tsk")
        }
    }
    
    /// Peeks at the token that's next (self.current + 1)
    pub fn peek_token(&mut self) -> Option<Token> {
        if self.get_current() < self.get_input().len() {
            self.get_input().get(self.get_current() + 1).cloned()
        } 
        else {
            None
        }
    }

    /// Entry point to the main parsing logic. Serves as a way to match the current token type to the file/expression we want to parse
    pub fn parse_router(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                // top level expressions
                Some(Token::FUNCTION) => return self.parse_function(),
                Some(Token::STRUCT) => return self.parse_struct(), 
                Some(Token::ENUM) => return self.parse_enum(),

                // statements
                Some(Token::IF) => return self.parse_if_statement(),
                Some(Token::FOR) => return self.parse_for_loop(),
                Some(Token::DO) => return self.parse_do_while_loop(), 
                Some(Token::WHILE) => return self.parse_while_loop(),
                Some(Token::MATCH) => return self.parse_match_statement(),
                Some(Token::LET) => return self.parse_initialization(),
                Some(Token::IDENTIFIER(_)) => return self.parse_identifier(),

                // binary operations
                Some(Token::PLUS) | 
                Some(Token::MINUS) | 
                Some(Token::MULTIPLY) | 
                Some(Token::DIVIDE) => return self.parse_binary_expression(),

                // unary operations
                Some(Token::LOGICALNOT) => return self.parse_unary_expression(), 

                // base elements like primitives, and protected keywords
                Some(Token::INT(_)) | 
                Some(Token::TRUE) | 
                Some(Token::FALSE) => return self.parse_primitive(),
                Some(Token::BREAK) |
                Some(Token::RETURN) |
                Some(Token::CONTINUE) |
                Some(Token::SEMICOLON) |
                Some(Token::EOF) => return self.parse_protected_keyword(),
                _ => panic!("Are you sure this is an expression: {:?} {:?}", self.get_input().get(self.get_current()), self.get_current()),

            }
        } else {
            panic!("You hooligan. You're out of tokens")
        }
    }
}
