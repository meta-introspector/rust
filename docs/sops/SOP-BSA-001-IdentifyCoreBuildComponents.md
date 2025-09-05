# SOP-BSA-001: Identify Core Build Components

## Purpose
To pinpoint the primary scripts and configuration files that govern the `rustc` build process.

## Procedure
1.  Start with `x.py` as the entry point.
2.  Trace its execution flow to identify the main Python scripts (`bootstrap.py`) and any critical configuration files (`config.toml`, `bootstrap.toml`).
3.  Identify the primary Rust crates responsible for the build orchestration (e.g., `src/bootstrap/src/`).

## Tools
*   `read_file`: To read the content of identified files.
*   `search_file_content`: To locate specific patterns or references within files.
*   `list_directory`: To explore directory contents and verify paths.

## Output
*   A list of identified core components (files and directories) and their absolute paths.

## Verification
*   Cross-reference findings with `rustc-dev-guide` documentation (if available via `web_fetch`).
