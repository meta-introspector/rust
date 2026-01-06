# Enhanced Usage Data

This directory has been moved to the mycelial-usage-data dataset to avoid large files in the compiler repository.

## Location
The enhanced usage data is now available at:
`/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/compiler_tools_data/`

## Files Moved
- `enhanced_rustc_metadata_literals.json`
- `enhanced_rustc_target_constants.json` 
- `enhanced_rustc_mir_transform_shim_async_destructor_ctor.json`
- `enhanced_rustc_lint_literals.json`
- All chunked data files (`enhanced_*_chunk_*.json`)
- Manifest files for reconstruction

## Usage
Update your code to point to the new location:
```rust
let data_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/compiler_tools_data";
```
