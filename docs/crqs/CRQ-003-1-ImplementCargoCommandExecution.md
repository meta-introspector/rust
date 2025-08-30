# Change Request (CRQ): Rust Bootstrap - Implement Cargo Command Execution

## 1. Objective
To enhance the existing `run_cargo_command` function within `rust-bootstrap`'s Cargo integration module to dispatch and execute arbitrary Cargo commands using the `cargo` crate's API, taking parsed arguments as input.

## 2. Scope
This CRQ covers:
*   Refactoring the existing `run_cargo_command` function in `crates/rust-bootstrap/src/cargo_integration/mod.rs` to dispatch based on the Cargo subcommand.
*   Utilization of the `cargo` crate's API (e.g., `cargo::ops::compile`, `cargo::ops::check`, etc.) for command execution.
*   Handling of parsed Cargo arguments and environment setup for the command.
*   Adding placeholders for future subcommand implementations.

## 3. Prerequisites
*   Completion of argument parsing within `rust-bootstrap`'s `cargo_integration` module.
*   Familiarity with the `cargo` crate's internal API for command execution.

## 4. Procedure

### Step 4.1: Refactor `run_cargo_command` for Subcommand Dispatch.
*   **Action:** Modify the `run_cargo_command` function in `crates/rust-bootstrap/src/cargo_integration/mod.rs` to use a `match` statement on the `subcommand_name` obtained from `parse_global_args`.
*   **Tool:** `replace`

### Step 4.2: Implement Dispatch Logic for `check` and `build`.
*   **Action:** Within the `match` statement, for `check` and `build` subcommands, call `cargo::ops::compile` (as both use `CompileOptions`).
*   **Tool:** `replace`

### Step 4.3: Add Placeholders for Other Subcommands.
*   **Action:** Include `TODO` comments for other common Cargo subcommands (e.g., `test`, `run`, `clean`), indicating that they will require their own `cargo::ops` functions and potentially different option structs.
*   **Tool:** `replace`

## 5. Verification
*   **Step 5.1:** Add a basic unit test for `execute_cargo_command` that attempts to run a simple Cargo command (e.g., `cargo check --workspace`) and verifies its successful execution.
*   **Step 5.2:** Run `cargo check` and `cargo test` to ensure compilation and test pass without warnings or errors.

## 6. Tools Used
*   `write_file`: For creating new Rust source files.
*   `replace`: For modifying existing Rust source files (e.g., `mod.rs`).
*   `run_shell_command`: For `cargo check` and `cargo test` verification.

## Progress Update (2025-08-30)
This CRQ is newly created to detail the implementation of Cargo command execution.
