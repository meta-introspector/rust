# Rust Bootstrap Critical Path - Update and Testing Plan

This document serves as an update to the `rust_bootstrap_critical_path.md` and outlines the plan for enhancing test coverage for each critical stage, adhering to the principle of extreme modularity (one test case per file).

## Answer to: What's stopping us from adding all of Rust into our bootstrap code directly?

Adding the entire Rust compiler and its ecosystem directly into the `rust-bootstrap` codebase is generally not feasible or desirable for several fundamental reasons, rooted in the principles of compiler bootstrapping, trust, and software engineering best practices:

1.  **The Trusting Trust Problem (Ken Thompson Hack):** The core idea of bootstrapping a compiler is to build a trusted compiler from a minimal, verifiable base. If you start with a full, complex Rust compiler, you're implicitly trusting that entire codebase. A minimal bootstrap aims to reduce the "trusted computing base" to a very small, auditable core, which then builds the next, more complex stage. Including everything from the start defeats this security and verification goal.

2.  **Complexity and Maintainability:** The Rust compiler (`rustc`) and its standard library are incredibly complex, with millions of lines of code and a vast dependency graph. Integrating all of this directly into a bootstrap project would make the bootstrap itself astronomically complex, difficult to understand, maintain, debug, and evolve. It would become a monolithic beast.

3.  **Circular Dependencies and Self-Hosting:** Rust is a self-hosting language, meaning `rustc` is written in Rust. The bootstrap process inherently deals with this circular dependency. You need a working Rust compiler (the `stage0` compiler) to build a newer version of `rustc`. If you tried to include all of `rustc` directly, you'd immediately run into a chicken-and-egg problem without a pre-existing, trusted compiler.

4.  **Cross-Compilation and Portability:** A significant aspect of `rustc`'s build system is its ability to cross-compile for various targets. The bootstrap process often involves building a compiler for a different architecture or operating system than the host. A full `rustc` codebase is highly optimized for its own build process and dependencies, which might not be easily adaptable to the minimal environment of an early bootstrap stage or a new target.

5.  **Resource Constraints:** Compiling the entire Rust compiler is a resource-intensive process (CPU, RAM, disk space). A bootstrap aims to be as lightweight as possible in its early stages to run on diverse or constrained environments (like Termux on ARM64, as in our project's context). Including everything would make the bootstrap prohibitively heavy.

6.  **Dependency Management and Tooling:** The Rust ecosystem relies heavily on Cargo for dependency management. While `rust-bootstrap` aims to integrate with Cargo's *library* for in-process operations, it doesn't mean it should absorb all of Cargo's external dependencies or the entire `rustc` source tree. The bootstrap focuses on orchestrating the build, not replicating the entire toolchain.

7.  **Modularity and Separation of Concerns:** Good software design advocates for modularity. The `rust-bootstrap` project is designed to be a *bootstrap* system, not a full compiler. Its role is to orchestrate the build of `rustc`, not to *be* `rustc`. Keeping these concerns separate leads to a cleaner, more robust architecture.

In essence, the goal of `rust-bootstrap` is to provide a minimal, verifiable, and efficient mechanism to build the Rust compiler, not to become the Rust compiler itself.

## Plan for New Tests (Extreme Modularity):

For each critical path stage, new test modules and individual test functions will be created in separate files, adhering to the extreme modularity principle. This ensures granular testing and easier maintenance.

### Stage 1: Initial Setup & Configuration Loading
*   `tests/stage1/test_arg_parsing.rs`: Test parsing of command-line arguments.
*   `tests/stage1/test_default_config_loading.rs`: Test loading of a default `bootstrap.toml` (or equivalent) configuration.
*   `tests/stage1/test_custom_config_loading.rs`: Test loading of a custom configuration file.
*   `tests/stage1/test_invalid_config_error_handling.rs`: Test error handling for invalid configuration files.
*   `tests/stage1/test_global_context_initialization.rs`: Test `GlobalContext` initialization.

### Stage 2: Git Analysis & Data Extraction
*   `tests/stage2/test_git_blob_extraction.rs`: Test extraction of Git blobs.
*   `tests/stage2/test_git_tree_extraction.rs`: Test extraction of Git trees.
*   `tests/stage2/test_git_tag_extraction.rs`: Test extraction of Git tags.
*   `tests/stage2/test_git_ref_extraction.rs`: Test extraction of Git references.
*   `tests/stage2/test_git_data_to_arrow_conversion.rs`: Test conversion of Git data to Arrow `RecordBatch`es.
*   `tests/stage2/test_git_data_to_parquet_writing.rs`: Test writing of Git data to Parquet files.
*   `tests/stage2/test_missing_git_object_handling.rs`: Test handling of missing Git objects (new specific scenarios).

### Stage 3: Toolchain Management (Stage0 Detection & Download)
*   `tests/stage3/test_stage0_detection_existing.rs`: Test detection of an existing `stage0` compiler.
*   `tests/stage3/test_stage0_detection_not_found.rs`: Test detection when `stage0` is not found.
*   `tests/stage3/test_stage0_download_logic.rs`: Test logic for downloading a `stage0` compiler (mocking download if necessary).
*   `tests/stage3/test_stage0_version_handling.rs`: Test handling of different `stage0` versions.

### Stage 4: Core Cargo Integration & Command Execution
*   `tests/stage4/test_cargo_build_command_parsing.rs`: Test parsing and translation of `cargo build` command.
*   `tests/stage4/test_cargo_check_command_parsing.rs`: Test parsing and translation of `cargo check` command.
*   `tests/stage4/test_cargo_run_command_parsing.rs`: Test parsing and translation of `cargo run` command.
*   `tests/stage4/test_cargo_clean_command_parsing.rs`: Test parsing and translation of `cargo clean` command.
*   `tests/stage4/test_cargo_build_execution.rs`: Test execution of `cargo build` via `cargo` library (mocking external calls).
*   `tests/stage4/test_cargo_check_execution.rs`: Test execution of `cargo check` via `cargo` library.
*   `tests/stage4/test_exec_panic_enforcement.rs`: Test `exec_panic` enforcement (ensure it prevents shell commands).

### Stage 5: Basic Build Orchestration
*   `tests/stage5/test_basic_build_flow.rs`: Test the basic flow of orchestrating a build.
*   `tests/stage5/test_build_config_parquet_generation.rs`: Test generation of `build_config.parquet` report.
*   `tests/stage5/test_build_config_parquet_content.rs`: Test the content of `build_config.parquet` (basic validation).

This detailed plan will guide the creation of new, modular tests to ensure the robustness and verifiability of the `rust-bootstrap` critical path.