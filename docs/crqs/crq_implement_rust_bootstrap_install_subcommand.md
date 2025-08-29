# CRQ: Implement rust-bootstrap install Subcommand

## Objective
Add an `install` subcommand to the `rust-bootstrap` command-line interface (CLI) that handles the deployment of the compiled Rust compiler artifacts into the specified installation prefix, replicating the functionality of `x.py install`.

## Scope
*   Extend `rust-bootstrap`'s CLI argument parsing (using `clap`) to include a new `install` subcommand.
*   Develop logic within `rust-bootstrap` to identify and copy the necessary compiled artifacts (e.g., `rustc`, `cargo`, standard library components) from the build output directory to the installation prefix.
*   Ensure that the installation process respects the `prefix` and `sysconfdir` settings from `bootstrap.toml` or command-line overrides.
*   Handle the creation of necessary directory structures at the installation target.
*   Provide clear and informative output during the installation process, including progress and confirmation messages.
*   Implement initial testing to confirm that `rust-bootstrap install` correctly places all required files.

## Impact
This CRQ completes the basic build-and-install cycle within `rust-bootstrap`, making it a more self-contained tool for managing the Rust compiler's lifecycle. It directly replaces a key functionality of `x.py`, moving closer to full migration.

## Verification Steps
*   Running `cargo run -p rust-bootstrap -- install --help` displays correct help information for the new subcommand.
*   Executing `cargo run -p rust-bootstrap -- install` (after a successful `build`) correctly copies all expected files to the installation directory.
*   The installed `rustc` and `cargo` binaries are executable from the installation prefix.
*   No errors are reported during the installation process.

## Dependencies
*   Successful completion of `crq_implement_rust_bootstrap_build_subcommand.md` (as installation typically follows a successful build).
*   Understanding of the Rust compiler's artifact structure and installation requirements.
