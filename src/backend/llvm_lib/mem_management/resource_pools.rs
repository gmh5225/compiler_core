extern crate llvm_sys as llvm;

use std::collections::HashMap;

use crate::backend::llvm_lib::mem_management::ir_pointer::IRPointer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueHandle(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockHandle(usize);

pub struct LLVMResourcePools<T> {
    values: Option<HashMap<ValueHandle, IRPointer<T>>>,
    basic_blocks: Option<HashMap<BasicBlockHandle, IRPointer<T>>>,
    next_handle: usize, // Generates unique IDs
}

impl<T> LLVMResourcePools<T> {
    pub fn new() -> Self {
        Self {
            values: None,
            basic_blocks: None,
            next_handle: 0,
        }
    }

    pub fn get_value(&self, handle: ValueHandle) -> Option<&IRPointer<T>> {
        match &self.values {
            Some(values) => {
                values.get(&handle)
            }
            None => None,
        }
    }

    pub fn create_value_handle(&mut self, value: *mut T) -> ValueHandle {
        let handle = ValueHandle(self.next_handle);
        self.next_handle += 1;

        let pointer = IRPointer::new(value);
        if let Some(values) = self.values.as_mut() {
            values.insert(handle, pointer);
        } else {
            let mut map = HashMap::new();
            map.insert(handle, pointer);
            self.values = Some(map);
        }

        handle
    }


    pub fn get_basic_block(&self, handle: BasicBlockHandle) -> Option<&IRPointer<T>> {
        match &self.basic_blocks {
            Some(basic_blocks) => {
                basic_blocks.get(&handle)
            }
            None => None,
        }
    }

    pub fn create_basic_block_handle(&mut self, basic_block: *mut T) -> BasicBlockHandle {
        let handle = BasicBlockHandle(self.next_handle);
        self.next_handle += 1;

        let pointer = IRPointer::new(basic_block);
        if let Some(basic_blocks) = self.basic_blocks.as_mut() {
            basic_blocks.insert(handle, pointer);
        } else {
            let mut map = HashMap::new();
            map.insert(handle, pointer);
            self.basic_blocks = Some(map);
        }

        handle
    }
}
