extern crate llvm_sys as llvm;
use llvm::core::*;
use llvm::prelude::*;

pub fn generate_ir(node: &ASTNode, 
                    context: LLVMContextRef, 
                    module: LLVMModuleRef, 
                    builder: LLVMBuilderRef,
                    named_values: Map<string, >) {
    unsafe {
        let context = "a";
        let module = "b";
        let builder = "c";
    }
    match node {
        _ => "not implemented"
    }
}
