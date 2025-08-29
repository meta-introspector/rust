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

## 6. Current Progress and Next Steps

### 6.1. Current Progress

Significant progress has been made in porting the `x.py` bootstrap process to Rust. The `rust-bootstrap` crate now handles:

*   **Argument Parsing and Configuration Loading:** Basic command-line arguments are parsed, and `bootstrap.toml` configuration is loaded.
*   **Stage0 Detection:** The `stage0` Rust compiler is detected.
*   **Command Execution Utility:** A robust `command_executor` module has been implemented to execute shell commands and report metrics, replacing duplicated logic.
*   **Bootstrap Compiler Build Orchestration:** The `build_bootstrap` function in `src/bootstrap_stages/build_bootstrap/mod.rs` is now responsible for orchestrating the `cargo build` command for the bootstrap compiler. This includes:
    *   Setting `CARGO_TARGET_DIR` and `RUSTC` environment variables.
    *   Setting library path environment variables (`LD_LIBRARY_PATH`, `DYLD_LIBRARY_PATH`, `LIBRARY_PATH`, `LIBPATH`).
    *   Setting `RUSTC_BOOTSTRAP`.
    *   Handling `RUSTFLAGS` (including `-Zallow-features=`, `-Wrust_2018_idioms`, `-Wunused_lifetimes`, and `-Dwarnings` based on configuration).
    *   Adding `--verbose` arguments.
    *   Adding `--locked`, `--frozen`, `--features build-metrics`, `--message-format=json`, and `--color` arguments based on configuration and command-line arguments.
    *   Handling `BOOTSTRAP_TRACING` and `CARGOFLAGS` environment variables.
*   **Bootstrap Binary Path Resolution:** The `bootstrap_binary` method in `src/builder.rs` provides the expected path to the built bootstrap executable.
*   **Main Application Flow:** `src/main.rs` now orchestrates the build of the bootstrap compiler and then attempts to execute the newly built binary, passing along the original command-line arguments.

### 6.2. Next Steps: Implementing the Multi-Stage Build

The immediate next major task is to implement the core multi-stage build logic within the `Builder::build` method (located in `src/builder.rs`). This method currently contains a `TODO` placeholder.

This will involve:

1.  **Analyzing `x.py`'s Build Stages:** A deeper dive into `src/bootstrap/bootstrap.py` to understand the precise sequence of `cargo` commands, environment setups, and intermediate steps required for each stage of the Rust compiler build (e.g., `stage0`, `stage1`, `stage2`).
2.  **Orchestrating `cargo` Commands:** Utilizing the `command_executor::execute_and_report_command` function to run `cargo build`, `cargo test`, `cargo install`, and other necessary commands for each stage.
3.  **Managing Build Artifacts:** Ensuring correct handling of output directories, intermediate compilation results, and the passing of artifacts between stages.
4.  **Cross-Compilation Details:** Integrating the specific requirements for cross-compiling the Rust compiler for the ARM64 Termux target within the build stages.
5.  **Error Handling and Reporting:** Implementing robust error handling for build failures at each stage.

This implementation will be a critical step towards `rust-bootstrap` fully replacing `x.py` for the Rust compiler's build process.