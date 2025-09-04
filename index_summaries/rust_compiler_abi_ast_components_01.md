# Rust Compiler ABI and AST Components (Part 1)

This index file (`compiler_components_01.txt`) lists `Cargo.toml` paths for foundational components of the Rust compiler. These crates are primarily responsible for handling the Application Binary Interface (ABI) and various aspects of the Abstract Syntax Tree (AST), which is the initial parsed representation of Rust code.

**Crates included in this section:**
*   `./compiler/rustc_abi/Cargo.toml`: Defines the ABI for various platforms and calling conventions.
*   `./compiler/rustc_arena/Cargo.toml`: Provides an arena allocator for efficient memory management of AST nodes and other compiler data structures.
*   `./compiler/rustc_ast/Cargo.toml`: Contains the core AST definitions for Rust.
*   `./compiler/rustc_ast_ir/Cargo.toml`: Likely defines intermediate representations related to the AST.
*   `./compiler/rustc_ast_lowering/Cargo.toml`: Handles the lowering of the AST to a lower-level intermediate representation (e.g., HIR).
*   `./compiler/rustc_ast_passes/Cargo.toml`: Contains various passes that operate on the AST.
*   `./compiler/rustc_ast_pretty/Cargo.toml`: Provides pretty-printing functionality for the AST, useful for debugging and visualization.
