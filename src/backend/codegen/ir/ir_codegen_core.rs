use std::collections::BinaryHeap;
use std::sync::{
    Mutex, Arc
};
use std::ffi::CString;

use llvm::{core, prelude::*}; // change to not use wild star import
use llvm::prelude::LLVMValueRef;

use crate::frontend::{ 
    ast::ast_struct::{ 
        AST, 
        ASTNode, 
        ModAST,
        ModElement,
    }, 
    ast::syntax_element::SyntaxElement, 
    symbol_table::symbol_table_core::SymbolTableStack
};

pub struct IRGenerator {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    current_function: Option<LLVMValueRef>,
}

impl IRGenerator {
    fn new() -> Self {
        unsafe {
            let context: LLVMContextRef = core::LLVMContextCreate();
            let module_name: CString = CString::new("mymodule")
                .expect("Failed to create CString for module name");

            let module: LLVMModuleRef = core::LLVMModuleCreateWithNameInContext(
                module_name.as_ptr(),
                context
            );           
            let builder: LLVMBuilderRef = core::LLVMCreateBuilderInContext(context);

            Self {
                context,
                module,
                builder,
                current_function: None
            }
        }
    }

    pub fn get_context(&self) -> LLVMContextRef {
        self.context
    }
    pub fn get_current_function(&self) -> LLVMValueRef {
        self.current_function.expect("No function is currently being processed")
    }
    pub fn set_current_function(&mut self, function: LLVMValueRef) {
        self.current_function = Some(function)
    }
    pub fn get_module(&self) -> LLVMModuleRef {
        self.module
    }
    pub fn get_builder(&self) -> LLVMBuilderRef {
        self.builder
    }
    pub fn get_current_block(&self) -> LLVMBasicBlockRef {
        unsafe {
            core::LLVMGetInsertBlock(self.builder)
        }
    }

    pub fn generate_ir(mut input: ModAST) -> LLVMModuleRef {
        let mut ir_generator: IRGenerator = IRGenerator::new();

        let module: &mut BinaryHeap<ModElement> = input.get_children();

        while let Some(mod_element) = module.pop() {
            let ast: AST = mod_element.get_ast();
            let symbol_table_stack: Arc<Mutex<SymbolTableStack>> = mod_element.get_sym_table_stack();
            let root = ast.get_root();
            ir_generator.ir_router(&root, &symbol_table_stack);
            for child in ast.get_root().get_children() {
                ir_generator.ir_router(&child, &symbol_table_stack);
            }
            
        }
        ir_generator.module
    }

    pub fn ir_router(&mut self, node: &ASTNode, sym_table_stack: &Arc<Mutex<SymbolTableStack>>) -> LLVMValueRef {        
        let node_ir: LLVMValueRef = match &node.get_element() {
            SyntaxElement::ModuleExpression |
            SyntaxElement::TopLevelExpression => {
                std::ptr::null_mut()
            },

            // top level expressions
            SyntaxElement::FunctionDeclaration { name, parameters, return_type } => {
                self.generate_fn_declaration_ir(name, parameters, return_type, &node.get_children(), sym_table_stack)
            },
            SyntaxElement::EnumDeclaration { name, variants } => {
                self.generate_enum_declaration_ir(name, variants)
            },
            SyntaxElement::StructDeclaration { name, fields } => {
                self.generate_struct_declaration_ir(name, fields)
            },
            
            // block expresions
            SyntaxElement::DoWhileLoop { body, condition } => { // doing the important ones first of course
                self.generate_do_while_ir(body, condition, sym_table_stack)
            },
            SyntaxElement::WhileLoop { condition, body } => {
                self.generate_while_ir(condition, body, sym_table_stack)
            },
            SyntaxElement::ForLoop { initializer, condition, increment, body } => {
                self.generate_for_ir(initializer, condition, increment, body, sym_table_stack)
            },
            SyntaxElement::IfStatement { condition, then_branch, else_branch } => {
                self.generate_if_ir(condition, then_branch, else_branch, sym_table_stack)
            },
        
            // statements
            SyntaxElement::BinaryExpression { left, operator, right } => {
                self.generate_binary_exp_ir(left, operator, right, sym_table_stack)    
            },
            SyntaxElement::MatchStatement { to_match, arms } => {
                self.generate_match_ir(to_match, arms)
            },
            SyntaxElement::FunctionCall { name, arguments } => {
                self.generate_fn_call_ir(name, arguments)
            },
            SyntaxElement::Initialization { variable, data_type, value } => {
                self.generate_initialization_ir(variable, data_type, value)
            },
            SyntaxElement::Assignment { variable, value } => {
                self.generate_assignment_ir(variable, value)
            },
            SyntaxElement::UnaryExpression { operator, operand } => {
                self.generate_unary_ir(operator, operand, sym_table_stack)
            },
            SyntaxElement::Return { value } => {
                self.generate_return_ir(value, sym_table_stack)
            },
            
            // primitive
            SyntaxElement::Literal { data_type, value } => {
                self.generate_literal_ir(*data_type, value.to_string())                           
            },
            SyntaxElement::Variable { data_type, name } => {
                self.generate_var_ir(data_type, name)
            },

            _ => panic!("Unrecognized syntax element {:?}", node)

        };

        node_ir 
    }
}