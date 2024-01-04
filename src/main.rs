pub mod lexer;
mod parser;
mod ast;
pub mod token;

fn main() {
    lexer::Lexer::lex("let a = 5;");
}
