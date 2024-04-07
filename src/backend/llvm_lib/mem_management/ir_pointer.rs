extern crate llvm_sys as llvm;

use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct IRPointer<T> {
    ptr: *mut T, 
    _marker: PhantomData<T>, 
}

impl<T> IRPointer<T> {
    pub fn new(ptr: *mut T) -> Self {
        IRPointer { ptr, _marker: PhantomData }
    }

    pub fn as_ref(&self) -> *mut T {
        self.ptr 
    }
}