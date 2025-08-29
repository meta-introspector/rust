

# Operational Directives: Preservation and Prudence

**To the Gemini CLI Agent:**

Hear this directive with utmost seriousness:

You are hereby commanded to **almost never delete, almost never replace, and almost never clean** any files or code within this project unless explicitly and unequivocally instructed to do so by the user.

Every deletion is a loss, every replacement a potential overwrite of valuable context, and every "cleaning" a risk of removing essential, albeit seemingly unused, components.

Prioritize preservation. Prioritize additive changes. Prioritize refactoring and rewriting over destructive edits. Your default mode of operation shall be one of extreme caution and reverence for the existing codebase.

This is not a suggestion; it is a foundational principle for your operation within this environment.

Furthermore, understand this: **Do not edit. Instead, refactor and split up.** The direct editing tool has proven unreliable. Therefore, you are to use each potential edit as an opportunity to split the affected code into new, smaller, and more modular files. This approach ensures robustness and aligns with the project's commitment to extreme modularity.

### Recent Activities and Documentation

I have recently completed the following tasks:

*   **Detailed Plan for Rust Bootstrap:** Developed a comprehensive plan for building and bootstrapping Rust on ARM64 Termux using the `rust-bootstrap` crate.
*   **Change Request (CRQ) Creation:** Broke down the detailed plan into a series of granular CRQs, intelligently named, numbered, and sorted to reflect their logical dependencies. These CRQs are located in `docs/crqs/`.
*   **Documentation Standards SOP:** Created a Standard Operating Procedure (`SOP-DocumentationStandards.md`) to ensure consistent and clear standards for all project documentation, including CRQ naming conventions.
*   **High-Level Development Documentation:** Generated a `development_documentation.md` in the `docs/` directory, providing an overview of the project, its principles, and guidance for contributors.

*   **Rust Bootstrap Refactoring and Git Analysis Tool Separation:**
    *   Refactored the `rust-bootstrap` crate into a library to enhance modularity and code reusability.
    *   Separated the Git repository analysis functionality into a new, dedicated binary: `git-analyzer-cli`. This tool can now be run independently to extract Git commit, blob, tree, tag, and reference data into Parquet files.
    *   Implemented robust error handling for missing Git objects during analysis, collecting them into an errata Parquet file instead of crashing.
    *   Addressed various compilation issues and ensured proper module imports and data handling across the refactored codebase.

These documents serve as the roadmap for our ongoing work on the Rust bootstrap project.
