/*
Converts raw text into lexemes
*/

use crate::frontend::syntax::token::{ Token, get_token };
use crate::frontend::error::ErrorType;

pub struct Lexer {
    input: Vec<char>, // Source code
    pub position: usize, // Current position in source code
    pub current: char, // Current character being read
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
        let mut lexer = Lexer::new(input.chars().collect());
        let mut tokens = Vec::new();
        lexer.current = lexer.input[0];

        loop {
            let token = lexer.next_token();
            if token == Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
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
        if matches!(self.current, ' ' | '\t' | '\n' | '\r') {
            self.read_char();
        }
    }

    /// Returns the current token type and advances to the next token
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.current {
            '=' => Token::EQUAL,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            ';' => Token::SEMICOLON,
            '{' => Token::LEFTBRACKET,
            '}' => Token::RIGHTBRACKET,
            '~' => Token::EOF,
            _ if is_letter(self.current) => {
                let identifier = self.read_identifier();
                get_token(&identifier).unwrap_or_else(|_| Token::IDENTIFIER(identifier))
            },
            _ if is_digit(self.current) => Token::INT(self.read_number()),
            _ => Token::ERROR,
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        let input = "let aaaa= 5444;"; 
        let tokens_result = Lexer::lex(input);

        let expected_tokens = vec![
            Token::LET,
            Token::IDENTIFIER(vec!['a', 'a', 'a', 'a']),
            Token::EQUAL,
            Token::INT(vec!['5', '4', '4', '4']),
            Token::SEMICOLON,
            Token::EOF,
        ];
        if let Ok(tokens) = tokens_result {
            assert_eq!(tokens, expected_tokens);
        } else {
            panic!("Lexer failed to make tokens");
        }

    }
}
