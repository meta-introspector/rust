Step 1: Constant Sublanguage of Rust

Definition: The minimal subset of Rust containing only constant declarations.

Grammar:
const IDENTIFIER: TYPE = VALUE;

Example:
const BITS: i32 = 64;
const ENABLED: bool = true;
const MESSAGE: &str = "hello";

Profiling Evidence:
- Empty file: 0.000 MB perf data
- Constants-only: 0.000 MB perf data + 6 dead code warnings
- Both compile successfully as libraries
- Compiler processes constants but generates warnings for unused items

This defines the atomic unit of Rust's constant system - pure value bindings with explicit types.
