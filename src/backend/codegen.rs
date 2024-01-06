extern crate llvm_sys as llvm;
// use llvm::core::*;
// use llvm::prelude::*;
// use std::collections::HashMap;
// use std::ffi::CString;

// pub fn generate_ir(node: &ASTNode, 
//                     context: LLVMContextRef, 
//                     module: LLVMModuleRef, 
//                     builder: LLVMBuilderRef,
//                     named_values: Map<string, >) {
//     unsafe {
//         let context = "a";
//         let module = "b";
//         let builder = "c";
//     }
//     match node {
//         _ => "not implemented"
//     }
// }


// let context = unsafe {
//     llvm::core::LLVMContextCreate()
// };
// let builder = unsafe {
//     llvm::core::LLVMCreateBuilderInContext(context)
// };
// let module = unsafe {
//     let module_name = CString::new("module_name").expect("Failed to create CString");
//     llvm::core::LLVMModuleCreateWithNameInContext(module_name.as_ptr(), context) 
// };
// let named_values: HashMap<String, llvm::prelude::LLVMValueRef> = HashMap::new();


// impl ASTNode for NumExpr {
//     fn codegen(&self) -> llvm::prelude::LLVMValueRef {
//         unsafe {
//             let ty = llvm::core::LLVMDoubleTypeInContext(context);
//             llvm_sys::core::LLVMConstReal(ty, self.val)
//         }
//     }
// }