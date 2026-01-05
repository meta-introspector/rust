#!/bin/bash

# Monster target harmonic cargo build - stores data in custom target directory
echo "=== MONSTER TARGET HARMONIC BUILD ==="

# Setup monster target environment
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}"
export MONSTER_TARGET_DIR="$CARGO_TARGET_DIR/monster"
export HARMONIC_DATA_DIR="$MONSTER_TARGET_DIR/harmonic"
export USAGE_OUTPUT_DIR="$HARMONIC_DATA_DIR/usage"
export HARMONIC_OUTPUT_DIR="$HARMONIC_DATA_DIR/analysis"
export RUSTC_VERSION="1.74.0"
export ENABLE_HARMONIC_ANALYSIS=1

# Create monster target directories
mkdir -p "$MONSTER_TARGET_DIR"
mkdir -p "$HARMONIC_DATA_DIR"
mkdir -p "$USAGE_OUTPUT_DIR" 
mkdir -p "$HARMONIC_OUTPUT_DIR"

echo "Monster target directory: $MONSTER_TARGET_DIR"
echo "Harmonic data directory: $HARMONIC_DATA_DIR"

# Copy monster target spec to project
MONSTER_TARGET_SPEC="$(pwd)/monster-target.json"
if [ ! -f "$MONSTER_TARGET_SPEC" ]; then
    # Create monster target spec
    cat > "$MONSTER_TARGET_SPEC" << 'EOF'
{
  "llvm-target": "x86_64-unknown-linux-gnu",
  "target-endian": "little",
  "target-pointer-width": 64,
  "target-c-int-width": 32,
  "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
  "arch": "x86_64",
  "os": "linux",
  "env": "gnu",
  "vendor": "monster",
  "linker-flavor": "gcc",
  "pre-link-args": {
    "gcc": ["-m64"]
  },
  "cpu": "x86-64",
  "features": "+cx8,+fxsr,+mmx,+sse,+sse2,+x87",
  "dynamic-linking": true,
  "executables": true,
  "relro-level": "partial",
  "has-rpath": true,
  "position-independent-executables": true,
  "tls-model": "global-dynamic",
  "panic-strategy": "unwind",
  "crt-static-allows-dylibs": true,
  "crt-static-default": false,
  "crt-static-respected": true,
  "stack-probes": {"kind": "inline-or-call", "min-llvm-version-for-inline": [16, 0, 0]},
  "supported-sanitizers": [
    "address",
    "cfi", 
    "leak",
    "memory",
    "thread"
  ]
}
EOF
    echo "✓ Created monster target spec: $MONSTER_TARGET_SPEC"
fi

# Build harmonic collector
EIGENMATRIX_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix"
COLLECTOR_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/introspector-collector"

echo "--- Building harmonic tools ---"
cd "$EIGENMATRIX_DIR"
cargo build --release --bin version_stable_compiler --bin practical_harmonic_compiler --bin perfect_hash_stable_ids

echo "--- Building usage collector ---"
cd "$COLLECTOR_DIR"
cargo build --release --bin working_usage_collector

# Set up harmonic rustc replacement
export HARMONIC_RUSTC="$COLLECTOR_DIR/target/release/working_usage_collector"

if [ ! -f "$HARMONIC_RUSTC" ]; then
    # Fallback to debug build
    export HARMONIC_RUSTC="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector"
fi

if [ ! -f "$HARMONIC_RUSTC" ]; then
    echo "❌ Harmonic collector not found"
    exit 1
fi

echo "✓ Using harmonic rustc: $HARMONIC_RUSTC"

# Return to original directory
cd - > /dev/null

# Function to run cargo with monster target
run_monster_cargo() {
    local cargo_args="$@"
    
    echo "--- Running cargo with monster target ---"
    echo "Command: cargo $cargo_args --target monster-target.json"
    echo "Target dir: $CARGO_TARGET_DIR"
    echo "Monster dir: $MONSTER_TARGET_DIR"
    echo "Harmonic dir: $HARMONIC_DATA_DIR"
    
    # Run cargo with monster target and harmonic rustc
    RUSTC="$HARMONIC_RUSTC" \
    USAGE_OUTPUT_DIR="$USAGE_OUTPUT_DIR" \
    HARMONIC_OUTPUT_DIR="$HARMONIC_OUTPUT_DIR" \
    RUSTC_VERSION="$RUSTC_VERSION" \
    ENABLE_HARMONIC_ANALYSIS="$ENABLE_HARMONIC_ANALYSIS" \
    cargo "$@" --target monster-target.json
    
    local cargo_exit_code=$?
    
    # Generate harmonic analysis after build
    if [ $cargo_exit_code -eq 0 ]; then
        echo "--- Generating monster harmonic analysis ---"
        
        # Count collected files
        usage_files=$(find "$USAGE_OUTPUT_DIR" -name "*.json" 2>/dev/null | wc -l)
        echo "Usage files collected: $usage_files"
        
        if [ "$usage_files" -gt 0 ]; then
            # Run perfect hash analysis
            cd "$EIGENMATRIX_DIR"
            echo "Running perfect hash analysis..."
            USAGE_OUTPUT_DIR="$USAGE_OUTPUT_DIR" HARMONIC_OUTPUT_DIR="$HARMONIC_OUTPUT_DIR" \
            ./target/release/perfect_hash_stable_ids > "$HARMONIC_OUTPUT_DIR/perfect_hash_report.txt" 2>&1
            
            # Run version stability analysis  
            echo "Running version stability analysis..."
            ./target/release/version_stable_compiler > "$HARMONIC_OUTPUT_DIR/version_stability_report.txt" 2>&1
            
            # Run practical harmonic analysis
            echo "Running practical harmonic analysis..."
            ./target/release/practical_harmonic_compiler > "$HARMONIC_OUTPUT_DIR/practical_harmonic_report.txt" 2>&1
            
            # Generate monster build summary
            cat > "$HARMONIC_OUTPUT_DIR/monster_build_summary.json" << EOF
{
  "build_timestamp": "$(date -Iseconds)",
  "cargo_command": "cargo $cargo_args --target monster-target.json",
  "target_directory": "$CARGO_TARGET_DIR",
  "monster_directory": "$MONSTER_TARGET_DIR", 
  "harmonic_directory": "$HARMONIC_DATA_DIR",
  "usage_files_collected": $usage_files,
  "rustc_version": "$RUSTC_VERSION",
  "harmonic_rustc": "$HARMONIC_RUSTC",
  "target_spec": "monster-target.json",
  "analysis_files": [
    "perfect_hash_report.txt",
    "version_stability_report.txt", 
    "practical_harmonic_report.txt"
  ],
  "monster_features": {
    "custom_target": true,
    "harmonic_analysis": true,
    "perfect_hash_ids": true,
    "version_stable": true,
    "data_location": "target/monster/harmonic/"
  }
}
EOF
            
            # Create monster manifest
            cat > "$MONSTER_TARGET_DIR/MONSTER_MANIFEST.md" << 'EOF'
# Monster Target Build

This directory contains build artifacts for the **monster** target, a custom Rust target that includes harmonic analysis data alongside regular compilation artifacts.

## Structure

```
target/monster/
├── MONSTER_MANIFEST.md     # This file
├── debug/                  # Debug build artifacts  
├── release/                # Release build artifacts
└── harmonic/               # Harmonic analysis data
    ├── usage/              # Raw usage data (like .rlib files)
    └── analysis/           # Processed harmonic analysis
        ├── perfect_hash_mapping.json
        ├── version_stability_report.txt
        ├── practical_harmonic_report.txt
        └── monster_build_summary.json
```

## What is Monster Target?

The monster target is identical to your native target but with enhanced data collection:

- **Version-stable IDs** for all Rust constructs
- **Perfect hash mapping** with collision resolution  
- **Harmonic analysis** of code relationships
- **Musical interval detection** in code patterns
- **LMFDB orbit classification** for enums

## Usage

```bash
# Build with monster target
cargo build --target monster-target.json

# Or use the monster build script
./monster_cargo_build.sh build
```

## Data Files

- `*.json` files contain structured analysis data
- `*.txt` files contain human-readable reports
- All data is stored alongside regular build artifacts
- Compatible with existing Rust tooling

Generated by Monster Target Harmonic Compiler
EOF
            
            echo "✓ Monster harmonic analysis complete"
            echo "✓ Results stored in: $HARMONIC_OUTPUT_DIR"
        else
            echo "⚠ No usage files collected - check harmonic rustc setup"
        fi
    else
        echo "❌ Monster cargo build failed with exit code: $cargo_exit_code"
    fi
    
    return $cargo_exit_code
}

# Main execution
if [ $# -eq 0 ]; then
    # Default: build current project with monster target
    run_monster_cargo build
else
    # Run with provided arguments
    run_monster_cargo "$@"
fi

# Show monster results
echo ""
echo "=== MONSTER BUILD RESULTS ==="
echo "Target directory: $CARGO_TARGET_DIR"
echo "Monster directory: $MONSTER_TARGET_DIR"
echo "Harmonic data: $HARMONIC_DATA_DIR"

if [ -d "$HARMONIC_DATA_DIR" ]; then
    echo "Monster harmonic files generated:"
    find "$HARMONIC_DATA_DIR" -name "*.json" -o -name "*.txt" | head -10
    
    total_files=$(find "$HARMONIC_DATA_DIR" -type f | wc -l)
    echo "Total monster files: $total_files"
    
    if [ -f "$HARMONIC_OUTPUT_DIR/monster_build_summary.json" ]; then
        echo ""
        echo "Monster build summary:"
        cat "$HARMONIC_OUTPUT_DIR/monster_build_summary.json"
    fi
    
    if [ -f "$MONSTER_TARGET_DIR/MONSTER_MANIFEST.md" ]; then
        echo ""
        echo "✓ Monster manifest created: $MONSTER_TARGET_DIR/MONSTER_MANIFEST.md"
    fi
else
    echo "❌ No monster harmonic data generated"
fi

echo ""
echo "🐉 Monster target build complete!"
echo "📁 Data location: $MONSTER_TARGET_DIR"
