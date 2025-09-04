# Rust Testing Infrastructure: Run-Make and Rustdoc GUI Tests (Part 1)

This index file (`testing_infrastructure_01.txt`) lists `Cargo.toml` paths for various test suites within the Rust project's testing infrastructure. These tests are crucial for ensuring the correctness and stability of the compiler, standard library, and associated tools.

**Crates included in this section:**
*   `./tests/run-make/apple-slow-tls/tls_test/Cargo.toml`: A `run-make` test specifically for TLS (Transport Layer Security) functionality on Apple platforms, potentially testing performance or specific behaviors.
*   `./tests/run-make/compiler-builtins/Cargo.toml`: A `run-make` test for the `compiler-builtins` crate, verifying the correctness of low-level compiler-generated functions.
*   `./tests/run-make/uefi-qemu/uefi_qemu_test/Cargo.toml`: A `run-make` test for UEFI (Unified Extensible Firmware Interface) applications, likely run within a QEMU (Quick Emulator) environment.
*   `./tests/run-make/rustdoc-scrape-examples-paths/foo/Cargo.toml`: A `run-make` test related to `rustdoc`'s ability to scrape example paths.
*   `./tests/run-make/x86_64-fortanix-unknown-sgx-lvi/enclave/Cargo.toml`: A `run-make` test for Intel SGX (Software Guard Extensions) enclaves on x86_64 architecture, specifically related to LVI (Load Value Injection) vulnerabilities.
*   `./tests/run-make/thumb-none-qemu/example/Cargo.toml`: A `run-make` test for ARM Thumb mode, likely for embedded systems, run within a QEMU environment.
*   `./tests/rustdoc-gui/src/extend_css/Cargo.toml`: A test for the graphical user interface of `rustdoc`, specifically testing CSS extension capabilities.
