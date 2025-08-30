## Instructions for Gemini CLI: Git Layer Data Extraction and Arrow Conversion

### Current Status Update (August 29, 2025)

This document outlines the initial task for Git Layer Data Extraction and Arrow Conversion. As of this update, significant progress has been made, and many of the detailed steps originally outlined below have been completed or superseded by a more robust, Parquet-native reporting mechanism.

**Key Achievements and Current State:**

*   **Git Data Extraction Implemented:** All extractor functions (`get_all_blobs`, `get_all_trees`, `get_all_tags`, `get_all_refs`) have been implemented and integrated.
*   **Arrow Conversion & Parquet Writing:** Git data is successfully converted to Arrow `RecordBatch`es and written to individual Parquet files (`git_commits.parquet`, `git_blobs.parquet`, etc.).
*   **Refactored `analyze_git_repository`:** The `analyze_git_repository` function now orchestrates the full extraction and writing process, and returns a `GitAnalysisSummary` struct.
*   **Parquet-Native Reporting:** A new, robust reporting mechanism has been implemented:
    *   `GitAnalysisSummary` struct and its corresponding Arrow schema (`git_analysis_summary_schema()`) have been defined.
    *   `write_git_analysis_summary_to_parquet` function writes the `GitAnalysisSummary` to a dedicated Parquet file (`git_analysis_summary.parquet`).
    *   `read_and_summarize_git_analysis_metrics` function reads and prints the summary from the Parquet file.
    *   The `run_bootstrap` main orchestration function now calls these new reporting utilities.
*   **Compilation Errors Resolved:** Several compilation errors related to module paths, type mismatches, and `Box<dyn Error>` syntax have been systematically identified and resolved, adhering to the project's extreme modularity principles.

**Next Steps for Verification:**

1.  **Build Verification:** Ensure the `rust-bootstrap` crate compiles successfully after all recent changes.
2.  **Functionality Verification:** Run the `rust-bootstrap` binary with a test repository and confirm:
    *   The Git analysis summary is printed to the console.
    *   All expected Git data Parquet files (`git_commits.parquet`, `git_blobs.parquet`, etc.) and the `git_analysis_summary.parquet` file are created in the project root.

---

**Project Context:**

The `rust-bootstrap` crate is undergoing an extreme modularity refactoring, aiming for a "one function per file per basic block" structure. The long-term vision is to represent the Rust compiler's internal data flow and intermediate representations as Arrow/Parquet datasets, enabling advanced analysis and formal verification.

**Current State:**

*   The `rust-bootstrap` crate has been refactored into highly granular modules.
*   A `git_analyzer` module has been introduced to extract Git repository data.
*   Preliminary Arrow schemas for Git objects (commits, blobs, trees, tags, refs) have been defined in `crates/rust-bootstrap/src/git_analyzer/schemas/mod.rs`.
*   The `get_all_commits` function has been implemented in `crates/rust-bootstrap/src/git_analyzer/extractors/get_all_commits.rs` to extract commit data and convert it into an Arrow `RecordBatch`.
*   The `analyze_git_repository` function in `crates/rust-bootstrap/src/git_analyzer/analysis/analyze_git_repository.rs` is a placeholder that currently only opens the repository and calls `get_all_commits`.
*   New SOPs and schema definition documents have been added to `docs/sops/` and `docs/schemas/`.
*   The `git2` crate has been added as a dependency and its version conflict resolved.

**Next Task: Implement Git Data Extraction and Arrow Conversion for Remaining Git Objects**

The primary goal is to complete the Git Layer implementation by extracting data for blobs, trees, tags, and refs, and converting them into Arrow `RecordBatch`es.

**Detailed Steps:**

1.  **Implement `get_all_blobs` function:**
    *   **Location:** `crates/rust-bootstrap/src/git_analyzer/extractors/get_all_blobs.rs` (create this new file).
    *   **Function Signature:** `pub fn get_all_blobs(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error::Error>>`
    *   **Logic:**
        *   Iterate through all blobs in the repository. You might need to traverse the commit history and tree objects to find all blobs.
        *   For each blob, extract its hash, size, and content.
        *   Store this data in Rust vectors (e.g., `Vec<String>` for hashes, `Vec<u64>` for sizes, `Vec<Vec<u8>>` for content).
        *   Convert these vectors into Arrow `Array`s (e.g., `StringArray`, `UInt64Array`, `BinaryArray`).
        *   Create an Arrow `RecordBatch` from these arrays using `git_blobs_schema()` (defined in `crates/rust-bootstrap/src/git_analyzer/schemas/mod.rs`).
        *   Return the `RecordBatch`.

2.  **Implement `get_all_trees` function:**
    *   **Location:** `crates/rust-bootstrap/src/git_analyzer/extractors/get_all_trees.rs` (create this new file).
    *   **Function Signature:** `pub fn get_all_trees(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error::Error>>`
    *   **Logic:**
        *   Iterate through all tree objects in the repository. You'll need to traverse the commit history to find all trees.
        *   For each tree, iterate through its entries.
        *   For each entry, extract the tree hash, entry name, entry type (blob, tree, commit), entry ID (hash of the entry), and entry mode.
        *   Store this data in Rust vectors.
        *   Convert these vectors into Arrow `Array`s.
        *   Create an Arrow `RecordBatch` from these arrays using `git_trees_schema()`.
        *   Return the `RecordBatch`.

3.  **Implement `get_all_tags` function:**
    *   **Location:** `crates/rust-bootstrap/src/git_analyzer/extractors/get_all_tags.rs` (create this new file).
    *   **Function Signature:** `pub fn get_all_tags(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error::Error>>`
    *   **Logic:**
        *   Iterate through all tags in the repository.
        *   For each tag, extract its hash, name, target ID (hash of the object it points to), target type, tagger name, tagger email, tag time, and message.
        *   Store this data in Rust vectors.
        *   Convert these vectors into Arrow `Array`s.
        *   Create an Arrow `RecordBatch` from these arrays using `git_tags_schema()`.
        *   Return the `RecordBatch`.

4.  **Implement `get_all_refs` function:**
    *   **Location:** `crates/rust-bootstrap/src/git_analyzer/extractors/get_all_refs.rs` (create this new file).
    *   **Function Signature:** `pub fn get_all_refs(repo: &Repository) -> Result<RecordBatch, Box<dyn std::error::Error>>`
    *   **Logic:**
        *   Iterate through all references (branches, tags, remotes) in the repository.
        *   For each reference, extract its name, target ID (hash of the object it points to), and reference type (direct or symbolic).
        *   Store this data in Rust vectors.
        *   Convert these vectors into Arrow `Array`s.
        *   Create an Arrow `RecordBatch` from these arrays using `git_refs_schema()`.
        *   Return the `RecordBatch`.

5.  **Update `crates/rust-bootstrap/src/git_analyzer/extractors/mod.rs`:**
    *   Declare the new extractor modules (`get_all_blobs`, `get_all_trees`, `get_all_tags`, `get_all_refs`).

6.  **Update `crates/rust-bootstrap/src/git_analyzer/analysis/analyze_git_repository.rs`:**
    *   Call the newly implemented `get_all_blobs`, `get_all_trees`, `get_all_tags`, and `get_all_refs` functions.
    *   Print the number of extracted items for each type (similar to how commits are printed).
    *   **Crucially, implement writing these `RecordBatch`es to Parquet files.** You'll need to decide on a naming convention for the Parquet files (e.g., `git_commits.parquet`, `git_blobs.parquet`). You can reuse the `parquet_reporter` module for writing.

**Important Considerations:**

*   **Error Handling:** Continue to use `Result` and `?` for robust error handling.
*   **Memory Management:** Be mindful of memory usage when extracting large repositories. Arrow's columnar nature helps, but large blobs or many objects can still consume significant memory.
*   **Git Traversal:** For blobs and trees, you'll likely need to traverse the commit history and then the tree objects within each commit to find all unique blobs and trees. Avoid re-processing already seen objects.
*   **`git2` API:** Refer to the `git2` crate documentation for specific API calls to iterate and extract Git object data.
*   **Arrow Builders:** Use Arrow's `ArrayBuilder`s (e.g., `StringArrayBuilder`, `BinaryBuilder`, `ListBuilder`) to efficiently construct the Arrow arrays.

**Verification:**

*   After implementing each function, run `cargo build -p rust-bootstrap` to ensure compilation.
*   After updating `analyze_git_repository` and implementing Parquet writing, run the `rust-bootstrap` executable on a test Git repository (e.g., the current project's repository) and verify that the Parquet files are created and contain the expected data.
