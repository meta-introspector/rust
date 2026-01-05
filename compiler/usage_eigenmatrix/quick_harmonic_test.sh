#!/bin/bash

# Quick test run on a few crates to validate the system
echo "=== QUICK HARMONIC VALIDATION TEST ==="

# Setup
export USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data"
export HARMONIC_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/harmonic_data"
export RUSTC_VERSION="1.74.0"
export ENABLE_HARMONIC_ANALYSIS=1

mkdir -p "$USAGE_OUTPUT_DIR" "$HARMONIC_OUTPUT_DIR"

# Build harmonic collector
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix
echo "Building harmonic tools..."
cargo build --bin version_stable_compiler --bin practical_harmonic_compiler

cd ../introspector-collector
echo "Building usage collector..."
cargo build --bin working_usage_collector

export RUSTC="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector"

# Test on a few small crates first
TEST_CRATES=("serde" "log" "anyhow")

TEMP_DIR="/tmp/harmonic_test"
rm -rf "$TEMP_DIR"
mkdir -p "$TEMP_DIR"
cd "$TEMP_DIR"

# Initialize test workspace
cat > Cargo.toml << 'EOF'
[workspace]
members = ["test_project"]
resolver = "2"
EOF

cargo new test_project --lib
cd test_project

echo "=== TESTING ON ${#TEST_CRATES[@]} CRATES ==="

for crate_name in "${TEST_CRATES[@]}"; do
    echo "--- Testing $crate_name ---"
    
    # Add dependency
    cargo add "$crate_name" 2>/dev/null || {
        echo "⚠ Could not add $crate_name, skipping..."
        continue
    }
    
    # Create usage code
    cat >> src/lib.rs << EOF

#[allow(unused_imports)]
use $crate_name::*;

pub fn test_$crate_name() {
    // Trigger harmonic analysis for $crate_name
}
EOF
    
    echo "  Running harmonic analysis on $crate_name..."
    timeout 30s cargo check 2>/dev/null && echo "  ✓ Success" || echo "  ⚠ Issue"
done

echo "=== VALIDATION RESULTS ==="

cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix

# Quick validation
usage_count=$(ls "$USAGE_OUTPUT_DIR"/*.json 2>/dev/null | wc -l)
echo "Usage files generated: $usage_count"

if [ "$usage_count" -gt 0 ]; then
    echo "✓ Harmonic collector is working!"
    echo "Sample files:"
    ls "$USAGE_OUTPUT_DIR"/*.json 2>/dev/null | head -3
    
    # Show sample content
    if [ -f "$USAGE_OUTPUT_DIR"/*.json ]; then
        sample_file=$(ls "$USAGE_OUTPUT_DIR"/*.json | head -1)
        echo "Sample content from $sample_file:"
        head -10 "$sample_file"
    fi
else
    echo "❌ No usage files generated - check collector setup"
fi

# Cleanup
rm -rf "$TEMP_DIR"

echo "✓ Quick validation complete!"
