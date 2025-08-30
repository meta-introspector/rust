# Change Request (CRQ): Integrate Miri as a Rustc Source Tree Component

## 1. Objective
To ensure `rust-bootstrap` correctly interacts with `miri` as a component already present within the `rustc` source tree, rather than attempting to vendor it as an external dependency. This aligns with `miri`'s nature as an internal tool of the Rust compiler.

## 2. Scope
This CRQ covers:
*   Clarifying that `miri` is not to be vendored into `rust-bootstrap`'s `vendor` directory.
*   Ensuring `rust-bootstrap`'s build process correctly identifies and utilizes `miri` from its location within the `rustc` source tree (`src/tools/miri`).
*   Removing any `Cargo.toml` entries related to vendoring `miri`.
*   Addressing any compilation issues arising from `miri`'s dependencies on internal `rustc_*` crates by relying on the standard `rustc` build environment.

## 3. Prerequisites
*   `rust-bootstrap` is configured to build within the `rustc` source tree context.
*   Understanding of how `rustc`'s internal components (`rustc_*` crates) are made available during the `rustc` build process.

## 4. Procedure (High-Level)
### Step 4.1: Remove `miri` vendoring entries
*   **Action**: Remove `miri` as a dependency from `crates/rust-bootstrap/Cargo.toml`.
*   **Action**: Remove any `[patch.crates-io]` entries related to `miri` or its `rustc_*` dependencies from the top-level `Cargo.toml`.

### Step 4.2: Ensure `rust-bootstrap` can build `miri` as a `rustc` tool
*   **Action**: Verify that `rust-bootstrap`'s overall build process (once implemented) correctly includes and builds `miri` as part of the `rustc` toolchain, leveraging the implicit availability of `rustc_*` crates.

## 5. Verification
*   `rust-bootstrap` compiles without errors related to `miri` or its `rustc_*` dependencies.
*   `miri` is successfully built as a tool when `rust-bootstrap` completes a full `rustc` build.

## 6. Tools Used
*   `cargo` (for build, test)
*   `write_file`, `read_file`, `replace` (for file manipulation)
*   `run_shell_command` (for `cargo check`)

## 7. Challenges & Considerations
*   Ensuring `rust-bootstrap` correctly replicates the `rustc` build environment for `miri` and other tools.
*   Managing the complex interdependencies within the `rustc` source tree.