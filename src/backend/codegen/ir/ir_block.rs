use std::sync::{Mutex, Arc};

use crate::{
    backend::{
        codegen::ir::ir_codegen_core::IRGenerator, 
        llvm_lib::ir_lib::{
            block::position_builder, element::{create_br, create_cond_br}, init_ir::create_basic_block
        }
    }, frontend::{
        ast::{ast_struct::ASTNode, syntax_element::SyntaxElement}, 
        symbol_table::symbol_table_struct::SymbolTableStack
    } 
};

use llvm::prelude::LLVMValueRef;
use llvm_sys::{LLVMBasicBlock, LLVMValue};

impl IRGenerator {
    /// Generates LLVM IR for a do while loop
    pub fn generate_do_while_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        if let SyntaxElement::DoWhileLoop = node.get_element() {
            let children: Vec<ASTNode> = node.get_children();
    
            let mut body: Option<&ASTNode> = None;
            let mut condition_node: Option<&ASTNode> = None;
    
            for child in children.iter() {
                match child.get_element() {
                    SyntaxElement::BlockExpression => {
                        body = Some(child);
                    },
                    SyntaxElement::Condition => {
                        condition_node = Some(child);
                    },
                    _ => panic!("Unexpected node DoWhileLoop: {:?}", child),
                }
            }
    
            let function: *mut LLVMValue = self.get_current_function();
            let do_body_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "do_body");
            let do_cond_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "do_cond");
            let do_end_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "do_end");
    
            let entry_bb: *mut LLVMBasicBlock = self.get_current_block();

            position_builder(self.get_builder(), entry_bb);
            create_br(self.get_builder(), do_body_bb);
    
            position_builder(self.get_builder(), do_body_bb);

            match body {
                Some(block_exp) => {
                    self.ir_router(block_exp); 
                }
                _ => panic!("Missing body")
            }
            
            create_br(self.get_builder(), do_cond_bb);
    
            position_builder(self.get_builder(), do_cond_bb);

            if let Some(condition) = condition_node {
                let condition_val = self.ir_router(condition); // Assuming ir_router returns an LLVMValueRef
                create_cond_br(self.get_builder(), condition_val, do_body_bb, do_end_bb);
            } else {
                panic!("DoWhileLoop missing condition node");
            }
    
            position_builder(self.get_builder(), do_end_bb);
    
            std::ptr::null_mut()
        } else {
            panic!("Expected DoWhileLoop node, got: {:?}", node.get_element());
        }
    }
    
    
    /// Generates LLVM IR for a while loop
    pub fn generate_while_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        if let SyntaxElement::WhileLoop = node.get_element() {
            let children: Vec<ASTNode> = node.get_children();

            let mut condition_node: Option<&ASTNode> = None;
            let mut body_node: Option<&ASTNode> = None;

            for child in children.iter() {
                match child.get_element() {
                    SyntaxElement::Condition => {
                        condition_node = Some(child);
                    },
                    SyntaxElement::BlockExpression => {
                        body_node = Some(child);
                    },
                    _ => {
                        panic!("Unexpected node WhileLoop: {:?}", child)
                    }
                }
            }

            let function: *mut LLVMValue = self.get_current_function();
            let while_cond_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "while_cond");
            let while_body_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "while_body");
            let while_end_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "while_end");

            let entry_bb: *mut LLVMBasicBlock = self.get_current_block();

            position_builder(self.get_builder(), entry_bb);
            create_br(self.get_builder(), while_cond_bb);

            position_builder(self.get_builder(), while_cond_bb);
            let condition_val = match condition_node {
                Some(node) => self.ir_router(node),
                None => panic!("While loop missing condition node"),
            };

            create_cond_br(self.get_builder(), condition_val, while_body_bb, while_end_bb);

            position_builder(self.get_builder(), while_body_bb);
            if let Some(body) = body_node {
                self.ir_router(body); 
            } else {
                panic!("While loop missing body node");
            }

            create_br(self.get_builder(), while_cond_bb);

            position_builder(self.get_builder(), while_end_bb);

            std::ptr::null_mut()
        } else {
            panic!("Expected WhileLoop node, got: {:?}", node.get_element());
        }
    }

    
    /// Generates LLVM IR for a for loop
    pub fn generate_for_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        if let SyntaxElement::ForLoop = node.get_element() {
            let children: Vec<ASTNode> = node.get_children();

            let mut initializer_node: Option<&ASTNode> = None;
            let mut condition_node: Option<&ASTNode> = None;
            let mut increment_node: Option<&ASTNode> = None;
            let mut body_node: Option<&ASTNode> = None;

            for child in children.iter() {
                match child.get_element() {
                    SyntaxElement::LoopInitializer => initializer_node = Some(child),
                    SyntaxElement::Condition => condition_node = Some(child),
                    SyntaxElement::LoopIncrement => increment_node = Some(child),
                    SyntaxElement::BlockExpression => body_node = Some(child),
                    _ => panic!("Unexpected node within ForLoop: {:?}", child),
                }
            }

            let function: *mut LLVMValue = self.get_current_function();
            let for_cond_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "for_cond");
            let for_body_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "for_body");
            let for_inc_bb: Option<*mut LLVMBasicBlock> = increment_node.map(|_| create_basic_block(self.get_context(), function, "for_inc"));
            let for_end_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "for_end");

            let entry_bb: *mut LLVMBasicBlock = self.get_current_block();
            position_builder(self.get_builder(), entry_bb);

            if let Some(init_node) = initializer_node {
                self.ir_router(init_node);
            }
            create_br(self.get_builder(), for_cond_bb);

            position_builder(self.get_builder(), for_cond_bb);
            let condition_val: *mut LLVMValue = condition_node.map_or_else(
                || panic!("For loop missing condition node"),
                |cond| self.ir_router(cond),
            );
            create_cond_br(self.get_builder(), condition_val, for_body_bb, for_end_bb);

            position_builder(self.get_builder(), for_body_bb);
            if let Some(body) = body_node {
                self.ir_router(body);
            }

            match for_inc_bb {
                Some(inc_bb) => {
                    create_br(self.get_builder(), inc_bb);
                    position_builder(self.get_builder(), inc_bb);
                    if let Some(inc_node) = increment_node {
                        self.ir_router(inc_node);
                    }
                    create_br(self.get_builder(), for_cond_bb);
                },
                None => {
                    create_br(self.get_builder(), for_cond_bb);
                },
            }

            position_builder(self.get_builder(), for_end_bb);

            std::ptr::null_mut()
        } else {
            panic!("Expected ForLoop node, got: {:?}", node.get_element());
        }
    }


    /// Generates LLVM IR for an if statement
    pub fn generate_if_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        if let SyntaxElement::IfStatement = node.get_element() {
            let children: Vec<ASTNode> = node.get_children();
    
            let mut condition_node: Option<&ASTNode> = None;
            let mut then_branch: Option<&ASTNode> = None;
            let mut else_branch: Option<&ASTNode> = None;
    
            for child in children.iter() {
                match child.get_element() {
                    SyntaxElement::Condition => {
                        condition_node = Some(child)
                    }
                    SyntaxElement::Action => {
                        then_branch = Some(child)
                    },
                    SyntaxElement::ElseStatement => {
                        else_branch = Some(child)
                    },
                    _ => {
                        panic!("Unexpected node2: {:?}", child)
                    }
                }
            }
    
            let function: *mut LLVMValue = self.get_current_function();
            let then_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "then");
            let else_bb: Option<*mut LLVMBasicBlock> = else_branch.as_ref().map(|_| create_basic_block(self.get_context(), function, "else"));
            let merge_bb: *mut LLVMBasicBlock = create_basic_block(self.get_context(), function, "merge");
    
            let condition_val: *mut LLVMValue = condition_node.map_or_else(
                || panic!("If statement missing condition node"),
                |cond| self.ir_router(cond),
            );
            create_cond_br(self.get_builder(), condition_val, then_bb, else_bb.unwrap_or(merge_bb));
    
            position_builder(self.get_builder(), then_bb);
            if let Some(node) = then_branch {
                self.ir_router(node);
            }
            create_br(self.get_builder(), merge_bb);
    
            if let Some(node) = else_branch {
                position_builder(self.get_builder(), else_bb.unwrap());
                
                self.ir_router(node);
                
                create_br(self.get_builder(), merge_bb);
            }
    
            position_builder(self.get_builder(), merge_bb);
    
            std::ptr::null_mut()
        } else {
            panic!("Expected IfStatement node, got: {:?}", node.get_element());
        }
    }
} 