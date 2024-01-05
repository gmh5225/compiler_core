extern crate llvm_sys as llvm;
use std::collections::HashMap;
use std::ffi::CString;

pub trait ASTNode {
    fn codegen(&self) -> llvm::prelude::LLVMValueRef;
}

struct NumExpr {
    val: f64,
}

impl NumExpr {
    fn new(val : f64) -> Self {
        NumExpr { val }
    }
}

impl ASTNode for NumExpr {
    fn codegen(&self) -> llvm::prelude::LLVMValueRef {
        let context = unsafe {
            llvm::core::LLVMContextCreate()
        };
        let builder = unsafe {
            llvm::core::LLVMCreateBuilderInContext(context)
        };
        let module = unsafe {
            let module_name = CString::new("module_name").expect("Failed to create CString");
            llvm::core::LLVMModuleCreateWithNameInContext(module_name.as_ptr(), context) 
        };
        let named_values: HashMap<String, llvm::prelude::LLVMValueRef> = HashMap::new();
        unsafe {
            let ty = llvm::core::LLVMDoubleTypeInContext(context);
            llvm_sys::core::LLVMConstReal(ty, self.val)
        }
    }
}