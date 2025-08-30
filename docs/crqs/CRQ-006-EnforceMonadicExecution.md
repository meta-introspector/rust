# Change Request (CRQ): Enforce Monadic Execution and Disallow Direct Shell Commands

## 1. Objective
To enforce the use of the new monadic execution model within `rust-bootstrap` by disallowing direct execution of shell commands, thereby ensuring all external interactions are routed through controlled, integrated library calls. This is a critical step towards formal verification and a self-contained bootstrap.

## 2. Scope
This CRQ covers:
*   Introduction of an `exec_panic` mechanism in `execute_shell_command` to prevent direct shell command execution.
*   Configuration of this mechanism via command-line arguments in `rust-bootstrap`.
*   Ensuring all necessary external commands (e.g., `cargo`) are now handled by integrated library calls.

## 3. Prerequisites
*   Successful implementation of direct Cargo integration (CRQ-005).
*   Understanding of the `command_executor` module and its role in the new execution model.

## 4. Procedure (High-Level)
### Step 4.1: Implement `exec_panic` in `execute_shell_command`
*   **Action**: Add a boolean flag `exec_panic` to `execute_shell_command` that, when true, causes the function to panic instead of executing the command.
*   **Files**: `crates/rust-bootstrap/src/bootstrap_stages/command_executor/execute_shell_command.rs`.

### Step 4.2: Expose `exec_panic` via `Args`
*   **Action**: Add an `exec_panic` argument to the main `Args` struct, defaulting to `true` to enforce the new behavior.
*   **Files**: `crates/rust-bootstrap/src/config/args.rs`.

### Step 4.3: Integrate `exec_panic` into `main.rs`
*   **Action**: Pass the `exec_panic` value from `Args` to the relevant command execution functions.
*   **Files**: `crates/rust-bootstrap/src/main.rs`.

## 5. Verification
*   Running `rust-bootstrap` with `exec_panic` enabled (default) and attempting to execute a direct shell command (if any remain) results in a panic.
*   All essential `rust-bootstrap` functionalities (e.g., building, checking) continue to work correctly, demonstrating that direct shell commands have been successfully replaced by integrated library calls.

## 6. Tools Used
*   `cargo` (for build, test)
*   `write_file`, `read_file`, `replace` (for file manipulation)

## 7. Challenges & Considerations
*   Identifying and replacing all instances of direct shell command execution.
*   Ensuring no critical functionality is inadvertently broken by disallowing shell commands.
*   Providing clear error messages when a panic occurs due to `exec_panic`.
