pub mod frontend;
pub mod backend;

use frontend::lexer::Lexer;
use frontend::syntax::token::Token;

use std::io::{self, Write};

use crate::frontend::{syntax::ast::AST, parser::Parser};

fn print_ready() {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    handle.write_all(b"ready> ").expect("Error writing to stderr");
    handle.flush().expect("Error flushing stderr");
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.to_string() 
}

fn main_loop() {
    // Command line interpreter
    loop {
        print_ready();
        let user_input: String = read_user_input();
        let tokens: Vec<Token> = Lexer::lex(&user_input);
        let ast: Option<AST> = Parser::parse(tokens);
        println!("{:?}", ast);
    }
}

fn main() {
    main_loop();
}
