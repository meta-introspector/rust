**Standard Operating Procedure (SOP): Debugging Arrow/Parquet Schema Mismatch in Rust**

**Objective:** To systematically diagnose and resolve `InvalidArgumentError` related to schema mismatches when working with Apache Arrow `RecordBatch`es and Parquet serialization in Rust, specifically focusing on `List` data types and nullability.

**Scope:** This SOP applies to Rust projects utilizing `arrow-rs` and `parquet-rs` crates where schema validation errors occur during `RecordBatch` creation or Parquet writing.

**Prerequisites:**
*   A Rust development environment with `cargo` installed.
*   Familiarity with Rust's module system, error handling (`Result`, `Box<dyn Error>`), and basic `git` commands.
*   Understanding of Apache Arrow data types, especially `List` and nullability.

**Procedure:**

**1. Initial Error Analysis:**
    a.  **Identify the exact error message:** Pay close attention to the `InvalidArgumentError` message, specifically the "expected" vs. "found" schema details (e.g., `nullable: false` vs. `nullable: true`).
    b.  **Locate the problematic code:** The error message usually points to the file and line number where the `RecordBatch` is being created or written to Parquet. In this case, `crates/rust-bootstrap/src/git_analyzer/extractors/get_all_commits.rs` and `crates/rust-bootstrap/src/git_analyzer/mod.rs` (where the schema is defined).
    c.  **Understand the data flow:** Trace how the data for the problematic column (e.g., `parent_hashes`) is generated and how its schema is defined.

**2. Schema and Data Consistency Check:**
    a.  **Review Schema Definition:** Examine the `Field::new` call for the problematic column in the schema definition (e.g., `git_commits_schema` in `crates/rust-bootstrap/src/git_analyzer/mod.rs`).
        *   For `List` types, ensure the `nullable` property of the *inner field* (the `item` field within the `List`) matches the nullability of the data being produced.
        *   Example: If the data contains `Option<String>` or is built with a `StringBuilder` (which often produces nullable items), the inner field's `nullable` property should be `true`.
    b.  **Review Data Generation:** Examine the code responsible for building the `Array` for the problematic column (e.g., `parent_hashes_builder` in `crates/rust-bootstrap/src/git_analyzer/extractors/get_all_commits.rs`).
        *   Confirm whether the builder explicitly handles nulls (e.g., `append_option`, `map(Some)`).
        *   Ensure the data being pushed into the builder aligns with the expected nullability.

**3. Runtime Schema Verification (if necessary):**
    a.  **Add `println!` statements:** Temporarily insert `println!("Actual RecordBatch Schema: {:#?}", record_batch.schema());` just before the `RecordBatch::try_new` call. This will print the actual schema inferred by `arrow-rs` from the data, which can reveal subtle discrepancies.
    b.  **Compare:** Compare the printed runtime schema with the schema defined in your code and the "expected" schema from the error message. This helps pinpoint where the mismatch originates.

**4. Dependency Management Check:**
    a.  **Review `Cargo.toml` and `Cargo.lock`:** Ensure that `arrow-rs` and `parquet-rs` dependencies are consistent across the workspace and are not conflicting.
    b.  **Update Dependencies:** If older versions are suspected, attempt to update to the latest compatible versions using `cargo update` or by explicitly setting versions in `Cargo.toml`.
    c.  **Clean Build:** After any dependency changes, perform a `cargo clean` followed by a `cargo build` or `cargo test` to ensure no stale artifacts are causing issues.

**5. Test Case Refinement:**
    a.  **Isolate the problem:** Create a minimal, self-contained unit test that specifically targets the problematic function and schema.
    b.  **Control external dependencies:** For tests involving file system or Git operations, use temporary directories (`tempfile` crate) and programmatically create test data to ensure reproducibility and isolation. This avoids issues with external state.

**6. Iterative Refinement and Verification:**
    a.  **Apply Fixes:** Based on the analysis, modify the schema definition or data generation logic to ensure consistency.
    b.  **Run Tests:** Execute the relevant unit tests (`cargo test -p <crate_name>`) to verify the fix.
    c.  **Repeat:** Continue iterating through diagnosis, fixing, and testing until the error is resolved.

**Tools Used:**
*   `git diff HEAD`: To inspect uncommitted changes.
*   `cargo test -p <crate_name>`: To run tests for a specific crate.
*   `cargo clean`: To remove build artifacts and force a fresh build.
*   `cargo update`: To update dependencies.
*   `read_file`: To inspect file contents.
*   `replace`: To modify file contents.
*   `run_shell_command`: To execute shell commands (e.g., `git init`, `mkdir`).
*   `tempfile` crate: For creating temporary directories in Rust tests.
