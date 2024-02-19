extern crate llvm_sys as llvm;

use std::collections::HashMap;

use llvm::prelude::LLVMValueRef;


pub struct Store {
    allocations: HashMap<String, LLVMValueRef>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
        }
    }

    pub fn add_allocation(&mut self, variable_name: String, allocation: LLVMValueRef) {
        self.allocations.insert(variable_name, allocation);
    }

    pub fn get_allocation(&self, variable_name: &str) -> Option<&LLVMValueRef> {
        self.allocations.get(variable_name)
    }
}
