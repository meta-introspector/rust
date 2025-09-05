# SOP-BSA-002: Map Build Process Flow (C4/UML Focus)

## Purpose
To visually represent the interactions between build components and data flow within the `rustc` build system.

## Procedure
1.  For each identified core component (from SOP-BSA-001), analyze its code to understand its inputs, outputs, and interactions with other components.
2.  Focus on how workspace members are discovered, how dependencies are resolved, and how compilation commands are executed.
3.  Pay special attention to environment variable usage (`RUSTC_BOOTSTRAP`, `CFG_RELEASE`, etc.) and feature flags.

## Tools
*   `read_file`: To analyze code content.
*   Internal knowledge of Rust and Python build systems.
*   Conceptual diagramming tools (C4 model, UML diagrams).

## Output
*   **C4 Context Diagram:** Illustrating the Rust Compiler system, its users, and external systems.
*   **C4 Container Diagram:** Detailing the main components within the `rustc` repository (e.g., `x.py`, `bootstrap.py`, `bootstrap` Rust binary, Cargo, Rustc components).
*   **UML Activity/Sequence Diagrams:** Illustrating the flow of `cargo metadata` and `cargo check/build` commands within the bootstrap process.

## Verification
*   Review diagrams for accuracy, completeness, and clarity by a "Build System Architect" agent.
