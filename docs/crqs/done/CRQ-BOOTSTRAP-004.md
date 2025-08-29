## Change Request (CRQ): CRQ-BOOTSTRAP-004 - Implement `download_toolchain` orchestration

**1. Title:** Implement `download_toolchain` orchestration

**2. Description:**
   Translate the `RustBuild.download_toolchain()` method from `bootstrap.py` into Rust. This function orchestrates the entire stage0 toolchain management process, including checking if components are out of date, downloading them if necessary, and unpacking them.

**3. Dependencies:**
   *   CRQ-BOOTSTRAP-002: Implement `rustc_stamp` function.
   *   CRQ-BOOTSTRAP-003: Implement `download_component` and `unpack_component` logic.
   *   `program_out_of_date` function (already implemented).
   *   `bin_root` function (already implemented).
   *   `BuildState` struct.

**4. Deliverables:**
   *   Create `crates/rust-bootstrap/src/main_stages/download_and_setup_toolchain.rs`
   *   Update `crates/rust-bootstrap/src/main_stages/mod.rs` to declare the new module.
   *   Update `crates/rust-bootstrap/src/main.rs` to call `download_and_setup_toolchain`.

**5. Verification:**
   *   Ensure the function compiles without errors.
   *   Run `rust-bootstrap` and verify that the stage0 toolchain is correctly downloaded and set up (check for existence of `rustc` and `cargo` in the `bin_root` directory).

**6. Coordination Instructions:**
   *   **Reporting:** Upon completion, update this CRQ document with "Status: Completed" and a brief summary of the work performed.
   *   **Integration:** The main Gemini thread will monitor this CRQ for completion and integrate the changes into the main codebase.
   *   **Conflicts:** In case of any unexpected issues or conflicts during implementation, report them immediately in this CRQ.

**7. Status:** Completed
