use std::sync::{Arc, Mutex};

use compiler_core::{
    frontend::{
        ast::{
            syntax_element::SyntaxElement, 
            ast_struct::{
                ASTNode, AST, ModElement, ModAST
            }, 
            data_type::DataType
        }, 
        analysis::symbol_table::{SymbolTableStack, SymbolTable},
    }, 
    backend::{codegen::ir::ir_codegen_core::IRGenerator, llvm_lib::ir_lib::utils::write_to_file}
}
;

fn create_ast_node(element: SyntaxElement) -> ASTNode {
    ASTNode::new(element)
}

#[test]
fn test_function_declaration() {
    let symbol_table_stack: Arc<Mutex<SymbolTableStack>> = Arc::new(Mutex::new(SymbolTableStack::new()));

    let function_ast = AST::new(create_ast_node(SyntaxElement::FunctionDeclaration {
        name: "testFunction".to_string(),
        parameters: vec![],
        return_type: Some(DataType::Integer),
    }));

    let mod_element: ModElement = ModElement::new(function_ast, Arc::clone(&symbol_table_stack), 0);

    let mut mod_ast: ModAST = ModAST::new();
    mod_ast.add_child(mod_element);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(module, "output_simple_fn.ll"){
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!()
        }
    }
}

#[test]
fn test_function_with_if_else() {
    let symbol_table_stack: Arc<Mutex<SymbolTableStack>> = Arc::new(Mutex::new(SymbolTableStack::new()));
    {
        let mut stack = symbol_table_stack.lock().expect("failed to lock stack");
        let symbol_table = SymbolTable::new();
        stack.push(symbol_table)
    }

    let if_condition = ASTNode::new(SyntaxElement::Literal {
        data_type: DataType::Boolean,
        value: "true".to_string(),
    });

    let then_branch_node = ASTNode::new(SyntaxElement::Return {
        value: Box::new(ASTNode::new(SyntaxElement::Literal {
            data_type: DataType::Integer,
            value: "1".to_string(),
        })),
    });

    let else_branch_node = ASTNode::new(SyntaxElement::Return {
        value: Box::new(ASTNode::new(SyntaxElement::Literal {
            data_type: DataType::Integer,
            value: "0".to_string(),
        })),
    });

    let if_statement = ASTNode::new(SyntaxElement::IfStatement {
        condition: Box::new(if_condition),
        then_branch: Box::new(vec![then_branch_node]),
        else_branch: Some(Box::new(vec![else_branch_node])),
    });

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration {
        name: "testFunctionWithIfElse".to_string(),
        parameters: vec![],
        return_type: Some(DataType::Integer),
    });

    function_declaration_node.add_child(if_statement);

    let function_ast = AST::new(function_declaration_node);

    let mod_element: ModElement = ModElement::new(function_ast, Arc::clone(&symbol_table_stack), 0);

    let mut mod_ast: ModAST = ModAST::new();
    mod_ast.add_child(mod_element);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(module, "output_if_else_fn.ll"){
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!()
        }
    }
}

#[test]
fn test_function_with_while_loop() {
    let symbol_table_stack: Arc<Mutex<SymbolTableStack>> = Arc::new(Mutex::new(SymbolTableStack::new()));
    {
        let mut stack = symbol_table_stack.lock().expect("failed to lock stack");
        let symbol_table = SymbolTable::new();
        stack.push(symbol_table)
    }
    let while_condition = ASTNode::new(SyntaxElement::Literal {
        data_type: DataType::Boolean,
        value: "true".to_string(),
    });

    let while_body_node = ASTNode::new(SyntaxElement::Return {
        value: Box::new(ASTNode::new(SyntaxElement::Literal {
            data_type: DataType::Integer,
            value: "42".to_string(),
        })),
    });

    let while_statement = ASTNode::new(SyntaxElement::WhileLoop {
        condition: Box::new(while_condition),
        body: Box::new(vec![while_body_node]),
    });

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration {
        name: "testFunctionWithWhileLoop".to_string(),
        parameters: vec![],
        return_type: Some(DataType::Integer),
    });

    function_declaration_node.add_child(while_statement);

    let function_ast = AST::new(function_declaration_node);

    let mod_element: ModElement = ModElement::new(function_ast, Arc::clone(&symbol_table_stack), 0);

    let mut mod_ast: ModAST = ModAST::new();
    mod_ast.add_child(mod_element);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(module, "output_while_loop.ll"){
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!()
        }
    }
    
}

#[test]
fn test_function_with_do_while_loop() {
    let symbol_table_stack: Arc<Mutex<SymbolTableStack>> = Arc::new(Mutex::new(SymbolTableStack::new()));
    {
        let mut stack = symbol_table_stack.lock().expect("failed to lock stack");
        let symbol_table = SymbolTable::new();
        stack.push(symbol_table)
    }
    let do_while_condition = ASTNode::new(SyntaxElement::Literal {
        data_type: DataType::Boolean,
        value: "true".to_string(),
    });

    let do_while_body_node = ASTNode::new(SyntaxElement::Return {
        value: Box::new(ASTNode::new(SyntaxElement::Literal {
            data_type: DataType::Integer,
            value: "24".to_string(),
        })),
    });

    let do_while_statement = ASTNode::new(SyntaxElement::DoWhileLoop {
        body: Box::new(vec![do_while_body_node]),
        condition: Box::new(do_while_condition),
    });

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration {
        name: "testFunctionWithDoWhileLoop".to_string(),
        parameters: vec![],
        return_type: Some(DataType::Integer),
    });

    function_declaration_node.add_child(do_while_statement);

    let function_ast: AST = AST::new(function_declaration_node);

    let mod_element: ModElement = ModElement::new(function_ast, Arc::clone(&symbol_table_stack), 0);

    let mut mod_ast: ModAST = ModAST::new();
    mod_ast.add_child(mod_element);

    let module = IRGenerator::generate_ir(mod_ast);

    match write_to_file(module, "output_do_while_loop_fn.ll"){
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!()
        }
    }
}

