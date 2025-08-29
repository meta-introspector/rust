# CRQ: Analyze x.py Build Logic for rust-bootstrap Migration

## Objective
Conduct a deep dive into the `x.py` Python script to thoroughly understand its internal workings, focusing on how it orchestrates the Rust compiler build process, manages different build stages, and interacts with `bootstrap.toml`.

## Scope
*   Read and analyze the `x.py` source code, paying close attention to its main execution flow, command-line argument parsing, and subcommand dispatch logic.
*   Identify the specific functions, classes, and external calls `x.py` uses to perform core build operations (e.g., invoking `rustc`, `cargo`, managing dependencies, handling build profiles).
*   Document the interaction between `x.py` and `bootstrap.toml`, detailing how configuration settings influence the build process.
*   Prioritize understanding the logic for the `build` subcommand, as this is the most critical component for initial migration.
*   Identify any platform-specific logic or external tool dependencies that `x.py` relies upon.

## Impact
Provides a comprehensive understanding of the existing `x.py` build system, which is essential for accurately and completely migrating its functionality to `rust-bootstrap`. This analysis will inform the design and implementation of `rust-bootstrap`'s core build orchestration.

## Verification Steps
*   A detailed internal document or set of notes outlining the key components and flow of `x.py`'s build logic is created.
*   Specific `x.py` functions responsible for compilation, linking, and dependency management are identified.
*   The role of `bootstrap.toml` in `x.py`'s execution is clearly mapped out.
*   A clear understanding of the minimal set of operations required to build the Rust compiler using `x.py` is achieved.

## Dependencies
*   Access to the Rust project source repository.
*   Familiarity with Python scripting.
*   Prior understanding of the `rust-bootstrap` project's goals and existing capabilities.
