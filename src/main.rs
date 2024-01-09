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

use crate::{ frontend::{ syntax::{ ast::AST, token::Token }, 
                       parser::Parser, 
                       lexer::Lexer, 
                       sem_analysis::SemAnalysis,
                       error::ErrorType },
              backend::code_generation::ir_codegen::IRGenerator };

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
    loop {
        print_ready();
        let user_input: String = read_user_input();

        let tokens: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(&user_input);
        match tokens {
            Ok(tokens) => {
                let ast_result: Result<AST, Vec<ErrorType>> = Parser::parse(tokens);
                match ast_result {
                    Ok(ast) => {
                        let sem_analysis_errors: Vec<ErrorType> = SemAnalysis::sem_analysis(ast.clone());
                        if sem_analysis_errors.is_empty() {
                            let generated_ir = IRGenerator::generate_ir(&ast);
                            println!("{:?}", generated_ir);
                            // next, pass to execution engine
                        } else {
                            for error in sem_analysis_errors {
                                println!("Error: {:?}", error);
                            }
                        }
                    },
                    Err(parser_errors) => {
                        for error in parser_errors {
                            println!("Parser Error: {:?}", error);
                        }
                    }
                }
            },
            Err(lexer_errors) => {
                for error in lexer_errors {
                    println!("Lexer Error: {:?}", error);
                }
            }
        }
    }
}


fn main() {
    main_loop();
}
