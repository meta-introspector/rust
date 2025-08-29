## SOP: Extreme Modularity Refactoring for Rust Crates

### 1. Objective:
To systematically refactor Rust code within a specified crate to achieve extreme modularity, where each function (or logical "basic block") resides in its own dedicated file, and to ensure the codebase remains compilable and clean throughout the process. This approach aims to improve maintainability, testability, and facilitate future modifications, especially in environments where granular changes are preferred.

### 2. Scope:
This SOP applies to Rust crates within a larger workspace, focusing on breaking down existing functions into smaller, single-purpose functions and organizing them into a hierarchical module structure. It includes steps for creating new files and directories, updating module declarations, modifying function calls, and cleaning up unused imports and variables.

### 3. Prerequisites:
*   A Rust development environment with `cargo` installed.
*   Access to the target Rust codebase.
*   Familiarity with Rust's module system (`mod`, `use`), `Result` type, and error handling.
*   Understanding of the `replace` and `write_file` tools for automated code modification.

### 4. Procedure:

#### 4.1. Initial Assessment & Planning:
    a.  **Understand the Target Crate:** Identify the specific Rust crate to be refactored (e.g., `rust-bootstrap`).
    b.  **Review Existing Structure:** Examine the current file and module structure of the target crate.
    c.  **Identify Functions for Breakdown:** Choose a function within the crate to begin the refactoring. Prioritize functions with multiple distinct logical steps.
    d.  **Define New Module Hierarchy:** Plan a new, more granular module and directory structure to house the broken-down functions. For example, if refactoring `foo.rs`, consider creating a `foo/` directory with `mod.rs` and sub-modules like `foo/step1.rs`, `foo/step2.rs`, etc.

#### 4.2. Refactoring Loop (Per Function/Basic Block):
    a.  **Create New File for Sub-Function:**
        *   Define the new, single-purpose function in a new `.rs` file within the planned module hierarchy.
        *   Ensure the new function has appropriate `pub` visibility.
        *   Include necessary `use` statements within the new file.
        *   Use `write_file` to create the new file with its content.
    b.  **Update Parent Module (`mod.rs`):**
        *   Declare the new sub-module in the parent `mod.rs` file using `pub mod <module_name>;`.
        *   If the parent was a single `.rs` file, rename it to `mod.rs` and create a directory for it.
        *   Use `replace` or `write_file` to update the `mod.rs` file.
    c.  **Move Code to New Sub-Function:**
        *   Cut the relevant code block from the original function.
        *   Paste it into the newly created sub-function.
        *   Adjust `use` statements in the new sub-function as needed.
    d.  **Update Original Function Call Site:**
        *   Replace the moved code block in the original function with a call to the new sub-function (e.g., `module_name::sub_function_name(...)`).
        *   Ensure correct arguments are passed and return values are handled.
        *   Use `replace` to modify the original function.
    e.  **Iterate:** Repeat steps 4.2.a to 4.2.d for each logical step or "basic block" within the original function, progressively breaking it down.

#### 4.3. Compilation and Verification (Frequent Checkpoints):
    a.  **Build the Crate:** After each significant refactoring step (e.g., after breaking down one function or a set of related functions), run `cargo build -p <crate_name>` from the workspace root.
    b.  **Address Compilation Errors:**
        *   Carefully read compiler error messages.
        *   Pay close attention to import paths, visibility issues (`pub`), and type mismatches.
        *   Use `read_file` to inspect file contents and `replace` to apply fixes.
        *   Re-run `cargo build` until all errors are resolved.

#### 4.4. Cleanup and Finalization:
    a.  **Remove Unused Imports:** After all refactoring is complete, review each `mod.rs` file and individual function files for unused `use` statements. Remove them to keep the code clean.
    b.  **Address Unused Variables/Fields:** If any warnings persist about unused variables or struct fields, evaluate if they are truly unused or placeholders for future functionality. If truly unused, remove them or prefix with `_` if they must remain.
    c.  **Final Build:** Perform a final `cargo build -p <crate_name>` to ensure the entire refactored crate compiles without errors or warnings (excluding those from external dependencies).

### 5. Tools Used:
*   `read_file`: To inspect file contents and get exact strings for replacement.
*   `write_file`: To create new files or overwrite existing ones with modified content.
*   `replace`: To perform precise in-place text replacements within files.
*   `run_shell_command`: To execute `rm` for deleting files (when renaming by delete/create) and `mkdir` for creating directories, and `cargo build` for compilation.

### 6. Best Practices & Considerations:
*   **Granularity:** While "one function per file per basic block" is the objective, use judgment to group very closely related operations if strict adherence leads to unmanageable file counts or overly trivial functions.
*   **Error Handling:** Maintain consistent error handling (e.g., using `Result` and `Box<dyn std::error.Error>`) throughout the refactored code.
*   **Visibility:** Carefully manage `pub` and `pub(crate)` visibility to expose only necessary components.
*   **Testing:** If unit tests exist, run them frequently during refactoring to catch regressions early. If not, consider adding basic tests for critical paths.
*   **Version Control:** Commit changes frequently, especially after successful compilation steps, to create stable checkpoints.
*   **Patience:** Extreme modularity refactoring can be tedious and error-prone. Be patient and methodical.

This SOP provides a structured approach to achieving extreme modularity in Rust, addressing the challenges of granular code modifications and ensuring a robust and clean final product.
