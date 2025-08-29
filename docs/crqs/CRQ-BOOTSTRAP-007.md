## Change Request (CRQ): CRQ-BOOTSTRAP-007 - Argument Parsing and Configuration Loading

**1. Title:** Implement argument parsing and configuration loading in `rust-bootstrap`.

**2. Description:**
This CRQ covers the task of moving the argument parsing and configuration loading logic from `bootstrap.py` to the `rust-bootstrap` crate. This is the first step in migrating the entire bootstrap process to Rust. The goal is to have `rust-bootstrap` handle all the command-line arguments and configuration from `config.toml` or `bootstrap.toml` that are currently handled by `bootstrap.py`.

**3. Dependencies:**
*   `clap` crate for argument parsing.
*   `toml` crate for parsing TOML configuration files.

**4. Deliverables:**
*   **Argument Parsing:**
    *   Implement a `clap` based argument parser in `rust-bootstrap` that mirrors the arguments in `bootstrap.py`.
    *   The parsed arguments should be stored in a struct.
*   **Configuration Loading:**
    *   Implement a function to find and read the `config.toml` or `bootstrap.toml` file.
    *   Parse the TOML file and store the configuration in a struct.
*   **Parquet Report:**
    *   Create a Parquet report that documents all the possible command-line arguments and configuration options.
    *   The report should include the name of the argument/option, its description, and its default value.

**5. Verification:**
*   The `rust-bootstrap` tool compiles and runs without errors.
*   The argument parser correctly parses command-line arguments.
*   The configuration loader correctly reads and parses the TOML configuration file.
*   The Parquet report is generated correctly and contains the expected information.

**6. Coordination Instructions:**
*   **Reporting:** This CRQ will be updated with the status of the implementation.
*   **Integration:** The changes will be integrated into the main codebase.
*   **Conflicts:** Any conflicts will be reported in this CRQ.

**7. Status:** In Progress

**Notes:**
- The argument parsing and configuration loading have been implemented.
- The Parquet report for arguments and options is not yet implemented.