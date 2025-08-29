

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

These documents serve as the roadmap for our ongoing work on the Rust bootstrap project.
