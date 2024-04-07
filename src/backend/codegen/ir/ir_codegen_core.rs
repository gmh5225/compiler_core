extern crate llvm_sys as llvm;

use std::{
    sync::{
        Mutex, Arc
    },
    collections::BinaryHeap,
};

use llvm::prelude::{
    LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMValueRef
};

use crate::{
    backend::{
        codegen::store::Store,
        llvm_lib::{
            ir_lib::{block, init_ir}, 
            mem_management::resource_pools::LLVMResourcePools
        },
    },
    frontend::{
        ast::{
            ast_struct::{ASTNode, ModElement, Module, AST},
            syntax_element::SyntaxElement, 
        }, 
        symbol_table::symbol_table_struct::SymbolTableStack
    },
};

/// Generates LLVM IR for a module
pub struct IRGenerator<T> {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    store: Arc<Mutex<Store>>,
    resource_pools: LLVMResourcePools<T>,
    current_function: Option<LLVMValueRef>,
    current_stack: Option<Arc<Mutex<SymbolTableStack>>>,
    current_stack_pointer: usize,
}

impl<T> IRGenerator<T> {
    fn new() -> Self {
        let context: LLVMContextRef = init_ir::create_context();
        let module: LLVMModuleRef = init_ir::create_module("dummy_module", context);
        let builder: LLVMBuilderRef = init_ir::create_builder(context);
        let store: Arc<Mutex<Store>> = Arc::new(Mutex::new(Store::new()));
        let resource_pools = LLVMResourcePools::new();
        Self {
            context,
            module,
            builder,
            store,
            resource_pools,
            current_function: None,
            current_stack: None,
            current_stack_pointer: 0,
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
        block::get_current_block(self.builder)
    }

    /// Retrieves the store
    pub fn get_store(&mut self) -> &Arc<Mutex<Store>> {
        &self.store
    }

    /// Increments the stack pointer
    pub fn increment_stack_pointer(&mut self) {
        self.current_stack_pointer += 1;
    }

    /// Decrements the stack pointer
    pub fn decrement_stack_pointer(&mut self) {
        self.current_stack_pointer -= 1;
    }

    /// Resets the stack pointer
    pub fn reset_stack_pointer(&mut self) {
        self.current_stack_pointer = 0;
    }
    
    /// Gets the current stack
    pub fn get_stack(&self) -> Option<Arc<Mutex<SymbolTableStack>>> {
        self.current_stack.clone()
    }

    /// Sets the current stack
    pub fn set_stack(&mut self, new_stack: Arc<Mutex<SymbolTableStack>>) {
        self.current_stack = Some(new_stack);
    }

    /// Gets the stack pointer
    pub fn get_stack_pointer(&self) -> usize {
        self.current_stack_pointer
    }

    /// Generates LLVM IR from a module
    pub fn generate_ir(mut input: Module) -> LLVMModuleRef {
        let mut ir_generator: IRGenerator<T> = IRGenerator::new();

        let module: &mut BinaryHeap<ModElement> = input.get_children();

        while let Some(mod_element) = module.pop() {
            let symbol_table_stack: Arc<Mutex<SymbolTableStack>> = mod_element.get_sym_table_stack();
            ir_generator.set_stack(symbol_table_stack);
            ir_generator.reset_stack_pointer();

            let ast: AST = mod_element.get_ast();
            let root = ast.get_root();
            ir_generator.ir_router(&root);
        }
        ir_generator.module
    }

    /// Routes the LLVM IR generation process
    pub fn ir_router(&mut self, node: &ASTNode) -> LLVMValueRef {
        let node_ir = match &node.get_element() {
            // --- MODULE & SCOPING SECTION --- //
            SyntaxElement::ModuleExpression |
            SyntaxElement::NoExpression |
            SyntaxElement::TopLevelExpression => {
                for child in node.get_children().iter() {
                    self.ir_router(child);
                }
                std::ptr::null_mut()
            },

            // --- DECLARATION SECTION --- //
            SyntaxElement::FunctionDeclaration => self.generate_fn_declaration_ir(node),

            // --- BASE EXPRESSION SECTION --- //
            SyntaxElement::BlockExpression => self.generate_block_exp(node),
            SyntaxElement::DoWhileLoop => self.generate_do_while_ir(node),
            SyntaxElement::WhileLoop => self.generate_while_ir(node),
            SyntaxElement::ForLoop => self.generate_for_ir(node),
            SyntaxElement::IfStatement => self.generate_if_ir(node),

            // --- CONTROL FLOW SECTION --- //
            SyntaxElement::Assignment => self.generate_assignment_ir(node),
            SyntaxElement::Return => self.generate_return_ir(node),
            SyntaxElement::Condition => {
                if !node.get_children().len() > 1 {
                    self.ir_router(&node.get_children()[0])
                } else {
                    panic!("Unexpected second node")
                }
            },

            // --- LOOP CONTROL SECTION --- //
            SyntaxElement::LoopIncrement | 
            SyntaxElement::Action |
            SyntaxElement::Variant |
            SyntaxElement::AssignedValue => {
                if !node.get_children().len() > 1 {
                    self.ir_router(&node.get_children()[0])
                } else {
                    panic!("Unexpected second node")
                }
            },
            SyntaxElement::Break => self.generate_break_ir(node),
            SyntaxElement::Continue => self.generate_continue_ir(node),
            SyntaxElement::ElifStatement |
            SyntaxElement::ElseStatement |
            SyntaxElement::MatchArm => {
                for child in node.get_children().iter() {
                    self.ir_router(child);
                }
                std::ptr::null_mut()
            },

            // --- PRIMITIVE SECTION --- //
            SyntaxElement::Literal(_) => self.generate_literal_ir(node),

            // --- TODO SECTION --- //
            SyntaxElement::Variable |
            SyntaxElement::UnaryExpression |
            SyntaxElement::Field |
            SyntaxElement::Parameter |
            SyntaxElement::BinaryExpression |
            SyntaxElement::MatchStatement |
            SyntaxElement::FunctionCall |
            SyntaxElement::EnumDeclaration |
            SyntaxElement::StructDeclaration |
            SyntaxElement::Initialization |
            SyntaxElement::LoopInitializer |
            SyntaxElement::Mutable(_) |
            SyntaxElement::Identifier(_) |
            SyntaxElement::Operator(_) |
            SyntaxElement::Operand |
            SyntaxElement::Type(_) => std::ptr::null_mut(),
        };
        node_ir
    }

}