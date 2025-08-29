# CRQ: Implement rust-bootstrap build Subcommand

## Objective
Add a `build` subcommand to the `rust-bootstrap` command-line interface (CLI) that invokes the core build orchestration logic to compile the Rust compiler.

## Scope
*   Modify `rust-bootstrap`'s CLI argument parsing (likely using `clap`) to include a new `build` subcommand.
*   Integrate the `build` subcommand with the core build orchestration logic developed in `crq_implement_core_build_orchestration.md`.
*   Ensure that command-line arguments passed to `rust-bootstrap build` are correctly processed and forwarded to the build orchestration logic.
*   Implement clear and informative output for the `build` process, including progress indicators and error messages.
*   Provide initial testing to confirm that `rust-bootstrap build` can successfully initiate and complete a Rust compiler compilation.

## Impact
This CRQ provides a user-facing entry point for building the Rust compiler directly through `rust-bootstrap`, marking a significant step towards replacing `x.py` as the primary build tool. It allows for direct testing and iteration on the `rust-bootstrap` build process.

## Verification Steps
*   Running `cargo run -p rust-bootstrap -- build --help` displays correct help information for the new subcommand.
*   Executing `cargo run -p rust-bootstrap -- build <REPO_PATH>` successfully triggers the Rust compiler build process.
*   The build process completes without errors, producing the expected compiler artifacts.
*   Any errors during the build are clearly reported to the user.

## Dependencies
*   Successful completion of `crq_implement_core_build_orchestration.md`.
*   Familiarity with `clap` for Rust CLI development.
