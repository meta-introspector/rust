# Gemini CLI Prompt Template

This template is designed to help you construct effective prompts for the Gemini CLI agent. Fill in the sections below to provide clear instructions and context for your task.

---

## 1. Task Description

*   **What do you want Gemini to do?** (Be clear, concise, and specific. What is the main goal? If the task implies a code change, consider using conventional commit prefixes like `feat:`, `fix:`, `docs:`, `refactor:`.)
    *   *Example: "Refactor the `calculate_area` function in `src/geometry.rs` to improve performance."*
    *   *Example: "Generate a new Rust module for handling user authentication, including login and registration functions."*

## 2. Context and Background

*   **What information does Gemini need to understand the task?** (Provide relevant file paths, code snippets, project structure, existing conventions, or any other necessary background.)
    *   *Example: "The `calculate_area` function is located at `/data/data/com.termux/files/home/storage/github/rust/src/geometry.rs`."*
    *   *Example: "The project uses `tokio` for async operations and `serde` for serialization/deserialization."*
    *   *Example: "Existing authentication logic is in `src/auth_legacy.rs` (for reference, but do not modify)."*
    *   *If the task relates to a specific change or issue, provide relevant Git commit hashes or diffs (e.g., `git show <commit_hash>`).*
    *   *You can use `read_file`, `glob`, or `search_file_content` to provide this context.* 

## 3. Desired Output

*   **How should Gemini present its response?** (Specify format, level of detail, and any specific elements you need.)
    *   *Example: "Provide the refactored code as a Rust code block."*
    *   *Example: "Outline a step-by-step plan before making any changes."*
    *   *Example: "Summarize the changes in bullet points."*
    *   *Example: "Generate a new file at `src/auth/mod.rs` with the new module."*

## 4. Constraints and Guidelines

*   **Are there any limitations, forbidden actions, or preferred approaches?** (e.g., coding style, specific libraries to use/avoid, performance targets, error handling requirements.)
    *   *Example: "Do not introduce any new external dependencies."*
    *   *Example: "Adhere strictly to the existing Rust coding style (e.g., `rustfmt` conventions)."*
    *   *Example: "Ensure all functions return `Result` for error handling."*
    *   *Example: "Prioritize readability over extreme conciseness."*
    *   *Example: "Do not modify any files outside of the `src/` directory."*
    *   *Adhere to project-specific architectural principles, such as "extreme modularity" (e.g., one function per file) and consistent `PathBuf` usage.*

## 5. Verification (Optional)

*   **How can Gemini verify its work?** (If applicable, specify commands to run tests, linters, or build processes.)
    *   *Example: "After making changes, run `cargo test` to ensure no regressions."*
    *   *Example: "Run `cargo clippy --fix` and `cargo fmt` to ensure code quality."*

---

**Example Usage:**

```
Hey Gemini, I need help with a Rust refactoring task.

# 1. Task Description
Refactor the `process_data` function in `src/data_processor.rs` to use an iterator-based approach instead of explicit loops for better performance and Rust idiomatic style.

# 2. Context and Background
The file is located at `/data/data/com.termux/files/home/storage/github/rust/src/data_processor.rs`.
The `process_data` function currently looks like this:
```rust
// src/data_processor.rs
pub fn process_data(data: &mut Vec<i32>) {
    for i in 0..data.len() {
        data[i] = data[i] * 2 + 1;
    }
}
```
The project uses Rust 2021 edition.

# 3. Desired Output
Provide the complete refactored `src/data_processor.rs` file content as a Rust code block.

# 4. Constraints and Guidelines
- Do not change the function signature.
- Ensure the refactored code is idiomatic Rust.
- No new dependencies.

# 5. Verification
Run `cargo test` after the changes.
```