# rust-bootstrap

`rust-bootstrap` is a new, Rust-native bootstrap tool for the Rust project. It is designed to replace the existing Python-based `x.py` script, providing a more integrated and efficient build experience.

## Purpose

The primary goals of `rust-bootstrap` are:

-   **Streamlined Toolchain Management:** Automatically download and manage the necessary Rust toolchains (including nightly versions) for building the Rust compiler and its components.
-   **Git Repository Analysis:** Extract and analyze Git repository data (commits, blobs, trees, tags, refs) and store it in structured Apache Arrow and Parquet formats. This enables advanced analysis and formal verification of the Rust compiler's development process.
-   **Build Metrics Reporting:** Capture and report detailed metrics about the build process, including command execution times, to identify performance bottlenecks and improve build efficiency.
-   **Extreme Modularity:** Adhere to an "one function per file per basic block" modularity principle, enhancing maintainability, testability, and future extensibility.

## Usage

To run `rust-bootstrap`, navigate to the root of the Rust project and execute:

```bash
cargo run -p rust-bootstrap -- <REPO_PATH>
```

Replace `<REPO_PATH>` with the absolute path to the Git repository you wish to analyze (e.g., the root of the Rust project itself).

**Example:**

```bash
cargo run -p rust-bootstrap -- /data/data/com.termux/files/home/storage/github/rust
```

## Development

`rust-bootstrap` is actively under development. Contributions are welcome, adhering to the project's established coding conventions and modularity principles.
