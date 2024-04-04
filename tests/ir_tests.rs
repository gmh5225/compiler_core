use std::sync::{Arc, Mutex};

use compiler_core::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::utils::write_to_file
    }, 
    constants::DEFAULT_PRIORITY_MODELEMENT, 
    frontend::{
        ast::{
            ast_stitcher::ast_stitch, 
            ast_struct::{
                ASTNode, ModElement, Module, AST
            }, 
            data_type::DataType, 
            syntax_element::SyntaxElement
        }, 
        symbol_table::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue},
    }
};

fn wrap_in_tle(ast_node: ASTNode) -> AST {
    let mut tle: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    tle.add_child(ast_node);
    AST::new(tle)
}

#[test]
fn test_function_declaration() {
    let mut function_ast = ASTNode::new(SyntaxElement::FunctionDeclaration);

    let fn_id = ASTNode::new(SyntaxElement::Identifier("testFunction".to_string()));
    let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));

    function_ast.add_child(fn_id);
    function_ast.add_child(fn_type);

    let ast: AST = wrap_in_tle(function_ast);

    let mut symbol_table_stack = SymbolTableStack::new();
    let mut symbol_table_global = SymbolTable::new();
    let fn_value = SymbolValue::FunctionValue{
        parameters: Vec::new(),
        return_type: Some(DataType::Integer),
    };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    symbol_table_global.add("testFunction".to_string(), fn_info);
    symbol_table_stack.push(SymbolTable::new());

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, Arc::new(Mutex::new(symbol_table_stack)), DEFAULT_PRIORITY_MODELEMENT)]);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(&module, "output_simple_fn.ll"){
        Ok(_) => {},
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }
}

#[test]
fn test_function_with_if_else() {
    let mut if_statement = ASTNode::new(SyntaxElement::IfStatement);

    let mut if_condition = ASTNode::new(SyntaxElement::Condition);

    let if_value = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    if_condition.add_child(if_value);

    let mut then_branch = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_statement = ASTNode::new(SyntaxElement::Return);

    let then_ret_value = ASTNode::new(SyntaxElement::Literal("1".to_string()));
    return_statement.add_child(then_ret_value);

    then_branch.add_child(return_statement);

    if_statement.add_child(if_condition);
    if_statement.add_child(then_branch);

    let mut else_branch = ASTNode::new(SyntaxElement::ElseStatement);
    let mut else_block = ASTNode::new(SyntaxElement::BlockExpression);

    let mut return_statement_else = ASTNode::new(SyntaxElement::Return);
    let return_value = ASTNode::new(SyntaxElement::Literal("1".to_string()));
    
    return_statement_else.add_child(return_value);

    else_block.add_child(return_statement_else);

    else_branch.add_child(else_block);

    if_statement.add_child(else_branch);

    let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);

    fn_block.add_child(if_statement);

    let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));

    let mut fn_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);
    fn_declaration_node.add_child(fn_type);
    fn_declaration_node.add_child(fn_block);

    let ast: AST = wrap_in_tle(fn_declaration_node);

    let mut symbol_table_stack = SymbolTableStack::new();

    symbol_table_stack.push(SymbolTable::new());

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, Arc::new(Mutex::new(symbol_table_stack)), DEFAULT_PRIORITY_MODELEMENT)]);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(&module, "output_if_else_fn.ll"){
        Ok(_) => {},
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }
}

#[test]
fn test_function_with_while_loop() {
    let mut while_condition = ASTNode::new(SyntaxElement::Condition);
    let while_condition_value = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    while_condition.add_child(while_condition_value);

    let mut while_body = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_statement = ASTNode::new(SyntaxElement::Return);
    let return_value = ASTNode::new(SyntaxElement::Literal("42".to_string()));
    return_statement.add_child(return_value);
    while_body.add_child(return_statement);

    let mut while_statement = ASTNode::new(SyntaxElement::WhileLoop);
    while_statement.add_child(while_condition);
    while_statement.add_child(while_body);

    let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);
    fn_block.add_child(while_statement);

    let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    let fn_id = ASTNode::new(SyntaxElement::Identifier("testFunctionWithWhileLoop".to_string()));

    let mut fn_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);
    fn_declaration_node.add_child(fn_id);
    fn_declaration_node.add_child(fn_type);
    fn_declaration_node.add_child(fn_block);

    let ast = wrap_in_tle(fn_declaration_node);

    let mut symbol_table_stack = SymbolTableStack::new();
    symbol_table_stack.push(SymbolTable::new());

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, Arc::new(Mutex::new(symbol_table_stack)), DEFAULT_PRIORITY_MODELEMENT)]);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(&module, "output_while_loop.ll") {
        Ok(_) => {},
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }
}


#[test]
fn test_function_with_do_while_loop() {
    let do_while_condition = ASTNode::new(SyntaxElement::Literal("true".to_string()));

    let mut return_statement = ASTNode::new(SyntaxElement::Return);
    let return_value = ASTNode::new(SyntaxElement::Literal("24".to_string()));
    return_statement.add_child(return_value);

    let mut do_while_body = ASTNode::new(SyntaxElement::BlockExpression);
    do_while_body.add_child(return_statement);

    let mut do_while_statement = ASTNode::new(SyntaxElement::DoWhileLoop);
    do_while_statement.add_child(do_while_condition);
    do_while_statement.add_child(do_while_body);

    let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let fn_id = ASTNode::new(SyntaxElement::Identifier("testFunctionWithDoWhileLoop".to_string()));
    function_declaration_node.add_child(fn_id);
    function_declaration_node.add_child(fn_type);
    function_declaration_node.add_child(do_while_statement);

    let ast = wrap_in_tle(function_declaration_node);

    let mut symbol_table_stack = SymbolTableStack::new();
    symbol_table_stack.push(SymbolTable::new());

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, Arc::new(Mutex::new(symbol_table_stack)), DEFAULT_PRIORITY_MODELEMENT)]);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(&module, "output_do_while_loop_fn.ll") {
        Ok(_) => {},
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }
}
