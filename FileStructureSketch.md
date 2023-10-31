- frontend/
│   ├── src/
│   │   ├── lexer.l                # Flex lexer definition
│   │   ├── parser.y               # Bison parser definition
│   │   ├── SemanticAnalyzer.cpp   # Handles semantic checks
│   │   ├── ASTBuilder.cpp         # Functions to build AST nodes
│   │   ├── SymbolTable.cpp        # Symbol table management
│   │   ├── IntermediateCode.cpp   # Generates intermediate representation
│   │   ├── FrontendOptimizer.cpp  # Frontend-specific optimization
│   │   └── ErrorHandler.cpp       # Handles and reports errors in parsing or semantics
│   ├── include/
│   │   ├── SemanticAnalyzer.h
│   │   ├── ASTBuilder.h
│   │   ├── SymbolTable.h
│   │   ├── IntermediateCode.h
│   │   ├── FrontendOptimizer.h
│   │   └── ErrorHandler.h
 
-  backend/
│   ├── src/
│   │   ├── llvm_gen/
│   │   │   ├── mod.rs             # Module file for LLVM generation
│   │   │   ├── codegen.rs         # Core code generation functions
│   │   │   └── helpers.rs         # Helper functions for codegen
│   │   ├── optimizer/
│   │   │   ├── mod.rs             # Module file for optimizations
│   │   │   ├── inline.rs          # Inlining optimizations
│   │   │   ├── loop_unroll.rs     # Loop unrolling optimizations
│   │   │   └── register_alloc.rs  # Register allocation functions
│   │   ├── runtime/
│   │   │   ├── mod.rs             # Module file for runtime support
│   │   │   ├── memory.rs          # Memory allocation functions
│   │   │   └── garbage_collector.rs # If your language has garbage collection
│   │   ├── debug/
│   │   │   ├── mod.rs             # Module file for debugging facilities
│   │   │   └── symbols.rs         # Symbol resolution for debugging
│   │   └── main.rs                # Main function to tie all modules together


