# SOP-SIM-002: Documentation & Quality Assurance

## Purpose
To ensure all integrated crates are properly documented, adhere to quality standards, and the overall integration process is robust and traceable.

## Procedure
1.  For each successfully integrated crate:
    *   Create or update its dedicated `builds/packagename/` directory structure.
    *   Ensure `builds/packagename/Cargo.toml` correctly wraps the original crate.
    *   Create or update `builds/packagename/docs/README.md` with relevant documentation.
    *   Create or update `builds/packagename/scripts/build.sh` with the appropriate build command (e.g., `x.py build <crate_name>`).
2.  Generate or update architectural diagrams:
    *   C4 Model (Context, Container, Component, Code) to reflect the updated workspace structure and new crate integrations.
    *   UML diagrams (e.g., Component Diagrams, Activity Diagrams) to illustrate the relationships and build flow.
3.  Review the entire integration process against established quality management principles:
    *   **ISO 9000:** Ensure processes are documented, repeatable, and customer-focused (developer experience).
    *   **ITIL:** Focus on service management, change management, and problem management for the build system.
    *   **GMP (Good Manufacturing Practices):** Emphasize controlled environments, clear procedures, and traceability of changes.
    *   **Six Sigma:** Identify and reduce defects (compilation errors, build failures) in the integration process.
4.  Identify and document areas for process improvement and automation.

## Tools
*   `write_file`, `read_file`, `glob`.
*   Conceptual diagramming tools.
*   Text editors for documentation.

## Output
*   Comprehensive project documentation, including updated architectural diagrams.
*   A report detailing adherence to quality standards and identified areas for improvement.

## Verification
*   Internal audit by a "Project Lead" agent.
*   Regular reviews by the development team.
