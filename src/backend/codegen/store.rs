extern crate llvm_sys as llvm;

use std::collections::HashMap;

use llvm::prelude::LLVMValueRef;

/// Maintains allocations in LLVM IR generation
pub struct Store {
    allocations: HashMap<String, LLVMValueRef>,
}

impl Store {
    /// Creates a new store
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
        }
    }

    /// Adds an allocation
    pub fn add_allocation(&mut self, variable_name: String, allocation: LLVMValueRef) {
        self.allocations.insert(variable_name, allocation);
    }

    /// Retrieves an allocation
    pub fn get_allocation(&self, variable_name: &str) -> Option<&LLVMValueRef> {
        self.allocations.get(variable_name)
    }

    
}
