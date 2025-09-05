# SOP-NCI-002: Pilot Crate Integration

## Purpose
To test and validate the proposed new crate integration method with a single, representative "new work" crate.

## Procedure
1.  Select a pilot crate (e.g., `crates/rust-bootstrap`) for initial integration testing.
2.  Implement the proposed integration method (from SOP-NCI-001) by making the necessary modifications to the `rustc` repository.
3.  Execute the `rustc` build process (via `x.py`) to attempt compilation of the integrated pilot crate.
4.  Capture and analyze all compilation output (stdout, stderr, exit codes).
5.  If compilation fails, identify the root cause of the failure. This may involve:
    *   Debugging dependency resolution issues.
    *   Addressing unstable feature usage.
    *   Resolving API mismatches between `rustc` components and their dependencies.
    *   Patching vendored crates or the new crate itself (documenting all patches).
6.  Iterate on fixing and re-compiling until the pilot crate successfully compiles within the `rustc` workspace.

## Tools
*   `read_file`: To inspect code and configuration files.
*   `write_file`: To apply modifications to files.
*   `replace`: For targeted text replacements in files.
*   `run_shell_command`: To execute `x.py` and `cargo` commands.
*   Rust compiler and associated tools (e.g., `cargo`).
*   Debugging tools (as needed, e.g., `strace`, `gdb` if available).

## Output
*   A detailed log of all integration steps, compilation attempts, error messages, and fixes applied.
*   Confirmation of successful compilation of the pilot crate.

## Verification
*   Successful compilation of the pilot crate within the `rustc` workspace.
*   Review of the detailed log by a "Build Engineer" agent.
