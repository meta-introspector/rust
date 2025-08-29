# CRQ: Implement Core Build Orchestration in rust-bootstrap

## Objective
Implement the foundational logic within `rust-bootstrap` to orchestrate the compilation of the Rust compiler, replicating the core build functionality of `x.py build`.

## Scope
*   Create a new module or extend an existing one within `rust-bootstrap` (e.g., `crates/rust-bootstrap/src/build_orchestrator.rs`) to encapsulate the build orchestration logic.
*   Develop functions to interpret the `bootstrap.toml` configuration (which `rust-bootstrap` already parses) and translate these settings into concrete build commands (e.g., `rustc` invocations, `cargo` commands).
*   Focus initially on implementing the minimal set of steps required to compile the Rust compiler, without necessarily handling all `x.py` features like testing or documentation generation.
*   Utilize Rust's `std::process::Command` or similar mechanisms to execute external commands (like `rustc` and `cargo`) as part of the build process.
*   Ensure proper error handling and reporting for build failures.

## Impact
This CRQ is a critical step towards `rust-bootstrap` becoming a self-sufficient build tool. Successfully completing this will enable `rust-bootstrap` to perform the fundamental task of compiling the Rust compiler, moving away from reliance on `x.py` for this core functionality.

## Verification Steps
*   The new build orchestration module compiles successfully within `rust-bootstrap`.
*   When invoked, the orchestration logic correctly interprets `bootstrap.toml` settings.
*   The orchestration logic successfully executes the necessary `rustc` and `cargo` commands to compile the Rust compiler.
*   (Initial verification will involve observing successful compilation output; later, integration with a `build` subcommand will provide more direct testing.)

## Dependencies
*   Successful completion of `crq_analyze_xpy_build_logic.md` (for understanding `x.py`'s build process).
*   Existing `rust-bootstrap` capabilities for argument parsing and `bootstrap.toml` loading.
