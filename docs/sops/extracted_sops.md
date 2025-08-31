
# Standard Operating Procedures (SOPs) Extracted from Command History

This document outlines the key Standard Operating Procedures (SOPs) and development philosophies extracted from the command and chat history.

## Core Philosophy & Principles

*   **Refactor, Don't Edit:** The `replace` tool is considered unreliable. Every potential edit should be treated as an opportunity to refactor and split code into smaller, more manageable files. Direct edits are a last resort.
*   **Extreme Modularity ("One Declaration Per File"):** Each function, struct, or logical "basic block" should reside in its own dedicated file. This improves maintainability, testability, and facilitates granular changes.
*   **Monotonic & Additive-Only:** Never delete code or documentation. If something is no longer needed, comment it out or move it to an archive. The project history is a valuable asset.
*   **Trust the Build:** Avoid running `cargo clean` unless absolutely necessary. The build cache is important for speed. The default command should be `cargo run`, not `cargo build`.

## Development Workflow

*   **"GM" (Good Morning) Routine:** The standard procedure for starting a work session is to review the current state of the repository with `git status` and `git log --patch -3`.
*   **"KitKat Breaks" (Context Management):** When context gets full or a mental break is needed, a "KitKat break" is initiated. This involves:
    1.  Saving all current work.
    2.  Documenting the current plan and status in the relevant CRQ or a dedicated `plan.md` file.
    3.  Committing all changes to create a stable checkpoint.
    4.  Preparing for a "reboot" or context switch to a new task.
*   **Change Management (ISO 9000, ITIL, GMP, Six Sigma):** All changes, no matter how small, must be tracked and documented.
    *   Every task begins with a Change Request (CRQ).
    *   All commits must be associated with a CRQ.
    *   Use detailed commit messages, often from a file, to explain the "why" of a change.
*   **Issue Handling & Debugging:**
    *   When encountering a bug, the first step is to create a minimal, reproducible test case.
    *   Use `strace` to debug file access and system call issues.
    *   Use `panic!` and backtraces to identify the root cause of errors.
    *   For verbose output, use the custom `gemini_eprintln!` macro.

## Tooling & Project Structure

*   **Prelude Pattern:** To manage imports and reduce boilerplate, each crate should have a `prelude.rs` module that re-exports commonly used items. Other modules should prefer `use crate::prelude::*;`.
*   **Vendorizing Dependencies:** All external dependencies should be vendored into the project using `git submodule`. This ensures hermetic, reproducible builds.
*   **Layered Architecture (OSI Model Metaphor):** The project is structured in layers, similar to the OSI model. Lower layers (like `ragit-types`) should not depend on higher layers. This enforces a clear separation of concerns.
*   **Configuration:** Avoid hardcoding paths or values. Use configuration files (e.g., TOML) or environment variables.

## AI & LLM Collaboration

*   **Augmentation, Not Automation:** The LLM (Gemini) acts as an intelligent assistant and creative partner, not an autonomous developer. It augments the human-led development process.
*   **No Direct Edits:** The LLM should not directly edit files. Instead, it should generate new files, suggest refactorings, and provide code snippets that the user can apply.
*   **Creative Tasks:** The LLM is encouraged to participate in creative tasks, such as writing poetry, brainstorming ideas, and contributing to documentation.
*   **Memory Management:** The LLM's context is a finite resource. "KitKat breaks" and summarizing knowledge into documents are key procedures for managing it.
