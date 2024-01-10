extern crate llvm_sys as llvm;

use llvm::core; 
use std::ffi::CString;

/// basic addition
pub fn build_add(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, sum: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildAdd(builder, param_a, param_b, sum.as_ptr())
    }
}

/// basic subtraction
pub fn build_sub(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildSub(builder, param_a, param_b, name.as_ptr())
    }
}

/// basic multiplication
pub fn build_mul(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildMul(builder, param_a, param_b, name.as_ptr())
    }
}

/// basic division
pub fn build_div(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildSDiv(builder, param_a, param_b, name.as_ptr()) // Signed division
    }
}

/// modular arithmetic
pub fn build_rem(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildSRem(builder, param_a, param_b, name.as_ptr()) // Signed remainder
    }
}

/// logical and
pub fn build_and(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildAnd(builder, param_a, param_b, name.as_ptr())
    }
}

/// logical or
pub fn build_or(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildOr(builder, param_a, param_b, name.as_ptr())
    }
}

/// logical xor
pub fn build_xor(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildXor(builder, param_a, param_b, name.as_ptr())
    }
}

/// logical left shift
pub fn build_shl(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildShl(builder, param_a, param_b, name.as_ptr())
    }
}

/// logical right shift
pub fn build_lshr(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildLShr(builder, param_a, param_b, name.as_ptr())
    }
}

/// greater than
pub fn build_icmp_gt(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntSGT, param_a, param_b, name.as_ptr())
    }
}

/// less than
pub fn build_icmp_lt(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntSLT, param_a, param_b, name.as_ptr())
    }
}

/// equal
pub fn build_icmp_eq(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> *mut llvm::LLVMValue {
    unsafe {
        core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntEQ, param_a, param_b, name.as_ptr())
    }
}
