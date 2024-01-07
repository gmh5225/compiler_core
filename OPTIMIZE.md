# Places we can optimize
 - AST 
    - Constant folding: evaluating constant expressions
    - Constant propogation: replace variables that are constant with their values
    - Dead code elimition
    - Function inlining

  - IR 
    - Loop transformations: loop unrolling/loop fusion
    - Strength reduction: replace expensive operations with cheaper ones
    - Instruction transofmrations
    - Control flow optimizations: branch prediction/tail prediction

  - Machine code
    - Instruction scheduling 
    - Register allocation
    - Instruction transformations