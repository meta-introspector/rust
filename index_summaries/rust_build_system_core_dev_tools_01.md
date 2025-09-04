# Rust Build System and Core Development Tools (Part 1)

This index file (`build_system_tools_01.txt`) lists `Cargo.toml` paths for essential components of the Rust project's build system and core development tools. These tools are crucial for managing the compilation process, ensuring code quality, and facilitating development workflows.

**Crates included in this section:**
*   `./src/bootstrap/Cargo.toml`: The main `Cargo.toml` for the Rust bootstrap system, responsible for orchestrating the build process of the entire Rust toolchain.
*   `./src/tools/build-manifest/Cargo.toml`: A tool likely used to generate or manage build manifests.
*   `./src/tools/bump-stage0/Cargo.toml`: A tool for managing or updating the 'stage0' compiler, which is the initial compiler used to build subsequent stages.
*   `./src/tools/cargotest/Cargo.toml`: A testing utility related to Cargo, the Rust package manager.
*   `./src/tools/clippy/Cargo.toml`: The main `Cargo.toml` for Clippy, the Rust linter, which provides a large collection of lints to catch common mistakes and improve Rust code.
*   `./src/tools/clippy/clippy_config/Cargo.toml`: Configuration-related components for Clippy.
*   `./src/tools/clippy/clippy_dev/Cargo.toml`: Development utilities for Clippy itself.
