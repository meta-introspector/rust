# SOP-NCI-001: Propose Integration Method

## Purpose
To define the most effective and maintainable way to add new crates to the `rustc` build system.

## Procedure
1.  Based on the analysis from Phase 1 (CRQ-20250905-001), propose a method for integrating new Rust crates into the `rustc` workspace.
2.  Justify the chosen method based on criteria such as:
    *   Minimal invasiveness to core `rustc` build logic.
    *   Maintainability and alignment with existing `rustc` development patterns.
    *   Leveraging existing `bootstrap` mechanisms (e.g., `cargo metadata` integration, `Step` execution).
    *   Scalability for integrating multiple crates.

## Tools
*   Analysis reports and diagrams generated in Phase 1.
*   Knowledge of Rust build systems and best practices.

## Output
*   A detailed proposal document outlining the chosen integration method, including any necessary modifications to `bootstrap.py`, `src/bootstrap/Cargo.toml`, `src/bootstrap/src/core/metadata.rs`, `src/bootstrap/src/core/builder/mod.rs`, or other relevant files.

## Verification
*   Review and approval by the "Build System Architect" agent.
