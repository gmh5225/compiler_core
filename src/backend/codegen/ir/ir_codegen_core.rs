extern crate llvm_sys as llvm;

use std::{
    sync::{
        Mutex, Arc
    },
    collections::BinaryHeap,
};
use llvm::{core, prelude::*}; // change to not use wild star import

use crate::{
    backend::{
        codegen::store::Store,
        llvm_lib::ir_lib::{block, init_ir},
    },
    frontend::{
        ast::{
            ast_struct::{ASTNode, ModElement, Module, AST},
            syntax_element::SyntaxElement, 
        }, symbol_table::symbol_table_struct::{SymbolTable, SymbolTableStack}
    },
};

/// Generates LLVM IR for a module
pub struct IRGenerator {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    store: Arc<Mutex<Store>>,
    current_function: Option<LLVMValueRef>,
    current_stack: Option<Arc<Mutex<SymbolTableStack>>>,
    current_stack_pointer: usize,
}

impl IRGenerator {
    fn new() -> Self {
        let context: LLVMContextRef = init_ir::create_context();
        let module: LLVMModuleRef = init_ir::create_module("dummy_module", context);
        let builder: LLVMBuilderRef = init_ir::create_builder(context);
        let store: Arc<Mutex<Store>> = Arc::new(Mutex::new(Store::new()));
        Self {
            context,
            module,
            builder,
            store,
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
    pub fn get_store(&mut self) -> &Arc<Mutex<Store>> {
        &self.store
    }

    pub fn increment_stack_pointer(&mut self) {
        self.current_stack_pointer += 1;
    }

    pub fn decrement_stack_pointer(&mut self) {
        self.current_stack_pointer -= 1;
    }

    pub fn reset_stack_pointer(&mut self) {
        self.current_stack_pointer = 0;
    }
    
    pub fn get_stack(&self) -> Option<Arc<Mutex<SymbolTableStack>>> {
        self.current_stack.clone()
    }

    pub fn set_stack(&mut self, new_stack: Arc<Mutex<SymbolTableStack>>) {
        self.current_stack = Some(new_stack);
    }

    pub fn get_stack_pointer(&self) -> usize {
        self.current_stack_pointer
    }

    /// Generates LLVM IR from a module
    pub fn generate_ir(mut input: Module) -> LLVMModuleRef {
        let mut ir_generator: IRGenerator = IRGenerator::new();

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
            SyntaxElement::ModuleExpression      |
            SyntaxElement::NoExpression          |
            SyntaxElement::TopLevelExpression => {
                for child in node.get_children().iter() {
                    self.ir_router(child);
                }
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
            SyntaxElement::BlockExpression => {
                self.generate_block_exp(node)
            }
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
            SyntaxElement::Literal(_) => {
                self.generate_literal_ir(node)                        
            },
            SyntaxElement::Variable => {
                self.generate_var_ir(node)
            },

            // TODO
            SyntaxElement::LoopIncrement => {
                std::ptr::null_mut()
            },
            SyntaxElement::Condition => {
                if !node.get_children().len() > 1 {
                    self.ir_router(&node.get_children()[0])
                }
                else {
                    panic!("Unexpected second node")
                }
            },
            SyntaxElement::Action => {
                std::ptr::null_mut()
            },
            SyntaxElement::Variant => {
                std::ptr::null_mut()
            },
            SyntaxElement::AssignedValue => {
                if !node.get_children().len() > 1 {
                    self.ir_router(&node.get_children()[0])
                }
                else {
                    panic!("Unexpected second node")
                }
            },
            SyntaxElement::Field => {
                std::ptr::null_mut()
            },
            SyntaxElement::Parameter => {
                std::ptr::null_mut()
            }
            SyntaxElement::LoopInitializer => {
                std::ptr::null_mut()
            }
            SyntaxElement::Mutable(_) => {
                std::ptr::null_mut()
            }
            SyntaxElement::Identifier(_) => {
               std::ptr::null_mut()
            }
            SyntaxElement::Operator(_) => {
                std::ptr::null_mut()
            }
            SyntaxElement::Operand => {
                std::ptr::null_mut()
            }
            SyntaxElement::Type(_) => {
                std::ptr::null_mut()
            }
            SyntaxElement::ElifStatement => {
                for child in node.get_children().iter() {
                    self.ir_router(child);
                }
                std::ptr::null_mut()
            }
            SyntaxElement::ElseStatement => {
                for child in node.get_children().iter() {
                    self.ir_router(child);
                }
                std::ptr::null_mut()
            }
            SyntaxElement::Break => {
                std::ptr::null_mut()
            }
            SyntaxElement::Continue => {
                std::ptr::null_mut()
            }
            SyntaxElement::MatchArm => {
                std::ptr::null_mut()
            }
        };
        node_ir
    }
}