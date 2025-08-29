# Change Request (CRQ): Rust Bootstrap - Initial Setup and Foundation

## 1. Objective
To establish the foundational elements of the `rust-bootstrap` crate, ensuring it can compile, handle basic configuration, and execute external commands within the Termux ARM64 environment. This CRQ covers the initial setup and verification of the core `rust-bootstrap` functionality.

## 2. Scope
This CRQ covers:
*   Verification of the Termux ARM64 environment.
*   Initial setup and compilation of the `rust-bootstrap` crate.
*   Implementation of configuration loading from `config.toml`.
*   Implementation of environment variable handling and external command execution.

## 3. Prerequisites
*   Access to the `rust` repository.
*   A Termux environment on an ARM64 device.

## 4. Procedure

### Step 1.1: Verify Termux ARM64 Environment.
*   **Action:** Run `uname -a` and `rustc -V` (if installed) to confirm ARM64 architecture and existing Rust installation.
*   **Tool:** `run_shell_command`

### Step 1.2: Initial `rust-bootstrap` Crate Setup.
*   **Action:** Ensure `crates/rust-bootstrap` exists with `Cargo.toml` and `src/main.rs`. Add `clap`, `serde`, `toml` dependencies to `Cargo.toml`. Implement basic "Hello World" functionality in `src/main.rs` and initial argument parsing using `clap`. (Refer to `rust_bootstrap_development_sop.md` section 4.1 for details).
*   **Tool:** `write_file`, `replace`, `run_shell_command`

### Step 1.3: Implement Configuration Loading.
*   **Action:** Implement `config.toml` parsing using `serde` and `toml` within `rust-bootstrap`, including `BuildConfig` and `Config` structs. This involves creating the necessary Rust structs and logic to deserialize the TOML file. (Refer to `rust_bootstrap_development_sop.md` section 4.1.e).
*   **Tool:** `write_file`, `replace`, `read_file`

### Step 1.4: Implement Environment Variable Handling and External Command Execution.
*   **Action:** Add logic for handling `RUSTC`, `CARGO` environment variables with Termux-specific defaults. Implement functionality for executing external commands using `std::process::Command` and capturing their output. (Refer to `rust_bootstrap_development_sop.md` section 4.1.f, 4.1.g).
*   **Tool:** `write_file`, `replace`, `run_shell_command`

## 5. Verification
*   **Step 5.1:** Confirm `uname -a` output shows `aarch64`.
*   **Step 5.2:** `cargo build -p rust-bootstrap` completes successfully without errors.
*   **Step 5.3:** `rust-bootstrap` can successfully load a sample `config.toml` and print its contents (e.g., via debug print).
*   **Step 5.4:** `rust-bootstrap` can execute a simple external command (e.g., `echo "Hello from rust-bootstrap"`) and print its output, and correctly resolve `RUSTC` and `CARGO` paths based on environment variables or Termux defaults.

## 6. Tools Used
*   `run_shell_command`: For executing shell commands and `cargo` commands.
*   `write_file`: For creating and modifying Rust source files and `Cargo.toml`.
*   `replace`: For in-place modifications of existing code.
*   `read_file`: For inspecting file contents (e.g., `config.toml`).
