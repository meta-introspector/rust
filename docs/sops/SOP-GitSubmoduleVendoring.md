SOP: Git Submodule Vendoring

## Objective
To standardize the process of vendoring external Git repositories as submodules within the project, ensuring consistent dependency management, traceability, and adherence to project conventions. This SOP applies to external dependencies that are managed as separate Git repositories and are intended to be included directly in the project's source tree.

## Principles
*   **No Copying/Tarring:** External dependencies must be included via `git submodule`, not by copying files or extracting tarballs.
*   **Traceability:** Submodules provide clear version control and traceability to the original source.
*   **Consistency:** All external Git dependencies will follow this standardized submodule approach.

## Procedure

1.  **Identify the External Repository:** Determine the URL of the external Git repository (e.g., `https://github.com/example/repo.git`) and the desired path within the project where it should reside (e.g., `vendor/repo_name`).

2.  **Add the Submodule:**
    *   Use the `git submodule add` command to add the external repository as a submodule.
    *   `git submodule add <repository_url> <path_within_project>`
    *   Example: `git submodule add https://github.com/example/repo.git crates/my-crate/vendor/repo_name`
    *   This command will:
        *   Create a new entry in the `.gitmodules` file at the project root.
        *   Create a new directory at `<path_within_project>` and clone the repository into it.

3.  **Initialize and Update Submodules:**
    *   After adding a new submodule or cloning a repository with submodules, initialize and update them:
    *   `git submodule update --init --recursive`
    *   This command ensures that all submodules (and their nested submodules) are cloned and checked out to the correct commit.

4.  **Configure `Cargo.toml` (if applicable):**
    *   If the vendored submodule is a Rust crate that needs to be a dependency of another crate in the project, add it as a path dependency in the dependent crate's `Cargo.toml`.
    *   Example (in `crates/my-crate/Cargo.toml`):
        ```toml
        [dependencies]
        repo_name = { path = "vendor/repo_name" }
        ```
    *   If the submodule contains multiple crates or needs specific features, configure it accordingly.
    *   For dependencies that are normally pulled from `crates.io` but are now vendored, use the `[patch.crates-io]` section in the workspace root's `Cargo.toml` to redirect Cargo to the local submodule path.
    *   Example (in top-level `Cargo.toml`):
        ```toml
        [patch.crates-io]
        some_crate = { path = "crates/my-crate/vendor/repo_name/some_crate" }
        ```

5.  **Commit Changes:**
    *   Commit the changes to `.gitmodules`, the newly added submodule directory, and any modified `Cargo.toml` files.

## Tools Used
*   `run_shell_command`: For `git submodule add`, `git submodule update`, `git submodule init`.
*   `read_file`, `write_file`, `replace`: For modifying `Cargo.toml` files.

## Best Practices & Considerations
*   **Shallow Clones:** For very large submodules, consider using `git submodule add --depth 1` for shallow clones, but be aware of limitations (e.g., cannot switch branches easily).
*   **Nested Submodules:** Always use `--recursive` when updating submodules to ensure nested submodules are also initialized.
*   **Dependency Conflicts:** Be mindful of version conflicts between the main project's dependencies and the submodule's dependencies. Use Cargo's dependency resolution features (e.g., `[patch]`, `[replace]`) as needed.
*   **Maintenance:** Regularly update submodules to pull in upstream changes.
