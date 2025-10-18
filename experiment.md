# Nix Flake Experiment Log

## Goal
The primary goal is to successfully build the `rust-src` project (Rust compiler and standard library) within a Nix flake environment, using Python for orchestration, and to ensure efficient iteration by reusing build results.

## Current State
- We have a `rust-src` flake (main flake) that defines a `devShell` with Rust and Python, and a `packages.default` that attempts to build the Rust compiler using `python x.py build`.
- We have a `test-rust` flake (sub-flake) that references the `rust-src` flake via a Git hash (`e6c1b92d0abaa3f64032d6662cbcde980c826ff2`). Its purpose is to provide a `devShell` that extends the `rust-src` environment and to build the `rust-src`'s default package.

## Knowns
- The `rust-src` flake's `default.nix` now includes `python x.py build` in its `buildPhase`.
- `pkgs.python3` and `pkgs.curl` have been added to `nativeBuildInputs` in `rust-src/default.nix` to resolve "python: not found" and "curl: not found" errors during the `rust-src` build.
- The `rust-src` flake's `devShell` has `pkgs.python3` and `pkgs.python3Packages.pip` in its `packages` list.
- The `test-rust` flake's `devShells` are explicitly defined for `aarch64-linux` and `x86_64-linux` to avoid `flake-utils.lib.eachSystem` issues.
- The `test-rust` flake's `packages.default` is set to `rustSrcFlake.packages.<system>.default` to trigger the `rust-src` build.
- We successfully built a minimal Python "hello world" package from a separate flake, demonstrating basic Python functionality in Nix.
- We successfully verified `rustc` and `cargo` executables directly from a built Rust toolchain in the Nix store.

## Unknowns / Problems
1.  **Persistent `devShells` Resolution Error in `test-rust` flake**: When running `nix develop` on the `test-rust` flake, we consistently get `error: flake '...' does not provide attribute 'devShells.aarch64-linux.devShells.default' ... or 'devShells.default'`. This happens even after explicitly defining `devShells` and removing `flake-utils.lib.eachSystem`. This suggests a deeper issue with how Nix resolves `devShells` when a flake is referenced via `git+file://` or a subtle syntax error.
2.  **`curl` not found during `rust-src` build**: Despite adding `pkgs.curl` to `nativeBuildInputs` in `rust-src/default.nix`, the `rust-src` build (triggered by `nix build` on `test-rust`) still fails with `ERROR: unable to run `curl --version`: [Errno 2] No such file or directory: 'curl'`. This indicates that `curl` is not being correctly propagated or found in the build environment of `x.py`.
3.  **`config.toml` error**: The `rust-src` build previously failed with `configure: ERROR: Existing 'config.toml' detected. Exiting`. We added `rm -f config.toml` to `buildPhase` in `default.nix` as a workaround.

## Experiments / Next Steps
1.  **Re-evaluate `devShells` definition in `test-rust/flake.nix`**: Since the explicit definition didn't work, we need to re-examine the standard way `devShells` are defined and accessed in flakes, especially when dealing with `git+file://` references.
2.  **Investigate `curl` propagation**: We need to understand why `curl` is not available in the `x.py` build environment. This might involve:
    *   Ensuring `curl` is in the `PATH` for the `buildPhase`.
    *   Checking if `x.py` has specific requirements for `curl` (e.g., a specific version or location).
    *   Inspecting the official Nixpkgs Rust build (`rustc.nix`) for how it handles `curl` and other build tools.
3.  **Inspect official Nixpkgs Rust build**: We need to successfully read and analyze `pkgs/development/compilers/rust/rustc.nix` and `pkgs/build-support/rust/rustc-wrapper/default.nix` from the `nixpkgs` input to understand the canonical way to build Rust from source in Nix. This will provide valuable insights into how to correctly set up the build environment and dependencies.
    *   **Problem**: Cannot directly read files outside workspace.
    *   **Attempted Solution**: Use `nix eval` to get `nixpkgs.outPath` and then read from there. This failed due to `undefined variable 'self'` and `cannot call 'getFlake' on unlocked flake reference`.
    *   **Next Attempt**: Try to get the `nixpkgs` input's `outPath` using `nix path-info --json .#nixpkgs` (which was cancelled). We need to successfully get this path to read the files.
4.  **Address `config.toml` more robustly**: While `rm -f config.toml` is a workaround, understanding why `configure` complains about it might lead to a more idiomatic Nix solution.

## Current Focus
The immediate next step is to successfully inspect the official Nixpkgs Rust build files to gain insight into how they manage dependencies and the build process. This requires successfully getting the `outPath` of the `nixpkgs` input.
