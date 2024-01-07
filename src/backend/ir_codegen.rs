extern crate llvm_sys as llvm;
use std::{ffi::CString, collections::HashMap};

// impl ASTNode for NumExpr {
//     fn codegen(&self) -> llvm::prelude::LLVMValueRef {
//         unsafe {
//             let ty = llvm::core::LLVMDoubleTypeInContext(context);
//             llvm_sys::core::LLVMConstReal(ty, self.val)
//         }
//     }
// }

use llvm_sys::{core, prelude::*};
use crate::frontend::syntax::{ ast::{AST, ASTNode}, 
                               syntax_element::SyntaxElement, 
                               data_type:: DataType };

pub struct IRGenerator {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    named_values: HashMap<String, llvm::prelude::LLVMValueRef>
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

    pub fn generate_ir(ast: &AST) -> Result<(), String> {
        let mut ir_generator: IRGenerator = IRGenerator::new();
        ir_generator.generate_node_ir(&ast.root)
    }

    fn generate_node_ir(&mut self, node: &ASTNode) -> Result<(), String> {
        match &node.element {
            _ => {}
        }

        for child in &node.children {
            self.generate_node_ir(child)?;
        }

        Ok(())
    }
}