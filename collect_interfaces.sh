#!/bin/bash

# Collect rustc_hir interface changes across versions
cd /home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build

VERSIONS=("1.80.0" "1.81.0" "1.82.0" "1.83.0" "1.84.0" "1.85.0" "1.85.1")
OUTPUT_DIR="interface_changes"
mkdir -p "$OUTPUT_DIR"

echo "=== Collecting rustc_hir interface changes ==="

for version in "${VERSIONS[@]}"; do
    echo "--- Checking out rust-$version ---"
    
    # Checkout version
    git checkout "rust-$version" 2>/dev/null || {
        echo "Warning: Could not checkout rust-$version, trying $version"
        git checkout "$version" 2>/dev/null || {
            echo "Skipping $version - not found"
            continue
        }
    }
    
    # Build collector for this version
    echo "Building collector for $version..."
    cd compiler/introspector-collector
    cargo build --bin working_usage_collector --quiet || {
        echo "Build failed for $version, skipping"
        cd ../..
        continue
    }
    
    # Run collector on rustc_hir itself to collect interface
    echo "Collecting interface for $version..."
    ../../target/debug/working_usage_collector ../rustc_hir/src/hir.rs > "$OUTPUT_DIR/hir_$version.log" 2>&1
    
    # Copy the generated usage data
    cp -r usage_data "$OUTPUT_DIR/usage_data_$version" 2>/dev/null
    
    cd ../..
    echo "Completed $version"
done

echo "=== Interface collection complete ==="
echo "Results in: $OUTPUT_DIR"
