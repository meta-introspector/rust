# Change Request (CRQ): Monadic Refactoring and Direct Cargo Integration

## 1. Objective
To refactor the `rust-bootstrap` codebase into a highly modular, functionally composed system, and to directly integrate Cargo's functionality as library calls, moving away from external shell command execution. This aligns with the principles outlined in `SOP-MonadicRefactoringAndAutomatedIntegration.md`.

## 2. Scope
This CRQ covers:
*   Implementation of extreme modularity ("one declaration per file") across `rust-bootstrap`.
*   Integration of `build.rs` for automated generation of `clap` argument and subcommand providers.
*   Direct integration of `cargo` library functions for operations like `build` and `check`, replacing external `cargo` process calls.
*   Refactoring of command execution to support a more controlled, monadic flow.
*   Use of `tracing` for enhanced logging and observability.

## 3. Prerequisites
*   Understanding of Rust's module system, traits, and error handling.
*   Familiarity with `clap` for argument parsing.
*   Access to vendored `cargo` and `tracing` crates.

## 4. Procedure (High-Level)
### Step 4.1: Implement `build.rs` for Automated Provider Generation
*   **Action**: Create/modify `build.rs` to scan `src/cargo_integration/global_args` and `src/cargo_integration/subcommands` and generate `generated_clap_providers.rs`.
*   **Files**: `crates/rust-bootstrap/build.rs`

### Step 4.2: Refactor `cargo_integration` for Direct Cargo Calls
*   **Action**: Implement `run_cargo_command` to orchestrate Cargo operations using vendored `cargo` library.
*   **Action**: Develop `dispatch_cargo_command` to handle specific Cargo subcommands (e.g., `build`, `check`) via `cargo::ops::compile`.
*   **Action**: Create modular components for `init_global_context`, `init_workspace`, `parse_cargo_args`, `parse_global_args`, `create_compile_options`.
*   **Files**: `crates/rust-bootstrap/src/cargo_integration/mod.rs`, `crates/rust-bootstrap/src/cargo_integration/dispatch_cargo_command.rs`, `crates/rust-bootstrap/src/cargo_integration/parse_global_args.rs`, and new files/directories under `src/cargo_integration/`.

### Step 4.3: Enhance Command Execution Modularity
*   **Action**: Refactor `command_executor` to break down command execution into smaller, composable units (e.g., `capture_start_time`, `capture_end_time_and_duration`, `create_command_execution_result`).
*   **Files**: `crates/rust-bootstrap/src/bootstrap_stages/command_executor/mod.rs`, `crates/rust-bootstrap/src/bootstrap_stages/command_executor/execute_shell_command.rs`.

### Step 4.4: Update Main Application Logic
*   **Action**: Modify `main.rs` to utilize the new modular Cargo integration and command execution components.
*   **Files**: `crates/rust-bootstrap/src/main.rs`.

### Step 4.5: Update Dependencies
*   **Action**: Ensure `Cargo.toml` reflects the necessary dependencies for `clap`, vendored `cargo`, `tracing`, and `syscall_instrumentation_macro`.
*   **Files**: `Cargo.lock`, `crates/rust-bootstrap/Cargo.toml`.

## 5. Verification
*   Successful compilation of `rust-bootstrap`.
*   `rust-bootstrap` can execute `cargo build` and `cargo check` commands via its integrated logic.
*   Logs show `tracing` output for command execution.
*   Property-based tests for argument parsing pass.

## 6. Tools Used
*   `cargo` (for build, test)
*   `write_file`, `read_file`, `replace` (for file manipulation)
*   `run_shell_command` (for `cargo check`)

## 7. Challenges & Considerations
*   Ensuring complete replacement of external `cargo` calls with internal library calls.
*   Managing the complexity of vendored `cargo` and its dependencies.
*   Maintaining performance during the transition to monadic execution.
