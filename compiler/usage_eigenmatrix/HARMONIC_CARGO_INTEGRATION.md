# Harmonic Cargo Integration

A drop-in replacement for `cargo build` that generates harmonic analysis data alongside your regular build artifacts.

## Quick Start

```bash
# Replace your normal cargo build with:
./harmonic_cargo_build.sh build

# Or for other cargo commands:
./harmonic_cargo_build.sh test
./harmonic_cargo_build.sh check
./harmonic_cargo_build.sh build --release
```

## What It Does

1. **Builds your project normally** using cargo
2. **Collects harmonic data** during compilation using our custom rustc
3. **Generates analysis files** in `target/harmonic/`
4. **Creates stable IDs** that won't change across rustc versions

## Output Structure

```
target/
├── debug/           # Normal cargo output
├── release/         # Normal cargo output  
└── harmonic/        # Harmonic analysis data
    ├── usage/       # Raw usage data (like .rlib files)
    └── analysis/    # Processed harmonic analysis
        ├── perfect_hash_mapping.json
        ├── version_stability_report.txt
        ├── practical_harmonic_report.txt
        └── build_summary.json
```

## Integration with Existing Builds

### Replace cargo in CI/CD:
```yaml
# .github/workflows/build.yml
- name: Build with harmonic analysis
  run: ./harmonic_cargo_build.sh build --release
```

### Use as cargo alias:
```toml
# .cargo/config.toml
[alias]
harmonic = "!./harmonic_cargo_build.sh"
```

Then use: `cargo harmonic build`

### Environment Variables

- `CARGO_TARGET_DIR`: Where to store build artifacts (default: `target`)
- `RUSTC_VERSION`: Version for stability tracking (default: `1.74.0`)
- `ENABLE_HARMONIC_ANALYSIS`: Enable/disable analysis (default: `1`)

## What You Get

### Stable IDs
- **Version-stable addresses** for all Rust constructs
- **Perfect hash mapping** with collision resolution
- **Semantic clustering** of similar code patterns

### Harmonic Analysis
- **Musical intervals** between code constructs
- **Mathematical relationships** in your codebase
- **LMFDB orbit classifications** for enum patterns

### Usage Data
- **Complete construct usage** across your codebase
- **Enum classification** with usage patterns
- **Cross-crate dependency analysis**

## Performance

- **Minimal overhead**: Builds run at normal speed
- **Parallel analysis**: Harmonic processing happens after build
- **Incremental**: Only analyzes changed files
- **Storage efficient**: Data stored alongside .rlib files

## Example Output

```json
{
  "build_timestamp": "2026-01-05T14:34:19-05:00",
  "cargo_command": "cargo build --release",
  "target_directory": "target",
  "harmonic_directory": "target/harmonic", 
  "usage_files_collected": 1247,
  "rustc_version": "1.74.0",
  "analysis_files": [
    "perfect_hash_report.txt",
    "version_stability_report.txt",
    "practical_harmonic_report.txt"
  ]
}
```

## Requirements

- Rust toolchain
- Our harmonic collector (built automatically)
- ~10MB extra storage per crate for harmonic data

## Production Ready

✓ **Version stable** - IDs won't change across rustc updates  
✓ **CI/CD friendly** - Drop-in cargo replacement  
✓ **Storage efficient** - Data stored in target/ like .rlib files  
✓ **Incremental** - Only processes changed code  
✓ **Parallel** - Analysis runs after successful build
