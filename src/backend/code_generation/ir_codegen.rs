extern crate llvm_sys as llvm;
use std::{ffi::CString, collections::HashMap};
use std::fs;
use std::path::Path;

use llvm::{core, prelude::*};
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

    pub fn generate_ir(ast: &AST) -> LLVMModuleRef {
        let mut ir_generator: IRGenerator = IRGenerator::new();
        let root: ASTNode = ast.root.clone();
        ir_generator.generate_node_ir(&ast.root);
        for child in &root.children {
            ir_generator.generate_node_ir(&child);
        };
        ir_generator.module
    }

    fn generate_node_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        let generated_ir: LLVMValueRef = match &node.element {
            SyntaxElement::FileExpression => {
                 std::ptr::null_mut()
            }
            SyntaxElement::Literal(data_type,
                                    value) => {
                self.generate_literal_ir(*data_type, value.to_string())                           
            },
            // SyntaxElement::Variable(name) => {

            // },
            SyntaxElement::BinaryExpression{ left,
                                             operator,
                                             right, } => {
                let left_val = self.generate_node_ir(left);
                let right_val = self.generate_node_ir(right); 
                match operator.as_str() {
                    "+" => unsafe {
                        let tmp_name  = CString::new("addtmp")
                            .expect("Failed to create CString for add");
                        core::LLVMBuildAdd(self.builder, left_val, right_val, tmp_name.as_ptr())
                    }
                    _ => unimplemented!("unimplemented operator")
                    
                }                           
            },
            // SyntaxElement::IfStatement{ condition,
            //                             then_branch,
            //                             else_branch, } => {

            // },
            // SyntaxElement::Initialization { variable,
            //                                 value, } => {
                                            
            // },
            // SyntaxElement::Assignment { variable,
            //                             value, } => {

            // },
            _ => unimplemented!("unimplemented expression")
        };

        for child in &node.children {
            self.generate_node_ir(child);
        };
        generated_ir
    }

    fn generate_literal_ir(&self, data_type: DataType, value: String) -> LLVMValueRef {
        match data_type {
            DataType::Integer => {
                let val = match value.parse::<i64>() {
                    Ok(val) => val,
                    Err(e) => panic!("Failed to parse integer: {}", e),
                };
                unsafe {
                    core::LLVMConstInt(
                        core::LLVMInt64TypeInContext(self.context),
                        val as u64,
                        0 // isSigned flag
                    )
                }
            },
            DataType::Float => {
                let val = match value.parse::<f64>() {
                    Ok(val) => val,
                    Err(e) => panic!("Failed to parse floating point: {}", e),
                };
                unsafe {
                    core::LLVMConstReal(
                        core::LLVMFloatTypeInContext(self.context),
                        val
                    )
                }
            },
            DataType::Boolean => {
                let val = match value.parse::<bool>() {
                    Ok(val) => val,
                    Err(e) => panic!("Failed to parse boolean: {}", e),
                };
                unsafe {
                    let bool_type = core::LLVMInt1TypeInContext(self.context);
                    core::LLVMConstInt(bool_type, val as u64, 0)
                }
            }
        }
    }
    pub fn write_ir_to_file(module: LLVMModuleRef) {
        let output_dir = Path::new("src/backend/code_generation/target");
        let output_file_path = output_dir.join("output.ll");
    
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).expect("Failed to create target directory");
        }
    
        let output_file_cstr = CString::new(output_file_path.to_str().expect("Failed to convert path to string"))
            .expect("Failed to create CString for filename");
    
        unsafe {
            core::LLVMPrintModuleToFile(module, output_file_cstr.as_ptr(), std::ptr::null_mut());
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::syntax::{syntax_element::SyntaxElement, data_type::DataType};

    #[test]
    fn basic_test() {
        let left_node = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "5".to_string()));
        let right_node = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "3".to_string()));
        let binary_expr = ASTNode {
            element: SyntaxElement::BinaryExpression {
                left: Box::new(left_node),
                operator: "+".to_string(),
                right: Box::new(right_node),
            },
            children: vec![],
        };
        let mut root_node = ASTNode::new(SyntaxElement::FileExpression);
        root_node.children.push(binary_expr);
        let ast = AST::new(root_node);

        IRGenerator::write_ir_to_file(IRGenerator::generate_ir(&ast))
    }
}
