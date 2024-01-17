
use std::path::Path;
use std::fs;

use crate::{
    frontend::{
        syntax::{token::Token, lexer::Lexer},
        utils::{
            error::ErrorType,
            entry_points::entry_points,
        }, 
        ast::{ast_struct::AST, ast_stitcher::ast_stitch}, 
        parser::parser_core::Parser, 
        analysis::sem_analysis::SemAnalysis,
    }, 
    backend::codegen::ir_codegen::IRGenerator
};

pub fn compile(file_path: &str) -> Result<Vec<u8>, Vec<ErrorType>> { // change to option
    let path: &Path = Path::new(file_path);
    validate_file_path(path, file_path)?;

    let entry_points: Vec<usize> = entry_points(path);

    let content: String = fs::read_to_string(path)
        .expect("Error reading the file");

    let mut asts: Vec<AST> = Vec::new();
    let mut i: usize = entry_points[0];
    for _ in entry_points.iter() {
        let slice: &str = &content[i..i+1];
        match generate_ast(slice.to_string()) {
            Ok(ast) => asts.push(ast),
            Err(errors) => return Err(errors),
        }
        i += 1;
    }
    // handles the last entry point to the end of the file
    match generate_ast(content[i..].to_string()) {
        Ok(ast) => asts.push(ast),
        Err(errors) => return Err(errors),
    }
    let mod_ast: AST = ast_stitch(asts);
    generate_obj(mod_ast)

}

fn validate_file_path(path: &Path, file_path: &str) -> Result<(), Vec<ErrorType>> {
    if !path.exists() || !path.is_file() {
        eprintln!("Error: File not found - {}", file_path);
        panic!("file not found"); 
    }
    Ok(())
}

fn generate_ast(content: String) -> Result<AST, Vec<ErrorType>> {
    // Lexer
    let tokens: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(&content);
        match tokens {
            Ok(tokens) => {
                // Parser
                Parser::parse(tokens)
            }
            Err(lexer_errors) => {
                for error in lexer_errors {
                    eprintln!("Lexer Error: {:?}", error);
                }
                panic!()
            }
        }
}

fn generate_obj(content: AST) -> Result<Vec<u8>, Vec<ErrorType>> {
    // Semantic Analysis
    let sem_analysis_errors: Vec<ErrorType> = SemAnalysis::sem_analysis(content.clone());
    if sem_analysis_errors.is_empty() {
        // IR Generation 
        let generated_ir: *mut llvm_sys::LLVMModule = IRGenerator::generate_ir(&content);
        println!("{:?}", generated_ir);
        return Ok(Vec::new()); // this will need to change obv

    } else {
        for error in sem_analysis_errors {
            eprintln!("Syntax Error: {:?}", error);
        }
        panic!()
    }
}

