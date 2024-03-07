use std::{
    collections::HashMap, fs, path::Path, sync::{Arc, Mutex}
};

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, execute::execution_engine::ExecutionEngine, llvm_lib::ir_lib::utils::write_to_file
    }, constants::DEFAULT_PRIORITY_MODELEMENT, frontend::{
        ast::{
            ast_stitcher::ast_stitch, ast_struct::{
                ModAST, ModElement, AST
            }, sem_rule::{RulesConfig, SemanticRule}, syntax_element::SyntaxElement
        }, lexer::{
            lexer_core::Lexer, token::Token
        }, parser::parser_core::Parser, sem_analysis::sem_analysis_core::SemAnalysis, symbol_table::{self, symbol_table_struct::SymbolTableStack}, 
        utils::{
            entry_points::entry_points, error::ErrorType
        }
    }
};

pub fn compile(file_path: &str, jit: bool, emit_ir: bool) -> Result<Vec<u8>, Vec<ErrorType>> {
    let path: &Path = Path::new(file_path);
    validate_file_path(path, file_path)?;

    let entry_points: Vec<usize> = entry_points(path);
    let content: String = fs::read_to_string(path).expect("no file");

    let mut mod_elements: Vec<ModElement> = Vec::new();

    if entry_points.is_empty() {
        panic!("empty file");
    }

    for window in entry_points.windows(2) {
        let start: usize = window[0];
        let end: usize = window[1];
        let slice: &str = &content[start..end];

        match generate_mod_element(slice.to_string()) {
            Ok(ast_with_sym_table) => mod_elements.push(ast_with_sym_table),
            Err(errors) => return Err(errors),
        }
    }

    let last_start: usize = *entry_points.last().unwrap();
    match generate_mod_element(content[last_start..].to_string()) {
        Ok(mod_element) => mod_elements.push(mod_element),
        Err(errors) => return Err(errors),
    }

    let rules: RulesConfig = read_config();
    let mod_ast: ModAST = ast_stitch(mod_elements);

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

fn generate_mod_element(content: String) -> Result<ModElement, Vec<ErrorType>> {
    let tokens: Vec<Token> = Lexer::lex(&content)?;
    let mut ast= Parser::parse(tokens)?;
    match SymbolTableStack::gen_sym_table_stack(ast) {
        Ok((ast, symbol_table_stack)) => {
            let arc_mutex_sym_table_stack = Arc::new(Mutex::new(symbol_table_stack));
            Ok(ModElement::new(ast,arc_mutex_sym_table_stack, DEFAULT_PRIORITY_MODELEMENT))
        }
        Err(e) => {
            Err(e)
        }
    }
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