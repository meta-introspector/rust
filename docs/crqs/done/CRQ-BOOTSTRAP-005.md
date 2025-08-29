## Change Request (CRQ): CRQ-BOOTSTRAP-005 - Implement `fix_bin_or_dylib` and related Nix logic

**1. Title:** Implement `fix_bin_or_dylib` and related Nix logic

**2. Description:**
   Translate the `RustBuild.should_fix_bins_and_dylibs()` and `RustBuild.fix_bin_or_dylib()` methods from `bootstrap.py` into Rust. This involves detecting if the system is NixOS and, if so, using `patchelf` to modify the interpreter and RPATH of ELF executables to ensure they link correctly against Nix store paths.

**3. Dependencies:**
   *   External command execution utility (e.g., `run_shell_command` equivalent).
   *   `BuildState` struct (for configuration, `build_dir`, `verbose`).

**4. Deliverables:**
   *   Create `crates/rust-bootstrap/src/bootstrap_stages/nix_patcher/mod.rs`
   *   Create `crates/rust-bootstrap/src/bootstrap_stages/nix_patcher/should_fix_bins_and_dylibs.rs`
   *   Create `crates/rust-bootstrap/src/bootstrap_stages/nix_patcher/fix_bin_or_dylib.rs`
   *   Update `crates/rust-bootstrap/src/bootstrap_stages/mod.rs` to declare the new module.
   *   (Potentially) Update `Cargo.toml` for new dependencies if needed for OS detection or external command execution.

**5. Verification:**
   *   Ensure the functions compile without errors.
   *   (Manual, on NixOS) Verify that binaries are correctly patched after a toolchain download.

**6. Coordination Instructions:**
   *   **Reporting:** Upon completion, update this CRQ document with "Status: Completed" and a brief summary of the work performed.
   *   **Integration:** The main Gemini thread will monitor this CRQ for completion and integrate the changes into the main codebase.
   *   **Conflicts:** In case of any unexpected issues or conflicts during implementation, report them immediately in this CRQ.

**7. Status:** Completed
