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
        Module,
        ModElement,
    }, 
    ast::syntax_element::SyntaxElement, 
    symbol_table::symbol_table_struct::SymbolTableStack
};

/// Generates LLVM IR for a module
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

    /// Retrieves the current context
    pub fn get_context(&self) -> LLVMContextRef {
        self.context
    }
    /// Retrieves the current function being built
    pub fn get_current_function(&self) -> LLVMValueRef {
        self.current_function.expect("No function is currently being processed")
    }
    /// Sets the current function being built
    pub fn set_current_function(&mut self, function: LLVMValueRef) {
        self.current_function = Some(function)
    }
    /// Retrieves the module
    pub fn get_module(&self) -> LLVMModuleRef {
        self.module
    }
    /// Retrieves the builder
    pub fn get_builder(&self) -> LLVMBuilderRef {
        self.builder
    }
    /// Retrieves the current insert block
    pub fn get_current_block(&self) -> LLVMBasicBlockRef {
        unsafe {
            core::LLVMGetInsertBlock(self.builder)
        }
    }

    /// Generates LLVM IR from a module
    pub fn generate_ir(mut input: Module) -> LLVMModuleRef {
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

    /// Routes the LLVM IR generation process
    pub fn ir_router(&mut self, node: &ASTNode, sym_table_stack: &Arc<Mutex<SymbolTableStack>>) -> LLVMValueRef {        
        let node_ir: LLVMValueRef = match &node.get_element() {
            SyntaxElement::ModuleExpression |
            SyntaxElement::TopLevelExpression => {
                std::ptr::null_mut()
            },

            // top level expressions
            SyntaxElement::FunctionDeclaration => {
                self.generate_fn_declaration_ir(node)
            },
            SyntaxElement::EnumDeclaration => {
                self.generate_enum_declaration_ir(node)
            },
            SyntaxElement::StructDeclaration => {
                self.generate_struct_declaration_ir(node)
            },
            
            // block expresions
            SyntaxElement::DoWhileLoop => { 
                self.generate_do_while_ir(node)
            },
            SyntaxElement::WhileLoop => {
                self.generate_while_ir(node)
            },
            SyntaxElement::ForLoop => {
                self.generate_for_ir(node)
            },
            SyntaxElement::IfStatement => {
                self.generate_if_ir(node)
            },
        
            // statements
            SyntaxElement::BinaryExpression => {
                self.generate_binary_exp_ir(node)    
            },
            SyntaxElement::MatchStatement => {
                self.generate_match_ir(node)
            },
            SyntaxElement::FunctionCall => {
                self.generate_fn_call_ir(node)
            },
            SyntaxElement::Initialization => {
                self.generate_initialization_ir(node)
            },
            SyntaxElement::Assignment => {
                self.generate_assignment_ir(node)
            },
            SyntaxElement::UnaryExpression => {
                self.generate_unary_ir(node)
            },
            SyntaxElement::Return => {
                self.generate_return_ir(node)
            },
            
            // primitive
            SyntaxElement::Literal(value) => {
                self.generate_literal_ir(node)                           
            },
            SyntaxElement::Variable => {
                self.generate_var_ir(node)
            },
            SyntaxElement::NoExpression => todo!(),
            SyntaxElement::Mutable(_) => todo!(),
            SyntaxElement::Identifier(_) => todo!(),
            SyntaxElement::Operator(_) => todo!(),
            SyntaxElement::Operand => todo!(),
            SyntaxElement::Type(_) => todo!(),
            SyntaxElement::ElifStatement => todo!(),
            SyntaxElement::ElseStatement => todo!(),
            SyntaxElement::Break => todo!(),
            SyntaxElement::Continue => todo!(),
            SyntaxElement::MatchArm => todo!(),
            SyntaxElement::BlockExpression => todo!(),
            SyntaxElement::LoopInitializer => todo!(),
            SyntaxElement::LoopIncrement => todo!(),
            SyntaxElement::Condition => todo!(),
            SyntaxElement::Action => todo!(),
            SyntaxElement::Variant => todo!(),
            SyntaxElement::AssignedValue => todo!(),
            SyntaxElement::Field => todo!(),
            SyntaxElement::Parameter => todo!(),
        };

        node_ir 
    }
}