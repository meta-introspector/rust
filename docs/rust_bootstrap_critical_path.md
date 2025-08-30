# Rust Bootstrap Critical Path

This document outlines the critical path stages for the `rust-bootstrap` project, focusing on the foundational steps required to achieve a self-hosting Rust build system. These stages represent the core functionalities that must be robust and verifiable for the project to succeed.

## Critical Path Stages:

1.  **Initial Setup & Configuration Loading:**
    *   **Purpose:** To initialize the `rust-bootstrap` environment and load essential build configurations.
    *   **Key Components:**
        *   Command-line argument parsing for `rust-bootstrap`.
        *   Loading and interpreting build configuration (e.g., `config.toml` equivalents).
        *   Establishing the `GlobalContext` for the build process.

2.  **Git Analysis & Data Extraction:**
    *   **Purpose:** To extract comprehensive Git repository data, which is fundamental for understanding the source code's history and enabling formal verification and deep bootstrap.
    *   **Key Components:**
        *   Functions for extracting Git blobs, trees, tags, and references.
        *   Conversion of extracted Git data into Arrow `RecordBatch`es.
        *   Writing this data to Parquet files for persistent storage and analysis.

3.  **Toolchain Management (Stage0 Detection & Download):**
    *   **Purpose:** To identify and acquire the necessary `stage0` compiler and other toolchain components required for the bootstrap process.
    *   **Key Components:**
        *   Logic for detecting the presence and version of the `stage0` compiler.
        *   Mechanisms for downloading or locating required toolchains and dependencies.

4.  **Core Cargo Integration & Command Execution:**
    *   **Purpose:** To enable `rust-bootstrap` to interact directly with the `cargo` build system, replacing external `cargo` process calls with in-process library calls.
    *   **Key Components:**
        *   Parsing and translating `cargo` commands into `cargo` library calls.
        *   Execution of `cargo` operations (e.g., `build`, `check`) using the `cargo` library.
        *   Enforcement of `exec_panic` to prevent direct shell command execution, promoting monadic execution.

5.  **Basic Build Orchestration:**
    *   **Purpose:** To implement the initial logic for orchestrating the Rust compiler build, laying the groundwork for full `x.py` feature parity.
    *   **Key Components:**
        *   Porting core build orchestration logic from `x.py`.
        *   Generation of the `build_config.parquet` report, detailing the build environment and configuration.

This critical path will guide our testing efforts and ensure that the foundational aspects of `rust-bootstrap` are robust and reliable.
