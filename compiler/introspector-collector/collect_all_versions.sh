#!/bin/bash

# Collect usage data for all rustc versions
RUST_BUILD_DIR="/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build"
VERSIONS=("1.80.0" "1.81.0" "1.82.0" "1.83.0" "1.84.0" "1.85.0" "1.85.1" "1.86.0" "1.87.0" "1.88.0" "1.89.0" "1.90.0" "1.91.0" "1.91.1")
BASE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/introspector-collector/interface_changes"

echo "=== Collecting usage data for all versions ==="

for version in "${VERSIONS[@]}"; do
    echo "--- Processing version $version ---"
    
    # Skip if already collected
    USAGE_DIR="$BASE_OUTPUT_DIR/usage_data_$version"
    if [ -d "$USAGE_DIR" ] && [ "$(ls $USAGE_DIR/*.json 2>/dev/null | wc -l)" -gt 0 ]; then
        echo "Skipping $version - already collected ($(ls $USAGE_DIR/*.json 2>/dev/null | wc -l) files)"
        continue
    fi
    
    cd "$RUST_BUILD_DIR"
    git checkout "$version" 2>/dev/null || {
        echo "Skipping $version - not found"
        continue
    }
    
    # Create version-specific directory
    USAGE_DIR="$BASE_OUTPUT_DIR/usage_data_$version"
    mkdir -p "$USAGE_DIR"
    
    cd compiler/rustc_hir
    
    echo "Collecting usage data for $version..."
    USAGE_OUTPUT_DIR="$USAGE_DIR" \
    CFG_RELEASE_CHANNEL=dev \
    RUSTC_INSTALL_BINDIR=/usr/bin \
    RUSTC=/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector \
    cargo build > "$BASE_OUTPUT_DIR/build_$version.log" 2>&1
    
    echo "Completed $version - usage: $(ls $USAGE_DIR/*.json 2>/dev/null | wc -l) files"
done

echo "=== Collection complete ==="
echo "Results in: $BASE_OUTPUT_DIR"
