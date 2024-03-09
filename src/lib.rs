//! compiler_core includes:
//! - A `frontend` module for lexing, parsing, symbol table stack generation, and syntax analysis.
//! - A `backend` module for object and IR code generation..
//! - A `compiler` module that drives the compilation process.
//! - A `runner` module for executing compiled programs.
//! - A `constants` module for shared constants used throughout the crate.
//!
extern crate llvm_sys as llvm;
extern crate threadpool;

/// Frontend Module
pub mod frontend;

/// Backend Module
pub mod backend;

/// Compile Driver 
pub mod compiler;

/// Runs Executables
pub mod runner;

/// Constants
pub mod constants;