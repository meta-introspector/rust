# Chunked Enhanced Data

This directory has been moved to the mycelial-usage-data dataset to avoid large files in the compiler repository.

## Location
The chunked enhanced data is now available at:
`/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/compiler_tools_data/`

## Files Moved
- All chunked files (`enhanced_*_chunk_*.json`)
- Manifest files for reconstruction
- Single enhanced files under 2MB

## Usage
Update your orbit analyzer and other tools to point to the new location:
```rust
let data_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/compiler_tools_data";
```
