#!/bin/bash

# Collect rustc_hir interface changes across versions
# Work in rust-build (clean repo) and checkout entire branches
# Keep our code safe in the original rust repo

RUST_BUILD_DIR="/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build"
VERSIONS=("1.80.0" "1.81.0" "1.82.0" "1.83.0" "1.84.0" "1.85.0" "1.85.1")
OUTPUT_DIR="interface_changes"

mkdir -p "$OUTPUT_DIR"
echo "=== Collecting rustc_hir interface changes ==="

for version in "${VERSIONS[@]}"; do
    echo "--- Checking out $version in rust-build ---"
    
    cd "$RUST_BUILD_DIR"
    
    # Checkout entire version (safe because this is the clean repo)
    git checkout "$version" 2>/dev/null || {
        echo "Skipping $version - not found"
        continue
    }
    
    # Use our pre-built collector binary
    COLLECTOR_BIN="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector"
    
    # Run collector on rustc_hir to collect interface
    echo "Collecting interface for $version..."
    $COLLECTOR_BIN compiler/rustc_hir/src/hir.rs > "$OUTPUT_DIR/hir_$version.log" 2>&1
    
    echo "Completed $version"
done

echo "=== Interface collection complete ==="
echo "Results in: $OUTPUT_DIR"
