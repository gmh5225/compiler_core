extern crate llvm_sys as llvm;

use std::{ { ffi::CString, collections::HashMap },
            fs,
            path::Path, };

use llvm::{core, prelude::*};
use crate::frontend::syntax::{ ast::{ AST, ASTNode }, 
                               syntax_element::SyntaxElement, 
                               data_type:: DataType };

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
    use crate::backend::llvm::{
        init::{create_basic_block, create_builder, create_context, create_module},
        binops::build_add,
        return_type::{int_return, void_return},
        create_element::{create_function, void_type, int_type},
        utils::{get_param, write_to_file, position_builder_at_end}
    };

    fn init() -> (*mut llvm::LLVMContext, *mut llvm::LLVMModule, *mut llvm::LLVMBuilder) {
        let context: *mut llvm::LLVMContext = create_context();
        let module: *mut llvm::LLVMModule = create_module("test_module", context);
        let builder: *mut llvm::LLVMBuilder = create_builder(context); // basically a pointer to where the llvm ir will write
        (context, module, builder)
    }

    #[test]
    fn test_basic_block_creation() {
        let (context, module, builder) = init();

        let return_type: *mut llvm::LLVMType = void_type(context);
        let function: *mut llvm::LLVMValue = create_function("test_function", Some(return_type), &[], false, module);

        let bb: *mut llvm::LLVMBasicBlock = create_basic_block(context, function, "entry");

        position_builder_at_end(builder, bb);

        void_return(builder);

        write_to_file(module, "output_basic.ll");
    }

    #[test]
    fn test_basic_add_expression() {
        let (context, module, builder) = init();
    
        let int_type: *mut llvm::LLVMType = int_type(context);
        let param_types: Vec<*mut llvm::LLVMType> = vec![int_type, int_type]; 
    
        let function: *mut llvm::LLVMValue = create_function("add", Some(int_type), &param_types, false, module);
    
        let bb: *mut llvm::LLVMBasicBlock = create_basic_block(context, function, "entry");
        position_builder_at_end(builder, bb);
    
        let param_a: *mut llvm::LLVMValue = get_param(function, 0);
        let param_b: *mut llvm::LLVMValue = get_param(function, 1);

        let sum: CString = CString::new("sum").expect("Failed to create sum name");

        let sum_build: *mut llvm::LLVMValue = build_add(builder, param_a, param_b, sum);
    
        int_return(builder, sum_build);
    
        write_to_file(module, "output_add.ll");
    }
    
}