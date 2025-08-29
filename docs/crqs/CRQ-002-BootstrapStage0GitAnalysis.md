# Change Request (CRQ): Rust Bootstrap - Stage0 Detection and Git Analysis Integration

## 1. Objective
To implement robust `stage0` Rust compiler detection within `rust-bootstrap` and integrate the Git repository analysis functionality, ensuring that Git data can be extracted and converted into Arrow/Parquet datasets.

## 2. Scope
This CRQ covers:
*   Implementation of `stage0` compiler detection logic.
*   Integration of Git data extractor functions.
*   Integration of the Parquet reporting mechanism.
*   Integration of Git analysis into the main `rust-bootstrap` flow.

## 3. Prerequisites
*   Completion of CRQ: `rust_bootstrap_initial_setup.md`.
*   A Git repository available for testing the analysis functionality.

## 4. Procedure

### Step 2.1: Implement `stage0` Detection Logic.
*   **Action:** Analyze `src/bootstrap/bootstrap.py` to understand its `stage0` detection mechanism. Implement a `Stage0` struct in `rust-bootstrap` to encapsulate `rustc` and `cargo` paths. Develop logic to detect the `stage0` compiler, prioritizing environment variables and falling back to Termux defaults. (Refer to `rust_bootstrap_development_sop.md` section 4.3).
*   **Tool:** `read_file`, `write_file`, `replace`

### Step 2.2: Integrate Git Analysis Extractor Functions.
*   **Action:** Implement the Git data extractor functions (`get_all_blobs`, `get_all_trees`, `get_all_tags`, `get_all_refs`, `get_all_commits`) within `crates/rust-bootstrap/src/git_analyzer/extractors/`. These functions should interact with the `git2` library to traverse the repository, collect data into Rust `Vec`s, and convert them into Apache Arrow `Array`s and `RecordBatch`es. (Refer to `gemini_git_analysis_sop.md` section 4.2).
*   **Tool:** `write_file`, `replace`, `run_shell_command` (for creating a test repo)

### Step 2.3: Integrate Parquet Reporter.
*   **Action:** Implement the generic `write_record_batch_to_parquet` function in `crates/rust-bootstrap/src/parquet_reporter/mod.rs`. This function should accept an `arrow_array::RecordBatch` and a `file_path` as input, and write the `RecordBatch` to a Parquet file. (Refer to `gemini_git_analysis_sop.md` section 4.3).
*   **Tool:** `write_file`, `replace`

### Step 2.4: Integrate Git Analysis into Main Flow.
*   **Action:** Update `crates/rust-bootstrap/src/git_analyzer/analysis/analyze_git_repository.rs` to call the newly implemented extractor functions. For each `RecordBatch` returned, use the `parquet_reporter` to write it to a Parquet file with a descriptive filename (e.g., `git_commits.parquet`). (Refer to `gemini_git_analysis_sop.md` section 4.4).
*   **Tool:** `write_file`, `replace`, `run_shell_command`

## 5. Verification
*   **Step 5.1:** Run `rust-bootstrap` and confirm it correctly identifies the `stage0` compiler path.
*   **Step 5.2:** Execute `rust-bootstrap` with a test Git repository. Verify that Parquet files (e.g., `git_commits.parquet`, `git_blobs.parquet`) are created in the expected output directory.
*   **Step 5.3:** (Optional) Use a Parquet viewer or library to inspect the generated files and confirm data integrity and schema correctness.

## 6. Tools Used
*   `run_shell_command`: For executing shell commands, `cargo` commands, and creating test Git repositories.
*   `write_file`: For creating and modifying Rust source files.
*   `replace`: For in-place modifications of existing code.
*   `read_file`: For inspecting file contents (e.g., `bootstrap.py`).
