# Compiler Design
- Independent Study sponsored by Aaron Cass.
- Students: Caleb L'Italien, John Daly, Thomas Breimer (tentatively)
------------------------------------
# Goals
- Compile basic parts of C
- Create the frontend in C++ (to utilize flex/bison)
- Create the backend in Rust, and generate LLVM IR
- Make the project workable on all types of machines
------------------------------------
# C++
- Documentation: https://en.cppreference.com/w/cpp
- Basic tutorials: https://cplusplus.com/doc/tutorial/
------------------------------------
# Rust
- Documentation: https://doc.rust-lang.org/book/
- Rustlings: https://github.com/rust-lang/rustlings
- Stanford Seminar Presenting Rust: https://www.youtube.com/watch?v=O5vzLKg7y-k&ab_channel=StanfordOnline
------------------------------------
# Resources
 - Textbook: Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jefrey D. Ullman. Compilers: Principles, Techniques, & Tools. Pearson Education, second edition, 2007
 - LLVM API: https://llvm.org/doxygen/group__LLVMC.html
 - Flex: https://www.cs.virginia.edu/~cr4bd/flex-manual/Introduction.html#Introduction
    - Flex patterns (for recognizing tokens): https://www.cs.virginia.edu/~cr4bd/flex-manual/Patterns.html#Patterns
 - Bison: https://www.gnu.org/software/bison/manual/bison.html
------------------------------------
# LLVM
- Download: https://releases.llvm.org/download.html#3.5
- Documentation: https://releases.llvm.org/17.0.1/docs/index.html
- To fix path on MacOS (Apple Sillicon chip):
   - echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> ~/.zshrc
   - source ~/.zshrc
   - llvm-config --version
- Developer package: https://packages.ubuntu.com/jammy/llvm-14-dev
------------------------------------
# Infrastructure
- TODO: Create syllabus/schedule
- TODO: Decide meeting times
------------------------------------
# General TODO
- TODO: Type-checking
- TODO: Symbol Table and Scope Management
- TODO: Memory Management
- TODO: Preprocessor
- TODO: Runtime Library
- TODO: Linking
- TODO: Optimizations
- TODO: Error Handling/Debugging
- TODO: Figure out if LLVM can work on all machines
