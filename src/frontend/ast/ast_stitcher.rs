use crate::frontend::ast::{
        ast_struct::{
            ASTNode, 
            ModAST,
            ModElement,
        },
        syntax_element::SyntaxElement,
    };

/// Pieces together mod elements into a cohesive module
pub fn ast_stitch(input: Vec<ModElement>) -> ModAST {
    let mut mod_ast: ModAST = ModAST::new(); 
    for mut mod_element in input {
        let root = mod_element.get_ast().get_root();
        let priority = get_ast_priority(root);
        mod_element.set_priority(priority);
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