# Task Templates from Git History Analysis

These templates are designed to guide future tasks, reflecting common development patterns observed in the project's recent history.

---

### **Theme: Documentation & Communication**

**1. Task: Refine Gemini CLI Prompt Template**
*   **Pattern to Reuse:** Iterative improvement of documentation based on project needs and observed patterns.
```
# 1. Task Description
Refine the `gemini_cli_prompt_template.md` based on new insights or project requirements.

# 2. Context and Background
The file is located at `/data/data/com.termux/files/home/storage/github/rust/docs/gemini_cli_prompt_template.md`.
(Provide specific insights or requirements here, e.g., "Consider adding a section on X" or "Refine the examples to include Y.")

# 3. Desired Output
Provide the updated content of `gemini_cli_prompt_template.md` as a Markdown code block.

# 4. Constraints and Guidelines
- Adhere to existing Markdown formatting.
- Ensure clarity and conciseness.

# 5. Verification (Optional)
N/A
```

**2. Task: Generate Creative Content (e.g., Poems) based on Project Themes**
*   **Pattern to Reuse:** Using AI for creative content generation related to project documentation or internal communication.
```
# 1. Task Description
Generate a poem (or other creative text) about [specific project theme/concept, e.g., "modularity," "Rust bootstrap," "formal verification"].

# 2. Context and Background
(Provide any relevant background, keywords, or desired tone/style for the creative content.)

# 3. Desired Output
Provide the generated poem (or text) as a Markdown code block.

# 4. Constraints and Guidelines
- (Specify any constraints on length, rhyming, style, etc.)

# 5. Verification (Optional)
N/A
```

**3. Task: Refactor and Reorganize Documentation Files**
*   **Pattern to Reuse:** Maintaining a clear and organized documentation structure, separating user-facing guides from internal task instructions.
```
# 1. Task Description
Refactor and reorganize documentation files within the `docs/` directory to improve clarity and accessibility.

# 2. Context and Background
(Specify which files or sections need reorganization, e.g., "Move all internal task instructions to `docs/internal_gemini_tasks/`" or "Create a new user guide for X.")

# 3. Desired Output
Provide a plan for the reorganization, including proposed file paths and content summaries.
(Alternatively, if direct action is desired, specify the `mv`, `write_file`, or `replace` commands.)

# 4. Constraints and Guidelines
- Maintain clear separation between user-facing and internal documentation.
- Ensure all links are updated if files are moved.

# 5. Verification (Optional)
N/A
```

**4. Task: Update Relevant CRQ(s) and Documentation Files to Reflect a Feature Implementation**
*   **Pattern to Reuse:** Keeping documentation (especially CRQs) synchronized with code changes.
```
# 1. Task Description
Update relevant CRQ(s) and documentation files to reflect the implementation of [specific feature/task].

# 2. Context and Background
(Specify the feature/task that was implemented and list the CRQ(s) and documentation files that need updating.)
Example: "The `feature X` has been implemented. Update `docs/crqs/CRQ-XXX-FeatureX.md` and `docs/development_documentation.md`."

# 3. Desired Output
Provide the updated content for the specified files, highlighting the changes.

# 4. Constraints and Guidelines
- Ensure accuracy and completeness of the updates.
- Follow CRQ and documentation standards.

# 5. Verification (Optional)
N/A
```

**5. Task: Update Documentation for Refactoring Efforts**
*   **Pattern to Reuse:** Documenting significant code refactoring, especially when it involves new tools or architectural changes.
```
# 1. Task Description
Update documentation to reflect recent refactoring of [specific component/area, e.g., "rust-bootstrap" or "Git analysis"].

# 2. Context and Background
(Specify the refactored component and the documentation files to be updated. Provide a brief summary of the refactoring changes.)
Example: "The `rust-bootstrap` crate has undergone significant modularity refactoring, and `git-analyzer-cli` was separated. Update `docs/development_documentation.md` and any relevant SOPs."

# 3. Desired Output
Provide the updated content for the specified documentation files.

# 4. Constraints and Guidelines
- Clearly explain the architectural changes and their implications.
- Ensure accuracy and consistency with the codebase.

# 5. Verification (Optional)
N/A
```

**6. Task: Update Project README with Current Status and Plans**
*   **Pattern to Reuse:** Keeping top-level documentation up-to-date with project progress and future direction.
```
# 1. Task Description
Update the `README.md` file for `[project_name]` to reflect its current status and future plans.

# 2. Context and Background
The `README.md` is located at `[path_to_readme]`.
(Provide a summary of the current status and key aspects of the future plans to be included in the README.)

# 3. Desired Output
Provide the updated content for the `README.md` file.

# 4. Constraints and Guidelines
- Ensure clarity, conciseness, and accuracy.
- Highlight key features and usage instructions.

# 5. Verification (Optional)
N/A
```

**7. Task: Organize Completed CRQs into a "Done" Directory**
*   **Pattern to Reuse:** Maintaining an organized system for tracking completed tasks/CRQs.
```
# 1. Task Description
Move completed Change Request (CRQ) documents from `docs/crqs/` to `docs/crqs/done/`.

# 2. Context and Background
(List the specific CRQ files that are completed and need to be moved.)

# 3. Desired Output
Provide the `mv` commands to move the specified CRQ files.

# 4. Constraints and Guidelines
- Ensure only truly completed CRQs are moved.
- Update any internal links that might point to the old location (if applicable).

# 5. Verification (Optional)
- Verify the files are moved to the correct directory.
```

**8. Task: Establish a Standard Operating Procedure (SOP) for Documentation Standards**
*   **Pattern to Reuse:** Formalizing documentation practices for consistency and quality.
```
# 1. Task Description
Establish a Standard Operating Procedure (SOP) for documentation standards within the project.

# 2. Context and Background
(Specify the types of documentation to be covered, e.g., CRQs, SOPs, READMEs, inline comments. Outline key areas to standardize, such as naming conventions, formatting, and content requirements.)

# 3. Desired Output
Provide the content for the new SOP document (e.g., `docs/sops/SOP-DocumentationStandards.md`).

# 4. Constraints and Guidelines
- Ensure the SOP is clear, concise, and actionable.
- Cover all relevant documentation types.

# 5. Verification (Optional)
N/A
```

**9. Task: Define and Structure Change Request (CRQ) Documents**
*   **Pattern to Reuse:** Using CRQs for granular task breakdown and tracking.
```
# 1. Task Description
Define and structure a series of Change Request (CRQ) documents for [specific project phase or feature].

# 2. Context and Background
(Specify the project phase or feature to be broken down into CRQs. Provide any high-level requirements or existing plans.)

# 3. Desired Output
Provide the content for each CRQ document, following the established CRQ template (e.g., Objective, Scope, Prerequisites, Procedure, Verification, Tools Used).

# 4. Constraints and Guidelines
- Ensure each CRQ is granular and focuses on a single, achievable objective.
- Follow the project's CRQ naming conventions.

# 5. Verification (Optional)
N/A
```

---

#### **Theme: Project & Crate Management**

**10. Task: Initialize a New Project/Crate with a Comprehensive Plan and Documentation**
*   **Pattern to Reuse:** Structured project initiation, including planning, CRQ definition, and documentation standards.
```
# 1. Task Description
Initialize a new project/crate named `[project_name]` with a comprehensive development plan and initial documentation.

# 2. Context and Background
(Specify the purpose of the new project, its location, and any high-level goals. Outline the desired structure for the plan and documentation.)

# 3. Desired Output
Provide:
- A multi-phase development plan (e.g., in a Markdown file).
- A series of initial Change Request (CRQ) documents outlining granular steps.
- A Standard Operating Procedure (SOP) for documentation standards.
- A high-level `development_documentation.md` overview.
- Updates to the top-level `README.md` and `GEMINI.md` to introduce the new project.

# 4. Constraints and Guidelines
- Ensure consistency in naming conventions and document structure.
- Align with project goals and architectural principles.

# 5. Verification (Optional)
- Verify all new files are created in the correct locations.
- Ensure all documentation links are functional.
```

**11. Task: Refactor a Crate for Improved Modularity (applying "one function per file")**
*   **Pattern to Reuse:** Applying extreme modularity principles (e.g., one function per file) to a Rust crate.
```
# 1. Task Description
Refactor the `[crate_name]` crate to improve modularity, applying principles like "one function per file per basic block."

# 2. Context and Background
The target crate is `crates/[crate_name]`.
(Specify any particular areas within the crate that need attention or any specific functions to start with.)

# 3. Desired Output
Provide a plan for the refactoring, including proposed new file structures and a list of functions to be moved/split.
(Alternatively, if direct action is desired, specify the `write_file`, `replace`, and `run_shell_command` for `mkdir` and `mv` operations.)

# 4. Constraints and Guidelines
- Maintain compilation success throughout the refactoring process.
- Ensure no functional regressions.
- Adhere to Rust coding standards.

# 5. Verification (Optional)
- Run `cargo build -p [crate_name]`.
- Run `cargo test -p [crate_name]`.
```

**12. Task: Separate a Specific Functionality into a New, Independent Binary/Crate**
*   **Pattern to Reuse:** Decomposing a monolithic application or library into smaller, specialized components.
```
# 1. Task Description
Separate the `[functionality_name]` from `[source_crate]` into a new, independent binary/crate named `[new_crate_name]`.

# 2. Context and Background
(Describe the functionality to be separated, its current location, and the rationale for separation. Specify the desired output structure for the new crate.)

# 3. Desired Output
Provide the necessary code changes, including new `Cargo.toml` files, `src/main.rs` (if a binary), and modifications to the original crate to remove the functionality.

# 4. Constraints and Guidelines
- Ensure the separated functionality works independently.
- Maintain clear interfaces between the old and new components.
- Update any relevant build scripts or documentation.

# 5. Verification (Optional)
- Run `cargo build -p [new_crate_name]`.
- Run `cargo test -p [new_crate_name]`.
- Verify the original crate still compiles and functions correctly without the separated part.
```

**13. Task: Add Initial Crate for a New Project**
*   **Pattern to Reuse:** Basic project setup for a new Rust crate.
```
# 1. Task Description
Add an initial crate for a new project named `[project_name]`.

# 2. Context and Background
(Specify the purpose of the new crate and its desired location within the workspace.)

# 3. Desired Output
Provide the `cargo new` command and any initial `Cargo.toml` or `src/lib.rs` content.

# 4. Constraints and Guidelines
- Ensure the crate compiles successfully.

# 5. Verification (Optional)
- Run `cargo build -p [project_name]`.
```

---

#### **Theme: Build System & Tooling**

**14. Task: Port a Specific Build Orchestration Component from `x.py` to `rust-bootstrap`**
*   **Pattern to Reuse:** Migrating build system logic from one language/framework to another.
```
# 1. Task Description
Port a specific build orchestration component from `x.py` to `rust-bootstrap`.

# 2. Context and Background
(Specify the `x.py` component, its location, and the target location in `rust-bootstrap`. Provide relevant code snippets from `x.py`.)
Example: "Port the `build_stage_X` function from `src/bootstrap/bootstrap.py` to `crates/rust-bootstrap/src/build_stages/stage_X.rs`."

# 3. Desired Output
Provide the Rust code for the ported component, along with any necessary updates to `Cargo.toml` or `mod.rs` files.

# 4. Constraints and Guidelines
- Ensure functional equivalence with the original `x.py` logic.
- Adhere to Rust coding standards and project architectural principles (e.g., extreme modularity).
- Handle error conditions appropriately.

# 5. Verification (Optional)
- Run `cargo build -p rust-bootstrap`.
- (Specify any relevant tests or manual verification steps.)
```

**15. Task: Formalize Migration Path using CRQs**
*   **Pattern to Reuse:** Breaking down large migration projects into formal CRQs.
```
# 1. Task Description
Formalize the migration of [component/system, e.g., "x.py functionality"] to [target, e.g., "rust-bootstrap"] by defining a set of Change Request (CRQ) documents for the critical path.

# 2. Context and Background
(Specify the scope of the migration, the source and target systems, and any known phases or key functionalities to be migrated.)

# 3. Desired Output
Provide the content for the CRQ documents, outlining the phased approach for the migration.

# 4. Constraints and Guidelines
- Ensure CRQs cover all critical aspects of the migration.
- Follow established CRQ standards.

# 5. Verification (Optional)
N/A
```

**16. Task: Analyze Existing Build Logic for Migration**
*   **Pattern to Reuse:** Thoroughly understanding legacy systems before migration.
```
# 1. Task Description
Conduct a deep dive into [legacy build system, e.g., "x.py"] to thoroughly understand its internal workings and build orchestration logic for migration to [new system, e.g., "rust-bootstrap"].

# 2. Context and Background
(Specify the files or components of the legacy system to be analyzed. Highlight areas of particular interest, such as command-line parsing, subcommand dispatch, or interaction with configuration files.)

# 3. Desired Output
Provide a detailed analysis document or set of notes outlining the key components, flow, and critical functionalities of the legacy build logic.

# 4. Constraints and Guidelines
- Focus on understanding the "why" behind the current implementation.
- Identify any platform-specific logic or external tool dependencies.

# 5. Verification (Optional)
N/A
```

**17. Task: Implement a Specific Build Subcommand (e.g., `build`, `install`)**
*   **Pattern to Reuse:** Extending CLI tools with new subcommands that encapsulate core logic.
```
# 1. Task Description
Add a `[subcommand_name]` subcommand to the `[cli_tool_name]` command-line interface (CLI) that invokes [core logic/functionality].

# 2. Context and Background
(Specify the CLI tool, the purpose of the new subcommand, and the core logic it should invoke. Provide details on desired arguments and expected behavior.)

# 3. Desired Output
Provide the necessary code changes to implement the subcommand, including updates to argument parsing (e.g., `clap`) and the integration with the core logic.

# 4. Constraints and Guidelines
- Ensure clear and informative output for the subcommand.
- Implement proper error handling.

# 5. Verification (Optional)
- Run `cargo run -p [cli_tool_name] -- [subcommand_name] --help`.
- Execute the subcommand with test cases and verify expected behavior.
```

**18. Task: Implement a Build Configuration Report**
*   **Pattern to Reuse:** Generating structured reports (e.g., Parquet) for build configurations or project state.
```
# 1. Task Description
Implement a build configuration report for `[project_name]`, storing build configuration and stage0 information in a `build_config.parquet` file.

# 2. Context and Background
(Specify the data structures that hold build configuration and stage0 information. Outline the desired schema for the Parquet file.)

# 3. Desired Output
Provide the Rust code for:
- A function to write the `BuildState` (or equivalent) to the Parquet file.
- A function to read and summarize the build configuration metrics from the Parquet file.
- Integration of these functions into the main build orchestration.

# 4. Constraints and Guidelines
- Use Apache Arrow and Parquet for data representation.
- Ensure data integrity and schema correctness.

# 5. Verification (Optional)
- Run the build process and verify the `build_config.parquet` file is created.
- Verify the summary function displays correct information.
```

**19. Task: Fix Dead Code Warnings**
*   **Pattern to Reuse:** Addressing compiler warnings, especially dead code, to maintain code quality.
```
# 1. Task Description
Resolve dead code warnings in `[crate_name]` by [specific action, e.g., "incorporating unused fields into a report" or "removing unused code"].

# 2. Context and Background
(Specify the files or modules where dead code warnings are present. Provide the warning messages if available.)

# 3. Desired Output
Provide the necessary code changes to eliminate the dead code warnings.

# 4. Constraints and Guidelines
- Ensure no functional regressions are introduced.
- Maintain code readability.

# 5. Verification (Optional)
- Run `cargo build -p [crate_name]` and confirm no dead code warnings.
```

**20. Task: Implement Comprehensive Git Data Extraction**
*   **Pattern to Reuse:** Extracting structured data from Git repositories for analysis.
```
# 1. Task Description
Implement comprehensive Git data extraction for [specific Git object type, e.g., "blobs," "trees," "tags," "refs"] within `[crate_name]`.

# 2. Context and Background
The extraction logic should reside in `crates/[crate_name]/src/git_analyzer/extractors/get_all_[object_type].rs`.
(Specify the data points to be extracted for the given object type and the desired Arrow schema.)

# 3. Desired Output
Provide the Rust code for the `get_all_[object_type]` function, which extracts data and converts it into an Arrow `RecordBatch`.

# 4. Constraints and Guidelines
- Use the `git2` crate for Git interaction.
- Convert data into Apache Arrow `RecordBatch`es.
- Implement robust error handling.

# 5. Verification (Optional)
- Run `cargo build -p [crate_name]`.
- (Specify tests for the extractor function.)
```

**21. Task: Convert Data to Apache Arrow and Write to Parquet**
*   **Pattern to Reuse:** Storing structured data in columnar formats (Arrow/Parquet) for efficient analysis.
```
# 1. Task Description
Convert [specific data type, e.g., "Git commit data"] into Apache Arrow `RecordBatch`es and persist it to Parquet files.

# 2. Context and Background
(Specify the source of the data, the desired Arrow schema, and the naming convention for the Parquet files.)

# 3. Desired Output
Provide the Rust code for the conversion and writing logic, including any necessary schema definitions.

# 4. Constraints and Guidelines
- Use the `arrow` and `parquet` crates.
- Ensure data integrity and schema correctness.

# 5. Verification (Optional)
- Verify the Parquet files are created and contain the expected data.
```

**22. Task: Enhance Command Execution to Capture Metrics**
*   **Pattern to Reuse:** Instrumenting shell command execution to collect performance or operational metrics.
```
# 1. Task Description
Enhance the `execute_shell_command` (or equivalent) function in `[crate_name]` to capture and report command execution metrics (e.g., start time, end time, duration).

# 2. Context and Background
The function is located at `[path_to_file]`.
(Specify the metrics to be captured and how they should be reported/stored.)

# 3. Desired Output
Provide the Rust code modifications to the command execution function.

# 4. Constraints and Guidelines
- Ensure minimal overhead to command execution.
- Provide clear and consistent reporting of metrics.

# 5. Verification (Optional)
- Run commands and verify that metrics are correctly captured and reported.
```

**23. Task: Define Apache Arrow Schemas for New Data Analysis Components**
*   **Pattern to Reuse:** Defining data schemas (e.g., Arrow schemas) for new data analysis components.
```
# 1. Task Description
Define Apache Arrow schemas for [specific data entities, e.g., "Git objects (commits, blobs, trees, tags, refs)"] for the `[analyzer_name]` component.

# 2. Context and Background
The schemas should be defined in `crates/[crate_name]/src/[analyzer_path]/schemas/mod.rs`.
(Specify the fields and data types required for each entity.)

# 3. Desired Output
Provide the Rust code for the Arrow schema definitions.

# 4. Constraints and Guidelines
- Ensure schemas accurately represent the data.
- Follow Arrow schema best practices.

# 5. Verification (Optional)
- Verify the schemas compile and can be used to create `RecordBatch`es.
```