
use std::{path::Path, collections::HashMap};
use std::fs;

use crate::{
    frontend::{
        syntax::{
            token::Token, 
            lexer::Lexer
        },
        utils::{
            error::ErrorType,
            entry_points::entry_points,
        }, 
        ast::{
            ast_struct::{
                AST, 
                ModAST,
            },
            ast_stitcher::ast_stitch, 
            sem_rule::RulesConfig,
            syntax_element::SyntaxElement,
            sem_rule::SemanticRule
        }, 
        parser::parser_core::Parser, 
        analysis::{
            sem_analysis::SemAnalysis, 
            symbol_table::SymbolTableStack
        },
    }, 
    backend::codegen::ir::ir_codegen_core::IRGenerator
};

pub fn compile(file_path: &str) -> Result<Vec<u8>, Vec<ErrorType>> {
    let path = Path::new(file_path);
    validate_file_path(path, file_path)?;

    let entry_points = entry_points(path);
    let content = fs::read_to_string(path).expect("no file");

    let mut asts_with_sym_tables: Vec<(AST, SymbolTableStack)> = Vec::new();

    if entry_points.is_empty() {
        panic!("empty file");
    }

    for window in entry_points.windows(2) {
        let start = window[0];
        let end: usize = window[1];
        let slice = &content[start..end];

        match generate_ast(slice.to_string()) {
            Ok(ast_with_sym_table) => asts_with_sym_tables.push(ast_with_sym_table),
            Err(errors) => return Err(errors),
        }
    }

    let last_start = *entry_points.last().unwrap();
    match generate_ast(content[last_start..].to_string()) {
        Ok(ast_with_sym_table) => asts_with_sym_tables.push(ast_with_sym_table),
        Err(errors) => return Err(errors),
    }

    let rules: RulesConfig = read_config();
    let mod_ast: ModAST = ast_stitch(asts_with_sym_tables);

    generate_obj(mod_ast, rules)
}



fn validate_file_path(path: &Path, file_path: &str) -> Result<(), Vec<ErrorType>> {
    if !path.exists() || !path.is_file() {
        eprintln!("Error: File not found - {}", file_path);
        panic!("file not found"); 
    }
    Ok(())
}

fn read_config() -> RulesConfig {
    // read configuration file
    let rules: HashMap<SyntaxElement, Vec<SemanticRule>> = HashMap::new();
    RulesConfig::new(rules)
}

fn generate_ast(content: String) -> Result<(AST, SymbolTableStack), Vec<ErrorType>> {
    let tokens: Vec<Token> = Lexer::lex(&content)?;
    let (ast, symbol_table) = Parser::parse(tokens)?;
    Ok((ast, symbol_table))
}


fn generate_obj(content: ModAST, rules: RulesConfig) -> Result<Vec<u8>, Vec<ErrorType>> {
    // Semantic Analysis
    let sem_analysis_result = SemAnalysis::sem_analysis(content, rules);

    match sem_analysis_result {
        Ok(processed_content) => {
            let generated_ir: *mut llvm_sys::LLVMModule = IRGenerator::generate_ir(processed_content);
            println!("{:?}", generated_ir);

            Ok(Vec::new()) // Replace with actual bytecode logic
        },
        Err(sem_analysis_errors) => {
            for error in sem_analysis_errors {
                eprintln!("Syntax Error: {:?}", error);
            }
            panic!()
        }
    }
}


