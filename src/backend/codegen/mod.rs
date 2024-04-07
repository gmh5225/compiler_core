/// Generates LLVM IR from modules
pub mod ir;

/// Generates Object code from LLVM IR
pub mod object_codegen;

/// Maintains the LLVM IR allocations of the LLVM IR generative process
pub mod store;