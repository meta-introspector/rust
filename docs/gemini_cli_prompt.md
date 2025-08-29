**Detailed Prompt for Gemini CLI: Rust Compiler Deep Introspection Project**

**Project Overview:**

We are working on a highly ambitious project to build the Rust compiler from scratch on Termux (Android environment) and integrate deep introspection capabilities using Apache Arrow, Apache Parquet, and Hugging Face datasets. The ultimate goal is to create a self-sampling Rust compiler that records its own bootstrap process into structured, layered datasets for formal verification, analysis, and machine learning applications.

**Current State & Goals:**

1.  **Rust Compiler Build:**
    *   We are attempting to build the Rust compiler from its source code.
    *   The standard `x.py` bootstrap script has been problematic due to its reliance on downloading pre-built `stage0` artifacts, which are often unavailable for the `aarch64-linux-android` target or become outdated.
    *   We have a working older `rustc` and `cargo` installed on Termux at `/data/data/com.termux/files/usr/bin/rustc` and `/data/data/com.termux/files/usr/bin/cargo` respectively.
    *   **Immediate Goal:** Successfully build the Rust compiler using our local `rustc` and `cargo` as the `stage0` bootstrap, bypassing the download mechanism.

2.  **`rust-bootstrap` Crate Development:**
    *   We are developing a new Rust-native bootstrap tool, `rust-bootstrap` (located at `crates/rust-bootstrap`), to replace the Python-based `x.py`.
    *   **Current Progress:**
        *   The `rust-bootstrap` crate is set up with `clap` for argument parsing, `serde` and `toml` for `config.toml` parsing, and environment variable handling (with Termux defaults for `RUSTC` and `CARGO`).
        *   It currently attempts to execute `cargo --version` via `sh -c` and capture its output.
        *   It is intended to write build metrics to a Parquet file (`build_metrics.parquet`).
    *   **Persistent Issue:** The `rust-bootstrap` program consistently exits cleanly (Exit Code 0) after executing the external command (`cargo --version`), but *before* executing any subsequent Rust code (e.g., printing trace statements or writing the Parquet file). This occurs even when the external command execution is simplified or removed, suggesting a subtle environmental or process-level interaction in Termux.
    *   **Immediate Goal:** Resolve the silent exit issue in `rust-bootstrap` to allow the program to execute fully, including the Parquet file writing.

3.  **Hugging Face Export & Deep Introspection:**
    *   The long-term vision is to make this version of Rust an "introspector" that samples every part of its bootstrap into layered Hugging Face datasets.
    *   We are using Apache Arrow and Apache Parquet for data sampling and storage.
    *   **Crucial Detail:** We are *not* using `crates.io` versions of `arrow` and `parquet`. Instead, we are using vendored forks located within `~/storage/github/mcp/monomcp/vendor/arrow-rs/`.
        *   `arrow` crate path: `../../../mcp/monomcp/vendor/arrow-rs/arrow`
        *   `parquet` crate path: `../../../mcp/monomcp/vendor/arrow-rs/parquet` (with `arrow` feature enabled)
    *   **Immediate Goal:** Ensure `rust-bootstrap` can successfully write data to Parquet files using these vendored libraries.

**Constraints & Preferences:**

*   **Environment:** Termux (Android, `aarch64-linux-android`). Be mindful of potential resource limits or unique Termux behaviors.
*   **Git:**
    *   We have successfully rebased our local `master` branch onto `upstream/master` (from `rust-lang/rust`).
    *   Our `rust-bootstrap` crate and related documentation (SOP, CRQ) are committed to the `master` branch.
    *   Avoid complex Git operations unless explicitly requested and necessary.
*   **Rust-Native:** Prioritize Rust-native solutions over Python scripts where feasible.
*   **Debugging:** When encountering issues, prioritize systematic debugging (e.g., trace statements, isolating components) to pinpoint the root cause.
*   **Documentation:** Maintain clear and concise documentation (SOPs, CRQs, code comments).

**Specific Tasks for Gemini CLI:**

1.  **Diagnose and Fix Silent Exit in `rust-bootstrap`:**
    *   Investigate why `rust-bootstrap` exits cleanly but prematurely after external command execution (or even when simulated).
    *   Propose and implement a solution that allows the program to execute fully, including the Parquet file writing. This might involve alternative ways to execute commands or handle process lifecycle in Termux.

2.  **Implement Parquet Data Writing:**
    *   Once the silent exit is resolved, ensure the `rust-bootstrap` successfully writes the `build_metrics.parquet` file with the captured `cargo --version` data.
    *   Verify the file's existence and basic integrity.

3.  **Prepare for Full Rust Compiler Bootstrap:**
    *   Once `rust-bootstrap` is stable and can write Parquet files, the next major step will be to integrate it with the actual Rust compiler build process. This will involve:
        *   Understanding how `x.py` orchestrates the multi-stage Rust build.
        *   Translating those orchestration steps into Rust code within `rust-bootstrap`.
        *   Using the detected `stage0` compiler to build `stage1`, `stage2`, etc.

**Communication Guidelines:**

*   Be concise and direct.
*   Explain proposed actions before execution, especially for file modifications or shell commands.
*   Prioritize user safety and project conventions.
