# x.py to rust-bootstrap Mapping and Status

This document maps the functionality of the Python-based `x.py` and `bootstrap.py` scripts to their Rust-native counterparts in the `rust-bootstrap` crate. It also tracks the implementation status of each feature.

## 1. Argument Parsing

| `bootstrap.py` | `rust-bootstrap` | Status |
| --- | --- | --- |
| `parse_args()` using `argparse` | `Args` struct in `src/config/args.rs` using `clap` | ✅ Implemented |

## 2. Configuration Loading

| `bootstrap.py` | `rust-bootstrap` | Status |
| --- | --- | --- |
| `get_toml()` and `get_toml_static()` | `load_config()` and `Config` struct in `src/config/loader.rs` | ✅ Implemented |

## 3. Stage0 Toolchain Management

| `bootstrap.py` | `rust-bootstrap` | Status |
| --- | --- | --- |
| `download_toolchain()` | `download_and_setup_toolchain()` in `src/main_stages/download_and_setup_toolchain.rs` | ✅ Implemented |
| `program_out_of_date()` | `program_out_of_date()` in `src/bootstrap_stages/toolchain_downloader/program_out_of_date.rs` | ✅ Implemented |
| `get()` | `download_component()` in `src/bootstrap_stages/toolchain_downloader/download_component.rs` | ✅ Implemented |
| `unpack()` | `unpack_component()` in `src/bootstrap_stages/toolchain_downloader/unpack_component.rs` | ✅ Implemented |
| `should_fix_bins_and_dylibs()` | `should_fix_bins_and_dylibs()` in `src/bootstrap_stages/nix_patcher/should_fix_bins_and_dylibs.rs` | ✅ Implemented |
| `fix_bin_or_dylib()` | `fix_bin_or_dylib()` in `src/bootstrap_stages/nix_patcher/fix_bin_or_dylib.rs` | ✅ Implemented |

## 4. Building the Bootstrap Compiler

| `bootstrap.py` | `rust-bootstrap` | Status |
| --- | --- | --- |
| `build_bootstrap()` | `build_bootstrap()` in `src/main_stages/build_bootstrap/build_bootstrap.rs` | ✅ Implemented |
| `build_bootstrap_cmd()` | `build_bootstrap_cmd()` in `src/main_stages/build_bootstrap/build_bootstrap_cmd.rs` | ✅ Implemented |

## 5. Command Execution

| `bootstrap.py` | `rust-bootstrap` | Status |
| --- | --- | --- |
| `run()` | `command_executor::shell::shell()` in `src/command_executor/shell.rs` | ✅ Implemented |

## 6. Git Analysis and Parquet Reporting

| `bootstrap.py` | `rust-bootstrap` | Status |
| --- | --- | --- |
| N/A | `git_analyzer` and `parquet_reporter` modules | ✅ Implemented |

## Overall Status

The `rust-bootstrap` crate has successfully implemented the core functionality of the `bootstrap.py` script, including argument parsing, configuration loading, toolchain management, and building the bootstrap compiler. In addition, it has introduced new features for Git analysis and Parquet reporting.

The next steps are to continue migrating the remaining functionality from `bootstrap.py` and to expand the capabilities of the `rust-bootstrap` crate.
