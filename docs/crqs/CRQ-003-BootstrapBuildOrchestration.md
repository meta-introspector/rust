# Change Request (CRQ): Rust Bootstrap - Build Orchestration and Bootstrapping

## 1. Objective
To implement the core build orchestration logic within `rust-bootstrap`, enabling it to parse `x.py` commands, execute `cargo` commands for multi-stage builds, and handle cross-compilation and the full bootstrapping process for the Rust compiler on ARM64 Termux.

## 2. Scope
This CRQ covers:
*   Implementation of `x.py` command parsing and translation.
*   Orchestration of `cargo` commands for different build stages.
*   Support for cross-compilation to the ARM64 Termux target.
*   Implementation of the multi-stage bootstrapping process.

## 3. Prerequisites
*   Completion of CRQ: `rust_bootstrap_stage0_and_git_analysis.md`.
*   A functional `stage0` Rust compiler detected by `rust-bootstrap`.

## 4. Procedure

### Step 3.1: Implement `x.py` Command Parsing.
*   **Action:** Analyze the `src/bootstrap/bootstrap.py` script to understand its command-line interface and internal command structure. Implement logic in `rust-bootstrap` to parse these `x.py` commands and translate them into corresponding Rust-native build steps or internal function calls. This will involve mapping `x.py` subcommands (e.g., `build`, `test`, `dist`) to `rust-bootstrap`'s internal logic. (Refer to `rust_bootstrap_development_sop.md` section 4.4.a).
*   **Tool:** `read_file`, `write_file`, `replace`

### Step 3.2: Orchestrate `cargo` Commands for Build Stages.
*   **Action:** Implement the core build orchestration logic in `rust-bootstrap`. This involves programmatically executing `cargo` commands (e.g., `cargo build`, `cargo install`) for different stages of the Rust compiler build process (e.g., `stage1`, `stage2`). Ensure proper management of build artifacts, output directories, and intermediate compilation results. (Refer to `rust_bootstrap_development_sop.md` section 4.4.b, 4.4.c).
    *   **Sub-task:** Implement Cargo Command Execution (Refer to `CRQ-003-1-ImplementCargoCommandExecution.md`).
*   **Tool:** `write_file`, `replace`, `run_shell_command`

### Step 3.3: Implement Cross-Compilation Support.
*   **Action:** Address the specific requirements for cross-compiling the Rust compiler for the ARM64 Termux environment. This may involve setting appropriate environment variables like `RUSTFLAGS`, `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS`, and ensuring correct target specifications are used during `cargo` invocations. (Refer to `rust_bootstrap_development_sop.md` section 4.4.c, implicitly).
*   **Tool:** `write_file`, `replace`, `run_shell_command`

### Step 3.4: Implement Multi-Stage Bootstrapping.
*   **Action:** Integrate the `stage0` compiler detection (from CRQ 2) with the build orchestration logic to perform the full multi-stage bootstrap of the Rust compiler. This means `rust-bootstrap` will use the `stage0` compiler to build `stage1`, and then `stage1` to build `stage2`, and so on, until a self-hosting compiler is produced for the ARM64 Termux environment.
*   **Tool:** `write_file`, `replace`, `run_shell_command`

## 5. Verification
*   **Step 5.1:** Run `rust-bootstrap` with a command equivalent to a basic `x.py build` and verify that it successfully compiles a simple Rust project through multiple stages.
*   **Step 5.2:** Verify that `rust-bootstrap` can successfully cross-compile a basic Rust program for the ARM64 Termux target.
*   **Step 5.3:** Execute the full multi-stage bootstrap process using `rust-bootstrap`. Verify that a self-hosting Rust compiler is produced for the ARM64 Termux environment.

## 6. Tools Used
*   `run_shell_command`: For executing `cargo` commands and verifying build outputs.
*   `write_file`: For creating and modifying Rust source files.
*   `replace`: For in-place modifications of existing code.
*   `read_file`: For analyzing `x.py` content.

## Progress Update (2025-08-29)
Significant progress has been made on implementing the core build orchestration logic. The `build_bootstrap` function in `src/bootstrap_stages/build_bootstrap/mod.rs` has been implemented to:
*   Set `CARGO_TARGET_DIR` and `RUSTC` environment variables.
*   Set library path environment variables (`LD_LIBRARY_PATH`, etc.).
*   Set `RUSTC_BOOTSTRAP`.
*   Handle `RUSTFLAGS` (including `-Zallow-features=`, `-Wrust_2018_idioms`, `-Wunused_lifetimes`, and `-Dwarnings` based on config).
*   Add `--verbose` arguments.
*   Add `--locked`, `--frozen`, `--features build-metrics`, `--message-format=json`, and `--color` arguments based on config and args.
*   Handle `BOOTSTRAP_TRACING` and `CARGOFLAGS` environment variables.

This covers a substantial part of Step 3.2 and lays the groundwork for Step 3.3. The `execute_and_report_command` utility has also been implemented and integrated.