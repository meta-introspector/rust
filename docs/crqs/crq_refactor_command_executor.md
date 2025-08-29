# CRQ: Refactor Duplicate Command Executor Code in rust-bootstrap

## Objective
Eliminate duplicate code within `rust-bootstrap` by refactoring the `command_executor` logic found in `src/bootstrap_stages/command_executor` and `src/main_stages/command_executor` into a single, reusable module.

## Scope
*   Identify the common functionalities and patterns in both `command_executor` implementations.
*   Design and implement a new, shared module (e.g., `crates/rust-bootstrap/src/utils/command_executor.rs`) that encapsulates the common command execution logic.
*   Replace the duplicated code in `src/bootstrap_stages/command_executor` and `src/main_stages/command_executor` with calls to the new shared module.
*   Ensure that the refactoring does not introduce any regressions or change the existing behavior of command execution.
*   Update any relevant internal documentation or comments to reflect the new module structure.

## Impact
Improves the maintainability, readability, and efficiency of the `rust-bootstrap` codebase by reducing redundancy. This refactoring is a prerequisite for further development and migration efforts, ensuring a cleaner and more robust foundation.

## Verification Steps
*   The `rust-bootstrap` crate compiles successfully after the refactoring.
*   All existing tests related to command execution (if any) pass without failures.
*   Manual testing confirms that `rust-bootstrap` still correctly executes commands and captures metrics as before the refactoring.
*   Code review confirms the successful removal of duplicate code and the proper integration of the new shared module.

## Dependencies
*   Prior knowledge of the `rust-bootstrap` codebase structure.
*   Understanding of Rust's module system and best practices for code organization.
