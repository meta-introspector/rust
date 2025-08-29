# Development Documentation: Rust Bootstrap Project

This document provides a high-level overview of the Rust Bootstrap project, its goals, key architectural components, development principles, and guidance for contributors.

## 1. Project Goal

The overarching goal of this project is to achieve a Rust bootstrap that builds and bootstraps Rust on an ARM64 Termux environment using the `rust-bootstrap` crate. This aims to replace the existing Python-based `x.py` bootstrap with a robust, formally verifiable, and deeply bootstrapped Rust compiler.

## 2. Key Architectural Components

The `rust-bootstrap` project is designed with extreme modularity and data-oriented principles. Key components include:

*   **`rust-bootstrap` Crate:** The core Rust application responsible for orchestrating the build process, managing stages, and interacting with the environment.
*   **Git Analysis Module:** Extracts Git repository data and converts it into Arrow/Parquet datasets for detailed analysis and formal verification.
*   **Parquet Reporter:** Handles the serialization of Arrow `RecordBatch`es into Parquet files, enabling efficient storage and retrieval of intermediate data states.
*   **Stage0 Detection:** Logic to identify and utilize the initial Rust compiler (`stage0`) available in the environment.
*   **Build Orchestration:** Manages the execution of `cargo` commands across different build stages, including handling cross-compilation for ARM64 Termux.

## 3. Development Principles

Our development philosophy emphasizes clarity, maintainability, and a structured approach:

*   **Extreme Modularity:** Code is broken down into the smallest logical units, with each function or basic block residing in its own dedicated file. This enhances readability, testability, and facilitates granular changes. (Refer to `SOP-ExtremeModularityRefactoring.md` and `SOP-GeminiExtremeModularityRefactoring.md`)
*   **Additive Changes (No Direct Edits):** We prioritize refactoring and splitting code into new files over direct in-place editing. This circumvents limitations of direct editing tools and promotes a cleaner, more auditable codebase. (Refer to `GEMINI.md` for the core directive: "Do not edit. Instead, refactor and split up.")
*   **Data Flow as Arrow/Parquet:** Intermediate data states and transformations are represented as Arrow/Parquet datasets, enabling formal verification, reproducibility, and efficient debugging. (Refer to `SOP-DataFlowAsArrowParquet.md`)
*   **CRQ-Driven Development:** All major features and development phases are managed through Change Request (CRQ) documents, providing a clear plan, scope, and verification steps. (Refer to `SOP-DocumentationStandards.md` for CRQ structure and naming).

## 4. Getting Started

To begin contributing or setting up your development environment, follow the steps outlined in the initial CRQs:

*   **CRQ-001-BootstrapInitialSetup.md:** Guides through verifying your Termux ARM64 environment and setting up the basic `rust-bootstrap` crate.
*   **CRQ-002-BootstrapStage0GitAnalysis.md:** Details the implementation of `stage0` detection and Git analysis integration.

Continue through the numbered CRQs in the `docs/crqs/` directory for a guided development path.

## 5. How to Contribute

We welcome contributions! To ensure consistency and quality, please adhere to the following guidelines:

*   **Follow Documentation Standards:** All new and updated documentation should conform to the standards outlined in `SOP-DocumentationStandards.md`.
*   **Adhere to Modularity Principles:** When making code changes, prioritize refactoring and splitting over direct edits, as detailed in the extreme modularity SOPs.
*   **Code Quality:** Ensure your code is formatted with `cargo fmt` and passes `cargo clippy` checks without warnings. (Refer to `CRQ-007-BootstrapCodeQualityTooling.md`)
*   **Testing:** Develop unit tests for new features and ensure existing tests pass.

For more detailed information on specific procedures, refer to the relevant SOPs in the `docs/sops/` directory.
