use std::collections::HashMap;

use llvm_sys::prelude::LLVMValueRef;

use super::ir_pointer::IRPointer;

/// --- Handle Types --- ///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueHandle(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockHandle(usize);

/// --- Resource Pools --- ///
struct LLVMResourcePools {
    values: Option<HashMap<ValueHandle, IRPointer>>,
    basic_blocks: Option<HashMap<BasicBlockHandle, IRPointer>>,
    next_handle: usize, // Generates unique IDs
}

impl LLVMResourcePools {
    fn new() -> Self {
        Self {
            values: None,
            basic_blocks: None,
            next_handle: 0,
        }
    }

    fn get_value(&self, handle: ValueHandle) -> Option<&IRPointer> {
        match &self.values {
            Some(values) => {
                values.get(&handle)
            }
            None => None,
        }
    }

    fn create_value(&mut self, value: LLVMValueRef) -> ValueHandle {
        let handle = ValueHandle(self.next_handle);
        self.next_handle += 1;

        // match &self.values {
        //     Some(values) => {
        //         values.insert(handle, value);
        //     }
        // }

        handle
    }
}
