# Change Request (CRQ): Rust Bootstrap - Initial Setup and Project Structure

## 1. Objective
To establish the foundational project structure for `rust-bootstrap`, including basic argument parsing, configuration loading, and initial setup for detecting the `stage0` Rust compiler.

## 2. Scope
This CRQ covers:
*   Creation of the `rust-bootstrap` crate.
*   Implementation of command-line argument parsing using `clap`.
*   Loading and parsing of `bootstrap.toml` configuration.
*   Initial detection of the `stage0` Rust compiler.
*   Basic project structure and module organization.

## 3. Prerequisites
*   A Rust development environment with `cargo` installed.
*   Familiarity with `x.py`'s basic operation and configuration.

## 4. Procedure

### Step 4.1: Create `rust-bootstrap` Crate.
*   **Action:** Initialize a new Rust binary crate named `rust-bootstrap` within the `crates/` directory of the main Rust project. (Refer to `rust_bootstrap_development_sop.md` section 4.1).
*   **Tool:** `run_shell_command` (`cargo new --bin rust-bootstrap`)

### Step 4.2: Implement Argument Parsing.
*   **Action:** Define command-line arguments for `rust-bootstrap` using the `clap` crate. Include arguments for configuration file path, build directory, and verbosity. (Refer to `rust_bootstrap_development_sop.md` section 4.2).
*   **Tool:** `write_file`, `replace`

### Step 4.3: Implement Configuration Loading.
*   **Action:** Develop logic to load and parse the `bootstrap.toml` file. This involves using a TOML parsing library (e.g., `toml`) and `serde` for deserialization into Rust structs. (Refer to `rust_bootstrap_development_sop.md` section 4.3).
*   **Tool:** `write_file`, `replace`

### Step 4.4: Implement Stage0 Detection.
*   **Action:** Implement a mechanism to detect the `stage0` Rust compiler. This might involve checking environment variables, well-known paths, or running `rustc --version --verbose`. (Refer to `rust_bootstrap_development_sop.md` section 4.4).
*   **Tool:** `write_file`, `replace`, `run_shell_command`

### Step 4.5: Establish Basic Project Structure.
*   **Action:** Organize the code into logical modules (e.g., `config`, `build_state`, `stages`) to ensure maintainability and scalability. (Refer to `rust_bootstrap_development_sop.md` section 4.5).
*   **Tool:** `write_file`, `run_shell_command` (`mkdir`)

## 5. Verification
*   **Step 5.1:** Run `rust-bootstrap --help` and verify that the argument parsing works correctly.
*   **Step 5.2:** Create a dummy `bootstrap.toml` and verify that `rust-bootstrap` can load and parse it without errors.
*   **Step 5.3:** Verify that `rust-bootstrap` correctly identifies the `stage0` compiler (or reports its absence).
*   **Step 5.4:** Ensure the project compiles without warnings or errors after initial setup.

## 6. Tools Used
*   `run_shell_command`: For `cargo new` and other shell commands.
*   `write_file`: For creating new Rust source files.
*   `replace`: For modifying existing Rust source files.

## Progress Update (2025-08-29)
This CRQ is largely complete. Argument parsing, configuration loading, and initial `stage0` detection are implemented and functional. The basic project structure is established. Further work on `x.py` porting will build upon this foundation.