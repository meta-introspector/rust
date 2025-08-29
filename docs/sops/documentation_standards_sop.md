# Standard Operating Procedure (SOP): Documentation Standards

## 1. Objective
To establish consistent and clear standards for all project documentation, including Change Requests (CRQs), Standard Operating Procedures (SOPs), and other relevant files. This SOP aims to improve readability, maintainability, and discoverability of documentation, ensuring a unified and professional presentation.

## 2. Scope
This SOP applies to all new and existing documentation within the project, including but not limited to:
*   Change Request (CRQ) documents
*   Standard Operating Procedure (SOP) documents
*   `README.md` files
*   Architectural design documents
*   Troubleshooting guides

## 3. Naming Conventions

### 3.1. General Principles
*   All documentation filenames shall be in `kebab-case` (lowercase, words separated by hyphens).
*   Filenames should be concise yet descriptive, clearly indicating the content.
*   Avoid special characters, spaces, or excessive abbreviations.

### 3.2. Change Request (CRQ) Documents
*   **Format:** `CRQ-NNN-ShortDescriptiveName.md`
*   **`NNN`:** A three-digit sequential number, starting from `001`. Numbers are assigned chronologically based on creation or logical dependency.
*   **`ShortDescriptiveName`:** A concise, `PascalCase` (or `UpperCamelCase`) name that summarizes the CRQ's primary objective or feature. Avoid acronyms unless universally understood within the project.
*   **Example:** `CRQ-001-BootstrapInitialSetup.md`, `CRQ-002-BootstrapStage0GitAnalysis.md`

### 3.3. Standard Operating Procedure (SOP) Documents
*   **Format:** `SOP-ShortDescriptiveName.md`
*   **`ShortDescriptiveName`:** A concise, `PascalCase` name that summarizes the SOP's primary function or area of focus.
*   **Example:** `SOP-DocumentationStandards.md`, `SOP-ExtremeModularityRefactoring.md`

### 3.4. Other Documentation Files
*   **Format:** `topic-descriptive-name.md`
*   **Example:** `troubleshooting-guide.md`, `architecture-overview.md`

## 4. Document Structure

### 4.1. General Structure for CRQs and SOPs
All CRQ and SOP documents shall follow a consistent structure to ensure clarity and completeness:

*   **Title:** H1 heading (`#`) with a clear, concise title.
*   **Objective:** H2 heading (`##`) followed by a brief statement of the document's purpose.
*   **Scope:** H2 heading (`##`) followed by a description of what the document covers and its boundaries.
*   **Prerequisites (for CRQs):** H2 heading (`##`) followed by a list of dependencies or conditions that must be met before starting the CRQ.
*   **Procedure:** H2 heading (`##`) followed by numbered steps. Each step should be clear, actionable, and include:
    *   A brief description of the action.
    *   Any relevant sub-steps.
    *   Mention of tools to be used (e.g., `run_shell_command`, `write_file`).
*   **Verification (for CRQs):** H2 heading (`##`) followed by steps to confirm successful completion of the CRQ.
*   **Tools Used:** H2 heading (`##`) followed by a list of specific tools mentioned in the procedure.

### 4.2. `README.md` Files
`README.md` files should provide a high-level overview of the directory or project they reside in. They should include:
*   Project/Directory Name
*   Brief Description
*   Purpose/Goal
*   Key Features
*   Setup/Installation Instructions (if applicable)
*   Usage Examples (if applicable)
*   Links to more detailed documentation (e.g., CRQs, SOPs).

## 5. Content Guidelines
*   **Clarity and Conciseness:** Use clear, unambiguous language. Avoid jargon where simpler terms suffice.
*   **Accuracy:** Ensure all information is factually correct and up-to-date.
*   **Consistency:** Maintain consistent terminology, formatting, and style throughout all documents.
*   **Markdown Formatting:** Use GitHub-flavored Markdown for all documentation.
*   **Code Examples:** Enclose code snippets in appropriate Markdown code blocks with language highlighting.
*   **Cross-referencing:** Use relative links to other documents within the project where appropriate.

## 6. Review and Maintenance
*   All new documentation should be reviewed by at least one other team member before being finalized.
*   Documentation should be regularly reviewed and updated to reflect changes in the codebase or project procedures.
