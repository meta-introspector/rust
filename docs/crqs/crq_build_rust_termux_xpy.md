# CRQ: Build Nightly Rust Compiler on Termux (using x.py)

## Objective
Successfully build and install the nightly Rust compiler from source on Termux using the existing `x.py` build system.

## Scope
*   Install all necessary Termux packages and dependencies as outlined in the provided guide (git, clang, cmake, make, python, wget, tar, ndk-multilib).
*   Clone the official Rust repository from `https://github.com/rust-lang/rust.git`.
*   Configure the build by copying `config.example.toml` to `config.toml` and editing it to target `aarch64-linux-android` (or appropriate architecture) and set `cargo`, `rustc`, `prefix`, and `channel` paths as specified in the guide.
*   Execute `./x.py build` from the Rust source directory to compile the Rust compiler.
*   Execute `./x.py install` to install the compiled artifacts into the Termux environment.
*   Verify the installation by checking `rustc --version` and `cargo --version`.

## Impact
Provides a functional, self-built nightly Rust toolchain on Termux, serving as a foundational environment for further development and testing of `rust-bootstrap` and other Rust-related projects.

## Verification Steps
*   All `pkg install` commands complete successfully.
*   Rust repository is cloned into `/data/data/com.termux/files/home/storage/github/rust` and `cd rust` is successful.
*   `config.toml` is correctly configured with the specified paths and targets.
*   `./x.py build` completes without any compilation or linking errors.
*   `./x.py install` completes without errors.
*   Running `rustc --version` and `cargo --version` from any directory in Termux outputs the expected nightly versions and build information.

## Dependencies
*   A device running Android 5.0 or above.
*   Termux installed from F-Droid or Google Play.
*   Adequate storage space on the Android device.
*   Active internet connection for cloning repositories and fetching snapshots.
