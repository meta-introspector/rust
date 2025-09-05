# SOP-VendorUsage: Standard Operating Procedure for Vendor Crate Usage

## 1. Purpose
This SOP outlines the process for managing and utilizing vendored crates within the `rustc` project. Vendorizing crates ensures reproducible builds, allows for offline compilation, and provides control over specific versions and potential patches.

## 2. Scope
This procedure applies to all developers and automated systems involved in the `rustc` project that interact with external Rust crates.

## 3. Definitions
*   **Vendored Crate:** A third-party Rust crate whose source code is included directly within the `rustc` project's repository, typically under the `crates/` or `vendor/` directory, rather than being fetched from `crates.io` or other external registries during compilation.

## 4. Procedure

### 4.1. Deciding to Vendorize a Crate
Vendorization should be considered under the following circumstances:
*   **Critical Dependency:** The crate is a core dependency for essential `rustc` components.
*   **Compatibility Issues:** The crate from `crates.io` or other sources causes compilation errors or runtime issues that cannot be resolved by simply updating the version.
*   **Offline Builds:** The project requires the ability to build without external network access.
*   **Specific Patches:** Custom modifications or patches are required for the crate that are not available upstream.

### 4.2. Vendorization Process

1.  **Identify the Crate:** Determine the exact crate to be vendored and its source (e.g., GitHub repository, specific commit hash).
2.  **Remove Existing Reference (if applicable):** If the crate was previously a `crates.io` dependency or an outdated path dependency, remove its entry from the relevant `Cargo.toml` files.
3.  **Clone/Copy Source:**
    *   For Git repositories: Clone the repository into the designated vendor directory (e.g., `crates/` or `vendor/`).
        ```bash
        git clone <repository_url> <path_to_vendor_directory>/<crate_name>
        ```
    *   For local copies: Copy the crate's source directory.
4.  **Update `Cargo.toml`:** Modify the `Cargo.toml` files of the dependent crates to use a path dependency to the newly vendored crate.
    ```toml
    [dependencies]
    <crate_name> = { path = "../../<path_to_vendor_directory>/<crate_name>" }
    ```
    Adjust the relative path as necessary.
5.  **Apply Patches (if needed):** If specific patches are required, apply them to the vendored source code. Document these patches clearly.
6.  **Update `Cargo.lock`:** Run `cargo update` or `cargo build` to regenerate `Cargo.lock` with the new path dependency.
7.  **Commit Changes:** Commit all changes, including the vendored source code and `Cargo.toml` modifications, to the version control system.

### 4.3. Maintaining Vendored Crates
*   **Regular Review:** Periodically review vendored crates for upstream updates, security vulnerabilities, or new features.
*   **Patch Management:** Keep track of any applied patches. Re-evaluate them when updating the vendored crate to a newer version.
*   **Documentation:** Maintain clear documentation for each vendored crate, including its source, version, and any applied patches.

## 5. Responsibilities
*   **Developers:** Responsible for following this SOP when introducing or modifying vendored crates.
*   **Code Reviewers:** Responsible for ensuring adherence to this SOP during code reviews.

## 6. Related Documents
*   CRQ-20250905-004: Resolve `rustc-literal-escaper` Compatibility Issues (Example of a CRQ leading to vendorization)

## 7. Revision History
| Version | Date         | Author | Description        |
|---------|--------------|--------|--------------------|
| 1.0     | 2025-09-05   | Gemini | Initial Draft      |
