/*
Converts raw text into lexemes
*/

use crate::frontend::{
    syntax::token::Token,
    utils::error::ErrorType,
};

pub struct Lexer {
    input: Vec<char>, // Source code
    position: usize, // Current position in source code
    current: char, // Current character being read
}

impl Lexer {
    /// Creates a new lexer
    fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            current: '~', // EOF token, set initially but not necessarily the first token
        }
    }

    /// Returns a vector of tokens
    pub fn lex(input: &str) -> Result<Vec<Token>, Vec<ErrorType>> {
        let mut lexer: Lexer = Lexer::new(input.chars().collect());
        let mut errors: Vec<ErrorType> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        lexer.current = lexer.input[0];

        loop {
            let token: Result<Token, ErrorType> = lexer.next_token();
            match token {
                Ok(token) => {
                    if token == Token::EOF {
                        tokens.push(token);
                        break;
                    }
                    tokens.push(token);
                }
                Err(error) => {
                    errors.push(error);
                }
            }

        }
        if errors.is_empty() {
            return Ok(tokens);
        }
        Err(errors)
    }
    
    /// Advances the currently read character
    fn read_char(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current = '~';
        } else {
            self.current = self.input[self.position];
        }
    }

    /// Ignores whitespace
    fn skip_whitespace(&mut self) {
        loop {
            if matches!(self.current, ' ' | '\t' | '\n' | '\r') {
                self.read_char();
            }
            else {
                break;
            }
        }
    }

    /// Returns the current token type and advances to the next token
    fn next_token(&mut self) -> Result<Token, ErrorType> {
        self.skip_whitespace();

        let tok: Result<Token, ErrorType> = match self.current {
            '~' => Ok(Token::EOF),

            '/' => Ok(Token::DIVIDE),
            '-' => Ok(Token::MINUS),
            '+' => Ok(Token::PLUS),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Ok(Token::EQUALEQUAL)
                }
                else {
                    Ok(Token::EQUAL)
                }
            },

            '}' => Ok(Token::RBRACKET),
            '{' => Ok(Token::LBRACKET), // depending on your text editor, this character may cause problems, but
            '(' => Ok(Token::LPAREN),   //      the rustc compiler is fine with this
            ')' => Ok(Token::RPAREN),
            ';' => Ok(Token::SEMICOLON),
            ':' => {
                if self.peek_char() == ':' {
                    self.read_char();
                    Ok(Token::COLONCOLON)
                }
                else {
                    Ok(Token::COLON)
                }
            },
            ',' => Ok(Token::COMMA),
            '%' => Ok(Token::MOD),
            '[' => Ok(Token::LBRACE),
            ']' => Ok(Token::RBRACE),
            '.' => Ok(Token::DOT),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Ok(Token::NOTEQUAL)
                }
                else {
                    Ok(Token::LOGICALNOT)
                }
            },
            '*' => Ok(Token::MULTIPLY),
            '^' => Ok(Token::EXPONENT),
            
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Ok(Token::LESSTHANEQUAL)
                } else {
                    Ok(Token::LESSTHAN)
                }
            },
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Ok(Token::GREATERTHANEQUAL)
                } else {
                    Ok(Token::GREATERTHAN)
                }
            },

            '&' => {
                if self.peek_char() == '&' {
                    self.read_char(); 
                    self.read_char(); 
                    Ok(Token::LOGICALAND)
                } else {
                    let mut err_token = String::new();
                    err_token.push(self.current);
                    Err(ErrorType::UnrecognizedToken { token: err_token })
                }
            },
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char(); 
                    self.read_char(); 
                    Ok(Token::LOGICALOR)
                } else {
                    let mut err_token = String::new();
                    err_token.push(self.current);
                    Err(ErrorType::UnrecognizedToken { token: err_token })
                }
            },
            _ if is_letter(self.current) => {
                let identifier: Vec<char> = self.read_identifier();
                Ok(get_token(&identifier).unwrap_or_else(|_| Token::IDENTIFIER(identifier))) // i don't love this solution
            },
            _ if is_digit(self.current) => Ok(Token::INT(self.read_number())),

            _ => { 
                let mut err_token = String::new();
                err_token.push(self.current);
                Err(ErrorType::UnrecognizedToken { token: err_token })
            },
        };

        self.read_char();
        tok
    }

    /// Reads an identifier from the input.
    fn read_identifier(&mut self) -> Vec<char> {
        self.read_while(is_letter)
    }

    /// Reads a number from the input.
    fn read_number(&mut self) -> Vec<char> {
        self.read_while(is_digit)
    }

    /// Gives the next character without changing the position
    fn peek_char(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '~' // EOF token
        } else {
            self.input[self.position + 1]
        }
    }
    

    /// Reads characters from the input while the given predicate is true.
    fn read_while<F>(&mut self, predicate: F) -> Vec<char>
        where
            F: Fn(char) -> bool,
        {
            let start_pos = self.position;
            while self.position < self.input.len() && predicate(self.current) {
                self.read_char();
            }
            self.position = self.position - 1; // hacky solution, fix later
            self.input[start_pos..=self.position].to_vec() 
        }
}

fn is_letter(current: char) -> bool {
    'a' <= current && current <= 'z' || 
        'A' <= current && current <= 'Z' || current == '_'
}

fn is_digit(current: char) -> bool {
    '0' <= current && current <= '9'
}

/// retrieves a token if text matches, else error
fn get_token(raw_text: &Vec<char>) -> Result<Token, ErrorType> {
    let identifier: String = raw_text.into_iter().collect();
    match &identifier[..] {
        "let" => Ok(Token::LET),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "return" => Ok(Token::RETURN),
        "Integer" => Ok(Token::TINTEGER),
        "Float" => Ok(Token::TFLOAT),
        "Boolean" => Ok(Token::TBOOLEAN),
        "fn" => Ok(Token::FUNCTION),
        "struct" => Ok(Token::STRUCT),
        "enum" => Ok(Token::ENUM),
        "String" => Ok(Token::TSTRING),
        "Void" => Ok(Token::TVOID),
        "Char" => Ok(Token::TCHAR),
        "elif" => Ok(Token::ELIF),
        "for" => Ok(Token::FOR),
        "break" => Ok(Token::BREAK),
        "do" => Ok(Token::DO),
        "while" => Ok(Token::WHILE),
        "match" => Ok(Token::MATCH),
        "continue" => Ok(Token::CONTINUE),
        _ => Err(ErrorType::UnrecognizedToken { token: String::from("Unrecognized token") }),
    }
}


/// TESTS ///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let inputs = "let true false if else return fn";
        let expected = vec![
            Token::LET, Token::TRUE, Token::FALSE,
            Token::IF, Token::ELSE, Token::RETURN, Token::FUNCTION, Token::EOF
        ];
        
        let result = Lexer::lex(inputs);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_identifiers() {
        let input = "variable another_var";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::IDENTIFIER(vec!['v', 'a', 'r', 'i', 'a', 'b', 'l', 'e']),
            Token::IDENTIFIER(vec!['a', 'n', 'o', 't', 'h', 'e', 'r', '_', 'v', 'a', 'r']),
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_int_literals() {
        let input = "123 456";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::INT(vec!['1', '2', '3']),
            Token::INT(vec!['4', '5', '6']),
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_operators_and_special_chars() {
        let input = "+ - = ; ( ) { } , :";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::PLUS, Token::MINUS, Token::EQUAL, Token::SEMICOLON,
            Token::LPAREN, Token::RPAREN, Token::LBRACKET, Token::RBRACKET,
            Token::COMMA, Token::COLON, Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_complex_expressions() {
        let input: &str = "let x = 5 + 10 / 5 % 3;";
        let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
        let expected: Vec<Token> = vec![
            Token::LET,
            Token::IDENTIFIER(vec!['x']),
            Token::EQUAL,
            Token::INT(vec!['5']),
            Token::PLUS,
            Token::INT(vec!['1', '0']),
            Token::DIVIDE,
            Token::INT(vec!['5']),
            Token::MOD,
            Token::INT(vec!['3']),
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "   let    x   = 5  ;  ";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::LET,
            Token::IDENTIFIER(vec!['x']),
            Token::EQUAL,
            Token::INT(vec!['5']),
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_invalid_char() {
        let input = "let $invalid = 5;";
        let result = Lexer::lex(input);
        let expected_error = ErrorType::UnrecognizedToken{token: "$".to_string()};
        let expected = Err(vec![expected_error]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_assignment() {
        let input = "let x: Integer = 5;";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::LET,
            Token::IDENTIFIER(vec!['x']),
            Token::COLON,
            Token::TINTEGER, 
            Token::EQUAL,
            Token::INT(vec!['5']),
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_function_declarations() {
        let input: &str = "fn add(a: Integer, b: Integer): Integer { return a + b; }";
        let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
        let expected: Vec<Token> = vec![
            Token::FUNCTION,
            Token::IDENTIFIER(vec!['a', 'd', 'd']),
            Token::LPAREN,
            Token::IDENTIFIER(vec!['a']),
            Token::COLON,
            Token::TINTEGER,
            Token::COMMA,
            Token::IDENTIFIER(vec!['b']),
            Token::COLON,
            Token::TINTEGER,
            Token::RPAREN,
            Token::COLON,
            Token::TINTEGER,
            Token::LBRACKET,
            Token::RETURN,
            Token::IDENTIFIER(vec!['a']),
            Token::PLUS,
            Token::IDENTIFIER(vec!['b']),
            Token::SEMICOLON,
            Token::RBRACKET,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_if_else_in_function() {
        let input: &str = "fn check(x: Integer) { if x > 0 { return true; } else { return false; } }";
        let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
        let expected: Vec<Token> = vec![
            Token::FUNCTION,
            Token::IDENTIFIER(vec!['c', 'h', 'e', 'c', 'k']),
            Token::LPAREN,
            Token::IDENTIFIER(vec!['x']),
            Token::COLON,
            Token::TINTEGER,
            Token::RPAREN,
            Token::LBRACKET,
            Token::IF,
            Token::IDENTIFIER(vec!['x']),
            Token::GREATERTHAN, 
            Token::INT(vec!['0']),
            Token::LBRACKET,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACKET,
            Token::ELSE,
            Token::LBRACKET,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACKET,
            Token::RBRACKET,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }


    #[test]
    fn test_logical_operators_and_parentheses() {
        let input: &str = "let result = (5 > 3) && (2 < 4);";
        let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
        let expected: Vec<Token> = vec![
            Token::LET,
            Token::IDENTIFIER(vec!['r', 'e', 's', 'u', 'l', 't']),
            Token::EQUAL,
            Token::LPAREN,
            Token::INT(vec!['5']),
            Token::GREATERTHAN, 
            Token::INT(vec!['3']),
            Token::RPAREN,
            Token::LOGICALAND,
            Token::LPAREN,
            Token::INT(vec!['2']),
            Token::LESSTHAN, 
            Token::INT(vec!['4']),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_nested_function_calls() {
        let input: &str = "let val = add(multiply(2, 3), 4);";
        let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
        let expected: Vec<Token> = vec![
            Token::LET,
            Token::IDENTIFIER(vec!['v', 'a', 'l']),
            Token::EQUAL,
            Token::IDENTIFIER(vec!['a', 'd', 'd']),
            Token::LPAREN,
            Token::IDENTIFIER(vec!['m', 'u', 'l', 't', 'i', 'p', 'l', 'y']),
            Token::LPAREN,
            Token::INT(vec!['2']),
            Token::COMMA,
            Token::INT(vec!['3']),
            Token::RPAREN,
            Token::COMMA,
            Token::INT(vec!['4']),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_boolean_literals() {
        let input = "true false";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::TRUE, Token::FALSE, Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_comparison_operators() {
        let input = "< > <= >= == !=";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::LESSTHAN, Token::GREATERTHAN, Token::LESSTHANEQUAL,
            Token::GREATERTHANEQUAL, Token::EQUALEQUAL, Token::NOTEQUAL,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_arithmetic_operators() {
        let input = "+ - * / % ^";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::PLUS, Token::MINUS, Token::MULTIPLY,
            Token::DIVIDE, Token::MOD, Token::EXPONENT,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_logical_operators() {
        let input = "&& || !";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::LOGICALAND, Token::LOGICALOR, Token::LOGICALNOT,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_struct_enum_declarations() {
        let input = "struct enum";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::STRUCT, Token::ENUM, Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_control_flow_tokens() {
        let input = "if elif else for while do break continue match";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::IF, Token::ELIF, Token::ELSE,
            Token::FOR, Token::WHILE, Token::DO,
            Token::BREAK, Token::CONTINUE, Token::MATCH,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_braces_and_parentheses() {
        let input = "{ } [ ] ( )";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::LBRACKET, Token::RBRACKET, Token::LBRACE,
            Token::RBRACE, Token::LPAREN, Token::RPAREN,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_type_annotations() {
        let input = "Integer Float Boolean String Char Void";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::TINTEGER, Token::TFLOAT, Token::TBOOLEAN,
            Token::TSTRING, Token::TCHAR, Token::TVOID,
            Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_dot_and_coloncolon_operators() {
        let input = ". ::";
        let result = Lexer::lex(input);
        let expected = vec![
            Token::DOT, Token::COLONCOLON, Token::EOF,
        ];
        assert_eq!(result, Ok(expected));
    }
}
