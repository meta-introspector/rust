# Standard Operating Procedure (SOP): Gemini's Extreme Modularity Refactoring

## 1. Objective
This SOP establishes a mandatory procedure for refactoring Rust code, specifically designed to circumvent limitations of direct in-place editing tools (e.g., `replace` tool failures on complex string matching). It enforces an "extreme modularity" approach, where every logical modification or refactoring opportunity is leveraged to split code into new, smaller, single-purpose files.

## 2. Principle
**"Don't edit, refactor and split up. Use each potential edit to split into new files."**

## 3. Scope
This procedure applies to all code modifications within Rust crates where direct in-place editing of existing functions or code blocks is intended. It promotes a granular, additive approach to code evolution.

## 4. Procedure

### 4.1. Identify Target for Modification
Instead of identifying a specific line or block of code to *edit*, identify the smallest logical unit (typically a function, or a distinct logical block within a function) that *would have been the subject of the modification*. This unit will be extracted.

### 4.2. Create New File for Extracted Unit
Create a new `.rs` file for the identified logical unit. The file name should clearly reflect the new unit's single responsibility (e.g., `process_input.rs`, `calculate_checksum.rs`).

### 4.3. Move Code to New File
Cut the entire code of the identified logical unit (e.g., the function definition, or the extracted block wrapped in a new function) from its original location and paste it into the newly created `.rs` file.

### 4.4. Apply Desired Changes in New File
Make all the intended modifications, refactorings, or new implementations directly within this *newly created file*. This new file is now the primary location for the logic.

### 4.5. Update Original Call Site
In the original file where the code was extracted from:
*   **Replace the extracted code** with a call to the new function/module.
*   Ensure all necessary arguments are passed to the new function and return values are handled correctly.
*   This step might still require the `replace` tool. If `replace` fails here (e.g., on a complex function call signature), consider applying this SOP recursively: extract the *calling* function into a new file and modify it there.

### 4.6. Update Parent Module (`mod.rs`)
Declare the new module in the parent `mod.rs` file using `pub mod <module_name>;`. Ensure correct module visibility and hierarchy.

### 4.7. Manage Imports
*   Add necessary `use` statements in the new file for any dependencies.
*   Remove any `use` statements from the original file that are no longer needed after the code extraction.

### 4.8. Compile and Verify
*   **Build Frequently:** After each significant refactoring step (e.g., after extracting a function and updating its call site), run `cargo build -p <crate_name>` to catch compilation errors early.
*   **Address Errors:** Resolve any compilation errors, paying close attention to module paths, visibility, and type mismatches.
*   **Test:** Run relevant tests to ensure the refactoring has not introduced regressions.

## 5. Tool Usage
*   `write_file`: Primarily used to create new `.rs` files with the extracted or modified code.
*   `replace`: Used sparingly for minimal, highly targeted changes in existing files, such as updating `use` statements or replacing a block of code with a function call. If `replace` consistently fails for a given modification, it is a strong signal to apply this SOP and split the code further.
*   `run_shell_command`: For `cargo build` and `cargo test` commands.

## 6. Benefits
*   **Circumvents Tool Limitations:** Provides a robust workflow when direct in-place editing tools are unreliable.
*   **Enforces Modularity:** Naturally leads to highly modular, single-responsibility functions and files.
*   **Improves Readability:** Smaller files and functions are easier to understand and maintain.
*   **Enhances Testability:** Isolated functions are simpler to unit test.
*   **Facilitates Collaboration:** Reduces merge conflicts by minimizing changes to shared files.
