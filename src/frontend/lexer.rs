/*
Converts raw text into lexemes
*/

use crate::frontend::syntax::token;

pub struct Lexer {
    input: Vec<char>, // Source code
    pub position: usize, // Current position in source code
    pub read_position: usize, // Next character to read
    pub current: char, // Current character being read
}

impl Lexer {
    /// Creates a new lexer
    fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            current: '0'
        }
    }

    /// Returns a vector of tokens
    pub fn lex(input: &str) -> Vec<token::Token> {
        let mut lexer = Lexer::new(input.chars().collect());
        let mut tokens = Vec::new();
        lexer.read_char();

        loop {
            let token = lexer.next_token();
            if token == token::Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
    
    /// Advances the currently read character
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current = '0';
        } else {
            self.current = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    
    /// Ignores whitespace
    pub fn skip_whitespace(&mut self) {
        if matches!(self.current, ' ' | '\t' | '\n' | '\r') {
            self.read_char();
        }
    }

    
    /// Returns the current token type and advances to the next token
    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        let tok = match self.current {
            '=' => token::Token::EQUAL,
            '+' => token::Token::PLUS,
            '-' => token::Token::MINUS,
            ';' => token::Token::SEMICOLON,
            '0' => token::Token::EOF,
            _ if is_letter(self.current) => {
                let identifier = self.read_identifier();
                token::get_token(&identifier).unwrap_or_else(|_| token::Token::IDENTIFIER(identifier))
            },
            _ if is_digit(self.current) => token::Token::INT(self.read_number()),
            _ => token::Token::ERROR,
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
        self.input[start_pos..self.position].to_vec()
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
        let input = "let a = 5 ;"; // fix this later
        let tokens = Lexer::lex(input);

        let expected_tokens = vec![
            token::Token::LET,
            token::Token::IDENTIFIER(vec!['a']),
            token::Token::EQUAL,
            token::Token::INT(vec!['5']),
            token::Token::SEMICOLON,
            token::Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
