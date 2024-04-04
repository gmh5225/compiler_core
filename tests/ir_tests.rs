use compiler_core::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::utils::write_to_file
    }, 
    frontend::{
        ast::{
            ast_stitcher::ast_stitch, 
            ast_struct::{
                ASTNode, ModAST, AST
            }, 
            data_type::DataType, 
            syntax_element::SyntaxElement
        }, 
        symbol_table::symbol_table::{SymbolTable, SymbolTableStack},
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

    symbol_table_stack.push(SymbolTable::new());

    let mod_ast: ModAST = ast_stitch(vec![(ast, symbol_table_stack)]);

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

//     let then_branch_node = ASTNode::new(SyntaxElement::Return {
//         value: Box::new(ASTNode::new(SyntaxElement::Literal {
//             data_type: DataType::Integer,
//             value: "1",
//         })),
//     });

//     let else_branch_node = ASTNode::new(SyntaxElement::Return {
//         value: Box::new(ASTNode::new(SyntaxElement::Literal {
//             data_type: DataType::Integer,
//             value: "0",
//         })),
//     });

//     let if_statement = ASTNode::new(SyntaxElement::IfStatement {
//         condition: Box::new(if_condition),
//         then_branch: Box::new(vec![then_branch_node]),
//         else_branch: Some(Box::new(vec![else_branch_node])),
//     });

//     let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration {
//         name: "testFunctionWithIfElse".to_string(),
//         parameters: vec![],
//         return_type: Some(DataType::Integer),
//     });

//     function_declaration_node.add_child(if_statement);

    let ast: AST = wrap_in_tle(function_declaration_node);

    let mut symbol_table_stack = SymbolTableStack::new();

    symbol_table_stack.push(SymbolTable::new());

    let mod_ast: ModAST = ast_stitch(vec![(ast, symbol_table_stack)]);

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
    let while_condition = ASTNode::new(SyntaxElement::Literal {
        data_type: DataType::Boolean,
        value: "true".to_string(),
    });

//     let while_body_node = ASTNode::new(SyntaxElement::Return {
//         value: Box::new(ASTNode::new(SyntaxElement::Literal {
//             data_type: DataType::Integer,
//             value: "42",
//         })),
//     });

//     let while_statement = ASTNode::new(SyntaxElement::WhileLoop {
//         condition: Box::new(while_condition),
//         body: Box::new(vec![while_body_node]),
//     });

//     let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration {
//         name: "testFunctionWithWhileLoop".to_string(),
//         parameters: vec![],
//         return_type: Some(DataType::Integer),
//     });

//     function_declaration_node.add_child(while_statement);

    let ast: AST = wrap_in_tle(function_declaration_node);

    let mut symbol_table_stack = SymbolTableStack::new();

    symbol_table_stack.push(SymbolTable::new());

    let mod_ast: ModAST = ast_stitch(vec![(ast, symbol_table_stack)]);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(&module, "output_while_loop.ll"){
        Ok(_) => {},
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }
}

#[test]
fn test_function_with_do_while_loop() {
    let do_while_condition = ASTNode::new(SyntaxElement::Literal {
        data_type: DataType::Boolean,
        value: "true".to_string(),
    });

//     let do_while_body_node = ASTNode::new(SyntaxElement::Return {
//         value: Box::new(ASTNode::new(SyntaxElement::Literal {
//             data_type: DataType::Integer,
//             value: "24",
//         })),
//     });

//     let do_while_statement = ASTNode::new(SyntaxElement::DoWhileLoop {
//         body: Box::new(vec![do_while_body_node]),
//         condition: Box::new(do_while_condition),
//     });

//     let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration {
//         name: "testFunctionWithDoWhileLoop".to_string(),
//         parameters: vec![],
//         return_type: Some(DataType::Integer),
//     });

//     function_declaration_node.add_child(do_while_statement);

    let ast: AST = wrap_in_tle(function_declaration_node);

    let mut symbol_table_stack = SymbolTableStack::new();

    symbol_table_stack.push(SymbolTable::new());

    let mod_ast: ModAST = ast_stitch(vec![(ast, symbol_table_stack)]);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(&module, "output_do_while_loop_fn.ll"){
        Ok(_) => {},
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }
}
