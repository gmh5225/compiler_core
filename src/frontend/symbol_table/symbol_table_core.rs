use crate::
    frontend::{
        ast::{ast_struct::{ASTNode, AST}, syntax_element::SyntaxElement}, 
        symbol_table::symbol_table_struct::{SymbolTable,  SymbolTableStack},
        utils::error::ErrorType,
    }
;

impl SymbolTableStack {
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

    fn sym_table_stack_router(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let mut errors: Vec<ErrorType> = Vec::new();
            
        match node.get_element() {
            SyntaxElement::FunctionDeclaration { name, parameters, return_type } => {
                match self.sym_table_fn(name, parameters, return_type) {
                    Ok(_) => {}
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            },
            SyntaxElement::StructDeclaration { name, fields } => {
                match self.sym_table_struct(name, fields) {
                    Ok(_) => {}
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            },
            SyntaxElement::EnumDeclaration { name, variants } => {
                match self.sym_table_enum(name, variants) {
                    Ok(_) => {}
                    Err(e) => {
                        errors.extend(e);
                    }
                }
            },
            SyntaxElement::Initialization { variable, value, data_type } => {
                match self.sym_table_init(variable, value, data_type) {
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