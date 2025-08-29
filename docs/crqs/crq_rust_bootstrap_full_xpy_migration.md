# CRQ: rust-bootstrap Full x.py Migration

## Objective
Refactor and extend `rust-bootstrap` to fully replace the functionality of the Python-based `x.py` build system, enabling `rust-bootstrap` to drive the entire Rust compiler build and installation process from source.

## Scope
*   **Configuration Migration:** Implement all remaining build configuration logic currently handled by `x.py setup` and `./configure` within `rust-bootstrap`.
*   **Core Build Logic Migration:** Migrate the core compilation logic from `x.py build` into `rust-bootstrap`, including managing build stages, dependencies, and compilation flags.
*   **Installation Logic Migration:** Migrate the installation logic from `x.py install` into `rust-bootstrap`, ensuring correct placement of binaries, libraries, and documentation.
*   **Duplicate Code Refactoring:** Address and eliminate duplicate code identified in `src/bootstrap_stages/command_executor` and `src/main_stages/command_executor` within `rust-bootstrap`.
*   **Toolchain Management:** Ensure `rust-bootstrap` can manage toolchain bootstrapping and snapshot fetching, replicating `x.py`'s capabilities.
*   **Documentation Update:** Update all relevant project documentation to reflect `rust-bootstrap` as the primary and sole build tool for the Rust compiler.

## Impact
Achieves the long-term vision of a Rust-native, formally verifiable build system for the Rust compiler. This will lead to improved build integration, potentially enhanced efficiency, and enable more advanced analysis and formal verification of the build process itself. It streamlines the development workflow by consolidating build logic into a single, Rust-native tool.

## Verification Steps
*   `rust-bootstrap` can successfully build and install the Rust compiler from scratch on supported platforms (including Termux) without any reliance on `x.py`.
*   All features and configurations previously supported by `x.py` are fully replicated and functional within `rust-bootstrap`.
*   A comprehensive test suite for `rust-bootstrap` passes, ensuring correctness and stability.
*   Performance metrics demonstrate that `rust-bootstrap` is comparable to or superior in performance to `x.py` for typical build scenarios.
*   All project documentation accurately reflects the use of `rust-bootstrap` as the primary build tool.

## Dependencies
*   Successful completion of `crq_build_rust_termux_xpy.md` (for a reference build environment).
*   Successful completion of `crq_analyze_rust_build_with_rust_bootstrap.md` (for initial analysis capabilities).
*   Ongoing development and contributions to the `rust-bootstrap` crate.
