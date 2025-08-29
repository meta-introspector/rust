## Change Request (CRQ): CRQ-BOOTSTRAP-006 - Git Analysis and Parquet Reporting

**1. Title:** Implement Git repository analysis and Parquet-based reporting.

**2. Description:**
This CRQ documents the implementation of a comprehensive Git repository analysis tool within `rust-bootstrap`. The tool extracts detailed information about Git objects (commits, trees, blobs, tags, and references), converts this data into Apache Arrow `RecordBatch`es, and writes it to Parquet files for efficient storage and analysis. This structured data provides a foundation for future build analysis, formal verification, and understanding the compiler's internal data flow. Additionally, this work includes enhancements to command execution metrics and the introduction of a build configuration report.

**3. Dependencies:**
*   `git2` crate for Git repository interaction.
*   `arrow` crate for in-memory data representation.
*   `parquet` crate for writing Parquet files.
*   `serde_json` for serialization.

**4. Deliverables:**
*   **Git Data Extraction:**
    *   Functions to extract all commits, trees, blobs, tags, and references from a Git repository.
    *   Conversion of Git data into Arrow `RecordBatch`es.
    *   Writing of Arrow `RecordBatch`es to individual Parquet files (e.g., `git_commits.parquet`, `git_blobs.parquet`).
*   **Git Analysis Summary:**
    *   A `GitAnalysisSummary` struct to provide a high-level overview of the analyzed repository.
    *   An Arrow schema for the `GitAnalysisSummary`.
    *   A function to write the `GitAnalysisSummary` to a Parquet file (`git_analysis_summary.parquet`).
    *   A function to read and display the summary from the Parquet file.
*   **Build Configuration Report:**
    *   A `build_config.parquet` report to store build configuration and stage0 information.
    *   A function to write the `BuildState` to the new parquet file.
    *   A function to read and summarize the build configuration metrics.
*   **Command Execution Metrics:**
    *   Enhanced `execute_shell_command` to capture and report command execution start time, end time, and duration.
*   **Refactoring:**
    *   Refactoring of the `operational_logger` module to follow the one-function-per-file principle.
*   **Documentation:**
    *   Updates to `docs/gemini_cli_instructions.md` to reflect the new capabilities.

**5. Verification:**
*   The `rust-bootstrap` tool compiles and runs without errors.
*   Parquet files for Git analysis and build configuration are generated correctly.
*   The `read_and_summarize_git_analysis_metrics` and `read_and_summarize_build_config_metrics` functions display the expected summary information.

**6. Coordination Instructions:**
*   **Reporting:** This CRQ is created post-implementation to document the work.
*   **Integration:** The changes have been integrated into the main codebase.
*   **Conflicts:** No conflicts are anticipated as this is a documentation of completed work.

**7. Status:** Documented
