pub mod frontend;

use frontend::token;
use frontend::lexer;
fn main() {
    let output : Vec<token::Token> 
        = lexer::Lexer::lex("let a = 5;");
    println!("{:?}", output);
}  
