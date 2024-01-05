pub mod frontend;
pub mod backend;

use frontend::lexer::Lexer;
use frontend::syntax::token::Token;

use std::io::{self, Write};

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
    // Loop until end of file
    loop {
        print_ready();
        let user_input = read_user_input();
        if user_input.is_empty() || user_input == "exit" {
            break;
        }
        let tokens: Vec<Token> = Lexer::lex(&user_input);
        println!("{:?}", tokens);
        // match cur_tok {
        //     Token::EOF => return,
        //     ';' => advance_token(),
        //     _ => handle_top_level_expression(),
        // }
    }
}

fn main() {
    main_loop();
}
