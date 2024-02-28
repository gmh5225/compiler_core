use std::sync::{Arc, Mutex};

use crate::frontend::{
    ast::{
        ast_struct::{
            ASTNode, 
            AST,
            ModAST,
            ModElement,
        },
        syntax_element::SyntaxElement,
    }, 
    symbol_table::symbol_table_core::SymbolTableStack
};


pub fn ast_stitch(input: Vec<(AST, SymbolTableStack)>) -> ModAST {
    let mut mod_ast: ModAST = ModAST::new(); 

    for (ast, sym_table_stack) in input {
        let root_node: ASTNode = ast.get_root();
        let priority: i32 = get_ast_priority(root_node);

        let arc_mutex_sym_table_stack: Arc<Mutex<SymbolTableStack>> = Arc::new(Mutex::new(sym_table_stack));
        
        let mod_element: ModElement = ModElement::new(ast, arc_mutex_sym_table_stack, priority);

        mod_ast.add_child(mod_element); 
    }

    mod_ast
}

fn get_ast_priority(ast_root: ASTNode) -> i32 {
    match ast_root.get_element() {
        SyntaxElement::ModuleExpression => {
            return 20;
        },
        SyntaxElement::TopLevelExpression => {
            return 10;
        },
        _ => panic!("Not a valid root expression: {:?}", ast_root.get_element())
    }
}