# Standard Operating Procedure (SOP): Gemini's Git Analysis Implementation Process

## 1. Objective
This SOP outlines the process followed by the Gemini CLI agent to implement and verify the Git data extraction and Arrow/Parquet conversion functionality within the `rust-bootstrap` crate, adhering to the project's extreme modularity principles.

## 2. Scope
This procedure covers the steps from understanding the task requirements to implementing code, debugging, and verifying the successful execution of the Git analysis module.

## 3. Prerequisites
*   Access to the `rust-bootstrap` crate within the project repository.
*   Familiarity with Rust programming language, `git2` crate, and Apache Arrow/Parquet concepts.
*   Understanding of the project's "extreme modularity" and "one function per file per basic block" principles.
*   Access to `cargo` for building and running Rust projects.

## 4. Procedure

### 4.1. Task Understanding and Planning
*   **Read Instructions:** Thoroughly review the provided `gemini_cli_instructions.md` to understand the specific requirements, desired outcomes, and file locations for new implementations (e.g., `get_all_blobs`, `get_all_trees`, `get_all_tags`, `get_all_refs`).
*   **Identify Gaps:** Compare the instructions with the existing codebase to identify missing functions, modules, or integration points.
*   **Review Existing Code:** Examine related files (e.g., `src/git_analyzer/mod.rs`, `src/git_analyzer/analysis/analyze_git_repository.rs`, `src/parquet_reporter/mod.rs`) to understand current structure, conventions, and available utilities.

### 4.2. Implementation - Extractor Functions
*   **Create New Files:** For each required extractor function (e.g., `get_all_blobs`, `get_all_trees`, `get_all_tags`, `get_all_refs`), create a new `.rs` file in the specified `crates/rust-bootstrap/src/git_analyzer/extractors/` directory.
*   **Implement Logic:**
    *   Write the Rust code to interact with the `git2` library to traverse the repository and extract the relevant Git object data (blobs, trees, tags, refs).
    *   Collect extracted data into appropriate Rust `Vec`s (e.g., `Vec<String>`, `Vec<u64>`, `Vec<Vec<u8>>`).
    *   Convert these `Vec`s into Apache Arrow `Array`s (e.g., `StringArray`, `UInt64Array`, `BinaryArray`, `TimestampNanosecondArray`).
    *   Utilize `HashSet`s where necessary to track visited objects and prevent redundant processing or infinite loops during graph traversal (e.g., for blobs and trees).
    *   Construct an `arrow_array::RecordBatch` from the Arrow `Array`s using the predefined schemas (e.g., `git_blobs_schema()`).
    *   Ensure proper error handling using `Result` and `?`.
*   **Update Extractor Module:** Modify `crates/rust-bootstrap/src/git_analyzer/extractors/mod.rs` to declare the newly created extractor modules (`pub mod <module_name>;`). If `mod.rs` does not exist, create it.

### 4.3. Implementation - Parquet Reporter Integration
*   **Analyze `parquet_reporter`:** Read `crates/rust-bootstrap/src/parquet_reporter/mod.rs` to understand its existing functionality.
*   **Add Generic Write Function:** Implement a new public function, `write_record_batch_to_parquet`, in `crates/rust-bootstrap/src/parquet_reporter/mod.rs`. This function should accept an `arrow_array::RecordBatch` and a `file_path` as input, and write the `RecordBatch` to a Parquet file. This typically involves using internal `parquet_reporter` utilities like `write_to_parquet_file` and `close_parquet_writer`.
*   **Add Necessary Imports:** Ensure `arrow_array::RecordBatch` is imported at the top level of `parquet_reporter/mod.rs` if needed.

### 4.4. Integration into Main Analysis Flow
*   **Update `analyze_git_repository`:** Modify `crates/rust-bootstrap/src/git_analyzer/analysis/analyze_git_repository.rs`.
    *   Import all newly implemented extractor functions.
    *   Call each extractor function, storing the returned `RecordBatch`.
    *   Print the number of records extracted for each Git object type.
    *   Call `parquet_reporter::write_record_batch_to_parquet` for each `RecordBatch`, specifying a descriptive output filename (e.g., `git_commits.parquet`).

### 4.5. CLI Argument Handling
*   **Identify Argument Definition:** Locate the `Args` struct definition (e.g., `crates/rust-bootstrap/src/bootstrap_stages/cli_parser/create_args_struct.rs`).
*   **Add Repository Path Argument:** Add a new field (e.g., `pub repo_path: String`) to the `Args` struct to accept the Git repository path as a command-line argument.
*   **Pass Argument to Analyzer:** Update `crates/rust-bootstrap/src/main.rs` to pass the parsed `repo_path` argument to the `analyze_git_repository` function.

### 4.6. Debugging and Verification
*   **Compile Frequently:** After each significant change (e.g., implementing a new extractor, updating a `mod.rs`), run `cargo build -p rust-bootstrap` to catch compilation errors early.
*   **Address Compilation Errors:**
    *   **Type Mismatches:** Correct `arrow_array` builder types (e.g., `StringArrayBuilder` to `StringBuilder`).
    *   **Lifetime Issues (E0597, E0716):** Ensure all borrowed data (e.g., `&str` from `git2` objects) is converted to owned data (`String` or `Vec<u8>`) before being stored in vectors or arrays that outlive the borrowed source.
    *   **Missing Imports:** Add `use` statements for all necessary modules and types.
*   **Run and Verify:** Execute the `rust-bootstrap` binary with a test repository path (`cargo run -p rust-bootstrap -- <repo_path>`).
    *   Check the console output for expected messages (e.g., "Extracted X commits.", "RecordBatch written to Y.parquet").
    *   Verify that the Parquet files are created in the expected location.
    *   (Optional) Use a Parquet viewer or library to inspect the generated files and confirm data integrity.

## 5. Ethical Considerations
*   **Trustworthiness:** Ensure the extracted data is accurate and reflects the true state of the Git repository.
*   **Efficiency:** Be mindful of performance, especially when traversing large repositories, and optimize data extraction where possible.

## 6. Documentation and Commit
*   **Update CRQ:** Document the completed work in the relevant Change Request (CRQ) file, detailing the implementation and verification steps.
*   **Commit Changes:** Stage all modified and new files, and create a clear, concise commit message summarizing the work done.
*   **Final Test:** Run a final build and execution after documentation and commit to ensure no regressions.
