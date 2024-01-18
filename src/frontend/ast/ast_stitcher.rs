use crate::frontend::ast::{
    ast_struct::{
        ASTNode, 
        AST,
    },
    syntax_element::SyntaxElement,
};

pub fn ast_stitch(input: Vec<AST>) -> AST {
    let mut root: ASTNode = ASTNode::new(SyntaxElement::ModuleExpression);

    for ast in input {
        let root_node = ast.get_root(); 
        root.add_child(root_node);
    }

    AST::new(root)
}
