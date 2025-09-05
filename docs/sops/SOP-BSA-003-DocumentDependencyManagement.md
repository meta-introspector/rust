# SOP-BSA-003: Document Dependency Management

## Purpose
To understand how `rustc` manages its internal and external dependencies, especially vendored ones.

## Procedure
1.  Analyze `Cargo.lock` to identify exact versions of key dependencies (e.g., `indexmap`, `measureme`).
2.  Examine `Cargo.toml` files of core `rustc` components (e.g., `rustc_data_structures`, `rustc_span`) to see how they declare dependencies.
3.  Investigate `bootstrap.py` and `metadata.rs` for any custom dependency resolution or vendoring logic.

## Tools
*   `read_file`: To read `Cargo.lock` and `Cargo.toml` files.
*   `search_file_content`: To find dependency declarations and usage patterns.

## Output
*   A detailed report on dependency resolution, including:
    *   Identified versions of key dependencies.
    *   Any inconsistencies or conflicts found.
    *   Description of vendoring mechanisms.
    *   Analysis of how `bootstrap` handles dependency resolution.

## Verification
*   Ensure consistency between `Cargo.toml` declarations and `Cargo.lock` resolutions.
*   Review by a "Build System Architect" agent.
