#!/bin/bash

# Scan rustc_middle/ty across versions to track TyCtxt API evolution
RUST_BUILD_DIR="/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build"
VERSIONS=("1.82.0" "1.83.0" "1.84.0" "1.85.0" "1.85.1" "1.86.0" "1.87.0" "1.88.0" "1.89.0" "1.90.0" "1.91.0" "1.91.1")
BASE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/introspector-collector/ty_changes"

echo "=== Scanning rustc_middle/ty across versions ==="

for version in "${VERSIONS[@]}"; do
    echo "--- Processing version $version ---"
    
    cd "$RUST_BUILD_DIR"
    git checkout "$version" 2>/dev/null || {
        echo "Skipping $version - not found"
        continue
    }
    
    # Create version-specific directory
    TY_DIR="$BASE_OUTPUT_DIR/ty_data_$version"
    mkdir -p "$TY_DIR"
    
    cd compiler/rustc_middle
    
    echo "Collecting ty usage data for $version..."
    USAGE_OUTPUT_DIR="$TY_DIR" \
    CFG_RELEASE_CHANNEL=dev \
    RUSTC_INSTALL_BINDIR=/usr/bin \
    RUSTC=/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector \
    cargo build > "$BASE_OUTPUT_DIR/ty_build_$version.log" 2>&1
    
    echo "Completed $version - ty usage: $(ls $TY_DIR/*.json 2>/dev/null | wc -l) files"
done

echo "=== TyCtxt API scan complete ==="
echo "Results in: $BASE_OUTPUT_DIR"
