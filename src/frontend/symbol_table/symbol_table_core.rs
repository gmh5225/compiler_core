use crate::
    frontend::{
        ast::{ast_struct::{ASTNode, AST}, syntax_element::SyntaxElement}, 
        symbol_table::symbol_table_struct::{SymbolTable,  SymbolTableStack},
        utils::error::ErrorType,
    }
;

impl SymbolTableStack {
    /// Drives the symbol table stack generation process returns back the original ast and the generated symbol table stack, else errors
    pub fn gen_sym_table_stack(ast: AST) -> Result<(AST, SymbolTableStack), Vec<ErrorType>> {
        let mut sym_table_stack: SymbolTableStack = SymbolTableStack::new();
        let global_scope: SymbolTable = SymbolTable::new(); 
        sym_table_stack.push(global_scope);

        let mut errors: Vec<ErrorType> = Vec::new();

        match sym_table_stack.sym_table_stack_router(&ast.get_root()) {
            Ok(_) => {},
            Err(e) => errors.extend(e),
        }

        for child in ast.get_root().get_children() {
            match sym_table_stack.sym_table_stack_router(&child) {
                Ok(_) => {},
                Err(e) => errors.extend(e),
            }
        }

        if errors.is_empty() {
            return Ok((ast, sym_table_stack));
        }
        Err(errors)

    }

    pub fn sym_table_stack_router(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let mut errors: Vec<ErrorType> = Vec::new();
            
        match node.get_element() {
            SyntaxElement::FunctionDeclaration => {
                match self.sym_table_fn(node) {
                    Ok(_) => {}
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            },
            SyntaxElement::StructDeclaration => {
                match self.sym_table_struct(node) {
                    Ok(_) => {}
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            },
            SyntaxElement::EnumDeclaration => {
                match self.sym_table_enum(node) {
                    Ok(_) => {}
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            },
            SyntaxElement::Initialization => {
                match self.sym_table_init(node) {
                    Ok(_) => {}
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            },
            _ => {},
        }
        for child in &node.get_children() {
            self.sym_table_stack_router(child)?;
        }
        Ok(())
    }
}