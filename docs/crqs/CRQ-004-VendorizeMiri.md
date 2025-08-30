# Change Request (CRQ): Vendorize Miri for Monolithic Mini Rust

## 1. Objective
To integrate `miri` (Rust's Mid-level Intermediate Representation interpreter) as a vendored dependency within the `rust-bootstrap` project, enabling in-memory execution of Rust MIR for the "monolithic mini Rust" initiative.

## 2. Scope
This CRQ covers:
*   Vendorizing the `miri` source code from the `rust-lang/rust` repository (specifically `src/tools/miri`).
*   Configuring `miri` as a dependency within the `rust-bootstrap` crate.
*   Resolving any compilation or dependency conflicts arising from `miri`'s integration.
*   Exposing a minimal API from the vendored `miri` that `rust-bootstrap` can use to execute MIR.

## 3. Prerequisites
*   Successful compilation of the `rust-bootstrap` crate with its existing `cargo` integration.
*   Familiarity with `miri`'s internal architecture and its API for MIR interpretation.
*   Understanding of Rust's vendoring process and dependency management for complex crates.

## 4. Procedure (High-Level)
### Step 4.1: Add `miri` as a Vendored Dependency
*   **Action**: Copy the `miri` source code (typically `src/tools/miri` from the `rust-lang/rust` repository) into a `vendor` directory within the `rust-bootstrap` workspace (e.g., `crates/rust-bootstrap/vendor/miri`).
*   **Tool**: `run_shell_command` (for `cp -r`)

### Step 4.2: Configure `miri` in `Cargo.toml`
*   **Action**: Add `miri` as a path dependency in `crates/rust-bootstrap/Cargo.toml`, pointing to the vendored source.
*   **Tool**: `replace`

### Step 4.3: Resolve Dependencies and Compilation Issues
*   **Action**: Address any new compilation errors or dependency conflicts introduced by `miri`. This may involve adjusting feature flags, patching `Cargo.toml` files within the vendored `miri`, or resolving version mismatches.
*   **Tool**: `read_file`, `replace`, `run_shell_command` (`cargo check`)

### Step 4.4: Expose Minimal `miri` API
*   **Action**: Identify and expose the necessary functions or structs from `miri` that `rust-bootstrap` will use to interpret and execute MIR. This may involve creating a small wrapper module.
*   **Tool**: `write_file`, `replace`

## 5. Verification
*   **Step 5.1**: Run `cargo check -p rust-bootstrap` to ensure that `rust-bootstrap` compiles successfully with `miri` as a dependency.
*   **Step 5.2**: Develop a basic unit test within `rust-bootstrap` that calls the exposed `miri` API to execute a simple MIR snippet and verifies its output.
*   **Step 5.3**: Monitor binary size to assess the impact of `miri` integration.

## 6. Tools Used
*   `run_shell_command`: For copying files and running `cargo check`.
*   `read_file`: To inspect `Cargo.toml` and source files.
*   `replace`: To modify `Cargo.toml` and source files.
*   `write_file`: To create new wrapper modules or test files.

## 7. Challenges & Considerations
*   **Binary Size**: Significant challenge due to embedding `rustc`, `cargo`, and LLVM. Requires aggressive stripping and minimal LLVM subset.
*   **Complexity of `miri`**: `miri` is a large and complex tool; integrating it may expose deep dependencies or require specific toolchain configurations.
*   **API Stability**: `miri`'s internal API might not be stable, requiring ongoing maintenance as `miri` evolves.
*   **Performance**: In-memory MIR execution performance needs to be evaluated against the project's goals.

## 8. Potential Use Cases
*   Enabling the "interpreter-like behavior" for the monolithic mini Rust.
*   Providing a robust and formally verified execution environment for Rust code snippets.
*   Facilitating advanced static analysis and code understanding within `rust-bootstrap`.
