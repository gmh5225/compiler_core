use std::collections::BinaryHeap;
use std::{ffi::CString, collections::HashMap};

use llvm::{core, prelude::*}; // change to not use wild star import
use llvm::prelude::LLVMValueRef;

use crate::frontend::ast::ast_struct::{ModAST, ModElement};
use crate::frontend::{ 
    ast::ast_struct::{ AST, ASTNode, }, 
    ast::syntax_element::SyntaxElement, 
};

pub struct IRGenerator {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    named_values: HashMap<String, LLVMValueRef>
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
                named_values: HashMap::new()
            }
        }
    }

    pub fn get_context(&self) -> LLVMContextRef {
        self.context.clone()
    }
    pub fn get_module(&self) -> LLVMModuleRef {
        self.module.clone()
    }
    pub fn get_builder(&self) -> LLVMBuilderRef {
        self.builder.clone()
    }
    pub fn get_named_value(&self, name: &str) -> Option<LLVMValueRef> {
        self.named_values.get(name).cloned()
    }
    pub fn add_named_value(&mut self, name: String, value: LLVMValueRef) {
        self.named_values.insert(name, value);
    }
    
    pub fn remove_named_value(&mut self, name: &str) {
        self.named_values.remove(name);
    }

    pub fn generate_ir(input: &ModAST) -> LLVMModuleRef {
        let mut ir_generator: IRGenerator = IRGenerator::new();

        // let module: BinaryHeap<ModElement> = input.get_children();

        // while let Some(mod_element) = module.pop() {
        //     let ast: AST = mod_element.get_ast();
        //     if ast.get_root().get_element() == SyntaxElement::ModuleExpression {
        //         for child in ast.get_root().get_children() {
        //             ir_generator.ir_router(&child);
        //         }
        //     }
        // }
        ir_generator.module
    }

    fn ir_router(&mut self, node: &ASTNode) -> LLVMValueRef {
        let node_ir: LLVMValueRef = match &node.get_element() {
            SyntaxElement::ModuleExpression |
            SyntaxElement::TopLevelExpression => {
                std::ptr::null_mut()
            },

            // top level expressions
            SyntaxElement::FunctionDeclaration { name, parameters, return_type } => {
                self.generate_fn_declaration_ir(name, parameters, return_type)
            },
            SyntaxElement::EnumDeclaration { name, variants } => {
                self.generate_enum_declaration_ir(name, variants)
            },
            SyntaxElement::StructDeclaration { name, fields } => {
                self.generate_struct_declaration_ir(name, fields)

            },
            
            // block expresions
            SyntaxElement::DoWhileLoop { body, condition } => { // doing the important ones first of course
                self.generate_do_while_ir(body, condition)
            },
            SyntaxElement::WhileLoop { condition, body } => {
                self.generate_while_ir(condition, body)
            },
            SyntaxElement::ForLoop { initializer, condition, increment, body } => {
                self.generate_for_ir(initializer, condition, increment, body)
            },
            SyntaxElement::IfStatement { condition, then_branch, else_branch } => {
                self.generate_if_ir(condition, then_branch, else_branch)
            },
        
            // statements
            SyntaxElement::BinaryExpression { left, operator, right } => {
                self.generate_binary_exp_ir(left, operator, right)
                // let left_val = self.ir_router(left);
                // let right_val = self.ir_router(right); 
                // match operator.as_str() {
                //     "+" => unsafe {
                //         let tmp_name  = CString::new("addtmp")
                //             .expect("Failed to create CString for add");
                //         core::LLVMBuildAdd(self.builder, left_val, right_val, tmp_name.as_ptr())
                //     }
                //     _ => unimplemented!("unimplemented operator")
                    
                // }     
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
            SyntaxElement::Break => {
                self.generate_break_ir()
            },
            SyntaxElement::Continue => {
                self.generate_continue_ir()
            },
            SyntaxElement::UnaryExpression { operator, operand } => {
                self.generate_unary_ir()
            },
            SyntaxElement::Return { value } => {
                self.generate_return_ir(value)
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

        for child in &node.get_children() {
            self.ir_router(child);
        };

        node_ir // this may be unnecessary, consider removing later
    }
}