## Change Request (CRQ) for `rust-bootstrap` Development

**CRQ ID:** `CRQ-RUST-BOOTSTRAP-001` (Initial Development)

**1. Title:**
   Development of Rust-Native Bootstrap (`rust-bootstrap`) for Rust Compiler.

**2. Description:**
   This Change Request outlines the initial development of a Rust-native bootstrap tool (`rust-bootstrap`) to replace the existing Python-based `x.py` build system for the Rust compiler. The goal is to enhance the trustworthiness, formal verifiability, and deep bootstrapping capabilities of the Rust compiler by moving the core build orchestration into a Rust-native environment.

**3. Objective:**
   *   To create a functional `rust-bootstrap` crate capable of parsing arguments, loading configuration, handling environment variables, and executing external commands.
   *   To establish a foundation for `rust-bootstrap` to detect and utilize an existing Rust compiler as its `stage0` for bootstrapping.
   *   To integrate `rust-bootstrap` development into the existing Rust repository via Git, ensuring proper version control and synchronization with upstream.

**4. Scope:**
   *   Creation of the `crates/rust-bootstrap` directory and its initial Rust source code.
   *   **4.1. Git Layer Data Extraction and Arrow Conversion:**
       *   Implementation of extractor functions (`get_all_blobs`, `get_all_trees`, `get_all_tags`, `get_all_refs`) to traverse Git repository objects and extract relevant metadata and content.
       *   Conversion of extracted Git data into Apache Arrow `RecordBatch`es, adhering to predefined schemas.
       *   Integration of a generic Parquet writing utility within the `parquet_reporter` module to persist `RecordBatch`es to `.parquet` files.
       *   Enhancement of the `analyze_git_repository` function to orchestrate the data extraction, Arrow conversion, and Parquet writing for all supported Git object types.
       *   Refinement of CLI argument parsing to accept a dynamic repository path for analysis.
   *   Implementation of core utilities: argument parsing, `config.toml` parsing, environment variable handling, and external command execution.
   *   Implementation of `stage0` compiler detection logic.
   *   Git integration: committing `rust-bootstrap` changes, adding `upstream` remote, and rebasing/merging with `upstream/master`.

**5. Impact:**
   *   **Positive:** Increased control over the bootstrap process, improved performance (Rust vs. Python), enhanced security and verifiability, and alignment with deep bootstrapping goals.
   *   **Negative:** Initial development effort, potential for new build complexities during transition.

**6. Verification Plan:**
   *   Successful compilation and execution of the `rust-bootstrap` executable.
   *   Correct parsing of command-line arguments and `config.toml`.
   *   Accurate detection and reporting of `RUSTC` and `CARGO` paths (including Termux defaults).
   *   Successful execution and output capture of external commands (e.g., `cargo --version`).
   *   Clean Git status after integration, with `rust-bootstrap` changes correctly committed and branch synced with `upstream/master`.

**7. Rollback Plan:**
   In case of critical issues, revert to the commit prior to the `rust-bootstrap` integration.

**8. Approvals:**
   *   User Approval: [Pending]
