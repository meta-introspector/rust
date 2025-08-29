## Change Request (CRQ): CRQ-BOOTSTRAP-003 - Implement `download_component` and `unpack_component` logic

**1. Title:** Implement `download_component` and `unpack_component` logic

**2. Description:**
   Translate the `download_component` and `unpack_component` functions from `bootstrap.py` into Rust. These functions handle the fetching of toolchain tarballs from the distribution server and their subsequent extraction.

**3. Dependencies:**
   *   `BuildState` struct (for `download_url`, `bin_root`, `verbose`, `stage0_data`, etc.)
   *   External crates for HTTP client (e.g., `reqwest`) and tarball handling (e.g., `flate2`, `tar`).

**4. Deliverables:**
   *   Create `crates/rust-bootstrap/src/bootstrap_stages/toolchain_downloader/download_component.rs`
   *   Create `crates/rust-bootstrap/src/bootstrap_stages/toolchain_downloader/unpack_component.rs`
   *   Update `crates/rust-bootstrap/src/bootstrap_stages/toolchain_downloader/mod.rs` to declare the new modules.
   *   (Potentially) Update `Cargo.toml` for new dependencies (`reqwest`, `flate2`, `tar`).

**5. Verification:**
   *   Ensure the functions compile without errors.
   *   (Manual) Verify that a test tarball can be downloaded and unpacked correctly.

**6. Coordination Instructions:**
   *   **Reporting:** Upon completion, update this CRQ document with "Status: Completed" and a brief summary of the work performed.
   *   **Integration:** The main Gemini thread will monitor this CRQ for completion and integrate the changes into the main codebase.
   *   **Conflicts:** In case of any unexpected issues or conflicts during implementation, report them immediately in this CRQ.

**7. Status:** Completed
