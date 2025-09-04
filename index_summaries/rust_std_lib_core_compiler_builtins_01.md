# Rust Standard Library Core and Compiler Builtins (Part 1)

This index file (`std_lib_components_01.txt`) lists `Cargo.toml` paths for core components of the Rust standard library and its underlying compiler builtins. These crates are fundamental for Rust program execution, providing basic functionalities like memory allocation and low-level operations.

**Crates included in this section:**
*   `./library/Cargo.toml`: The main `Cargo.toml` for the Rust standard library.
*   `./library/alloc/Cargo.toml`: Provides the `alloc` crate, which handles dynamic memory allocation.
*   `./library/alloctests/Cargo.toml`: Contains tests specifically for the `alloc` crate.
*   `./library/compiler-builtins/Cargo.toml`: The main `Cargo.toml` for the `compiler-builtins` crate, which provides implementations of functions that the compiler might generate (e.g., for integer arithmetic, floating-point operations).
*   `./library/compiler-builtins/builtins-shim/Cargo.toml`: A shim layer for compiler builtins.
*   `./library/compiler-builtins/builtins-test-intrinsics/Cargo.toml`: Tests for intrinsic functions within compiler builtins.
*   `./library/compiler-builtins/builtins-test/Cargo.toml`: General tests for compiler builtins.
