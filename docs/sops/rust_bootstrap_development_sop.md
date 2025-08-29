## Standard Operating Procedure (SOP) for `rust-bootstrap` Development and Integration

This SOP outlines the process for developing and integrating the `rust-bootstrap` crate into the Rust compiler's build system.

**1. Purpose:**
   To establish a standardized procedure for replacing the Python-based `x.py` bootstrap with a Rust-native `rust-bootstrap` crate, ensuring a robust, formally verifiable, and deeply bootstrapped Rust compiler.

**2. Scope:**
   This SOP applies to all development, testing, and integration activities related to the `rust-bootstrap` crate within the `rust` repository.

**3. Responsibilities:**
   *   **Gemini Agent:** Responsible for implementing the `rust-bootstrap` crate, performing code modifications, and executing build/test procedures.
   *   **User:** Responsible for providing guidance, reviewing changes, and approving critical steps.

**4. Procedure:**

   **4.1. Initial Setup of `rust-bootstrap` Crate:**
      a.  Create the `crates/rust-bootstrap` directory and initial `Cargo.toml` and `src/main.rs` files.
      b.  Add basic "Hello World" functionality to `src/main.rs`.
      c.  Add `clap`, `serde`, and `toml` as dependencies to `crates/rust-bootstrap/Cargo.toml`.
      d.  Implement argument parsing using `clap` in `src/main.rs`.
      e.  Implement `config.toml` parsing using `serde` and `toml`, including `BuildConfig` and `Config` structs.
      f.  Implement environment variable handling (`RUSTC`, `CARGO`) with Termux-specific defaults.
      g.  Implement external command execution using `std::process::Command` (e.g., `cargo --version`).
      h.  Add a `.gitignore` file to `crates/rust-bootstrap/` to ignore `target/` and `Cargo.lock`.

   **4.2. Git Integration and Synchronization:**
      a.  Stage and commit the `rust-bootstrap` crate and its related files (`crates/rust-bootstrap/`, `Cargo.lock`).
      b.  Add `rust-lang/rust` as an `upstream` remote if not already present.
      c.  Fetch latest changes from `upstream`.
      d.  Rebase the current development branch onto `upstream/master` (or `upstream/main`).
      e.  Resolve any merge conflicts during the rebase, prioritizing `upstream` changes where appropriate, and ensuring `rust-bootstrap` changes are preserved.
      f.  Continue the rebase until completion.
      g.  (Optional) Create a new branch for `rust-bootstrap` development if not already on one.

   **4.3. Core `stage0` Logic Implementation:**
      a.  Analyze `src/bootstrap/bootstrap.py` to understand its `stage0` detection and usage.
      b.  Implement a `Stage0` struct in `rust-bootstrap` to encapsulate `rustc` and `cargo` paths.
      c.  Develop logic within `rust-bootstrap` to detect the `stage0` compiler, prioritizing environment variables and falling back to Termux defaults.
      d.  Modify `rust-bootstrap` to use the detected `stage0` compiler for subsequent build orchestration.

   **4.4. Build Orchestration (Future Steps):**
      a.  Implement logic to parse `x.py` commands and translate them into Rust-native build steps.
      b.  Orchestrate `cargo` commands to build different stages of the Rust compiler (e.g., `stage1`, `stage2`).
      c.  Manage build artifacts and directories.

   **4.5. Testing and Verification (Future Steps):**
      a.  Develop unit tests for `rust-bootstrap` components.
      b.  Integrate `rust-bootstrap` with the existing Rust test suite.
      c.  Perform end-to-end testing to ensure `rust-bootstrap` can successfully build the Rust compiler.

**5. Documentation:**
   *   All code changes will be documented with clear comments explaining *why* changes are made.
   *   Relevant `README.md` files will be updated as `rust-bootstrap` matures.

**6. Review and Approval:**
   *   All significant changes will be reviewed by the user.
   *   Critical steps will require explicit user approval.
