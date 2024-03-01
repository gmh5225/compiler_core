use std::{
    path::Path, 
    collections::HashMap,
    fs,
};

use crate::{
    frontend::{
        lexer::{
            token::Token, 
            lexer_core::Lexer
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
        sem_analysis::sem_analysis_core::SemAnalysis, 
        symbol_table::symbol_table::SymbolTableStack,
    }, 
    backend::{
        llvm_lib::ir_lib::utils::write_to_file,
        execute::execution_engine::ExecutionEngine,
        codegen::ir::ir_codegen_core::IRGenerator,
    },
};

pub fn compile(file_path: &str, jit: bool, emit_ir: bool) -> Result<Vec<u8>, Vec<ErrorType>> {
    let path: &Path = Path::new(file_path);
    validate_file_path(path, file_path)?;

    let entry_points: Vec<usize> = entry_points(path);
    let content: String = fs::read_to_string(path).expect("no file");

    let mut asts_with_sym_tables: Vec<(AST, SymbolTableStack)> = Vec::new();

    if entry_points.is_empty() {
        panic!("empty file");
    }

    for window in entry_points.windows(2) {
        let start: usize = window[0];
        let end: usize = window[1];
        let slice: &str = &content[start..end];

        match generate_ast(slice.to_string()) {
            Ok(ast_with_sym_table) => asts_with_sym_tables.push(ast_with_sym_table),
            Err(errors) => return Err(errors),
        }
    }

    let last_start: usize = *entry_points.last().unwrap();
    match generate_ast(content[last_start..].to_string()) {
        Ok(ast_with_sym_table) => asts_with_sym_tables.push(ast_with_sym_table),
        Err(errors) => return Err(errors),
    }

    let rules: RulesConfig = read_config();
    let mod_ast: ModAST = ast_stitch(asts_with_sym_tables);

    ast_to_obj(mod_ast, rules, jit, emit_ir)
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

fn ast_to_obj(content: ModAST, rules: RulesConfig, jit: bool, emit_ir: bool) -> Result<Vec<u8>, Vec<ErrorType>> {
    let sem_analysis_result: Result<ModAST, Vec<ErrorType>> = SemAnalysis::sem_analysis(content, rules);

    match sem_analysis_result {
        Ok(processed_content) => {
            let generated_ir: *mut llvm::LLVMModule = IRGenerator::generate_ir(processed_content); 
            eprintln!("Successfully Compiled.");
            if emit_ir {
                match write_to_file(&generated_ir, "output_builder.ll") { 
                    Ok(_) => eprintln!("IR written to file."),
                    Err(e) => {
                        eprintln!("File write error: {}", e);
                        panic!()
                    }
                }
            }

            if jit {
                match ExecutionEngine::execute_ir(generated_ir, &[]) { 
                    Ok(_) => println!("Executed using JIT."),
                    Err(e) => {
                        eprintln!("Execution error: {}", e);
                        panic!()
                    }
                }
            }

            Ok(Vec::new()) 
        },
        Err(sem_analysis_errors) => {
            for error in &sem_analysis_errors {
                eprintln!("Syntax Error: {:?}", error);
            }
            Err(sem_analysis_errors)
        }
    }
}