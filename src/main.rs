/*
Program driver
    Goal capabilities:
        - Command line interpreter
        - Compile to LLVM IR/.o
        - Automated execution of .o
        - Regression testing
 */

pub mod frontend;
pub mod backend;

use std::io::{self, Write};

use crate::frontend::{ syntax::{ ast::AST, token::Token }, 
                       parser::Parser, 
                       lexer::Lexer, 
                       sem_analysis::SemAnalysis,
                       error::ErrorType };

use crate::backend::code_generation::ir_codegen::IRGenerator;

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
        let tokens: Vec<Token> = Lexer::lex(&user_input); // switch to Result(Vec<Token>, Vec<ErrorType>)
        let ast: Option<AST> = Parser::parse(tokens); // switch to Result(AST, Vec<ErrorType>)
        if let Some(ast) = ast {
            let sem_analysis_errors: Vec<ErrorType> = SemAnalysis::sem_analysis(ast.clone());
            if sem_analysis_errors.len() == 0 {
                let generated_ir = IRGenerator::generate_ir(&ast);
                println!("{:?}", generated_ir);
            } else {
                panic!("{:?}", sem_analysis_errors);
            }
        } else {
            panic!("Unsuccessfully tried to make AST");
        }
    }
}

fn main() {
    main_loop();
}
