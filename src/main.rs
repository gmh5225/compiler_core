pub mod frontend;
pub mod backend;

use std::io::{self, Write};

fn print_ready() {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    handle.write_all(b"ready> ").expect("Error writing to stderr");
    handle.flush().expect("Error flushing stderr");
}

fn main_loop() {
    // Initialization and first token fetch
    print_ready();
    // let mut cur_tok = get_next_token(); 

    // // Loop until end of file
    // loop {
    //     match cur_tok {
    //         token::Token::EOF => return,
    //         ';' => get_next_token(),
    //         _ => handle_top_level_expression(),
    //     }
    // }
}

fn main() {
    main_loop();
}
