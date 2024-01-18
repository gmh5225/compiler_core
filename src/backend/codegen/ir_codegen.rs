use std::{ { ffi::CString, collections::HashMap },
            fs,
            path::Path, };

use llvm::{core, prelude::*}; // change to not use wild star import

use crate::frontend::{ 
    ast::ast_struct::{ AST, ASTNode, }, 
    ast::syntax_element::SyntaxElement, 
    ast::data_type:: DataType 
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

    pub fn generate_ir(ast: &AST) -> LLVMModuleRef {
        let mut ir_generator: IRGenerator = IRGenerator::new();

        let root: ASTNode = ast.get_root().clone();
        ir_generator.generate_node_ir(&ast.get_root());

        for child in &root.get_children() {
            ir_generator.generate_node_ir(&child);
        };
        ir_generator.module
    }

    fn generate_node_ir(&mut self, node: &ASTNode) -> LLVMValueRef {
        let generated_ir: LLVMValueRef = match &node.get_element() {
            SyntaxElement::ModuleExpression => {
                 std::ptr::null_mut()
            },
            SyntaxElement::Literal(data_type,
                                    value) => {
                self.generate_literal_ir(*data_type, value.to_string())                           
            },
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
            SyntaxElement::FunctionDeclaration { name, parameters, return_type } => {
                std::ptr::null_mut()
            }
            _ => unimplemented!("unimplemented expression")
        };

        for child in &node.get_children() {
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
            },
            DataType::String => {
                unimplemented!("abc")
            }
        }
    }
    fn write_ir_to_file(module: LLVMModuleRef) {
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