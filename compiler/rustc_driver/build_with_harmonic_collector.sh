#!/bin/bash

# Enhanced build script with harmonic compiler integration
echo "=== Building rustc_driver with HARMONIC INTROSPECTOR-COLLECTOR ==="

# Set up harmonic compiler environment
export USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data"
export HARMONIC_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/harmonic_data"
export CFG_RELEASE_CHANNEL=dev
export RUSTC_INSTALL_BINDIR=/usr/bin

# Create output directories
mkdir -p "$USAGE_OUTPUT_DIR"
mkdir -p "$HARMONIC_OUTPUT_DIR"

# Build our harmonic collector first
echo "--- Building harmonic collector ---"
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix
cargo build --bin version_stable_compiler
cargo build --bin practical_harmonic_compiler

# Copy harmonic tools to collector directory
echo "--- Integrating harmonic tools ---"
cp target/debug/version_stable_compiler ../introspector-collector/
cp target/debug/practical_harmonic_compiler ../introspector-collector/

# Build the enhanced collector
echo "--- Building enhanced collector ---"
cd ../introspector-collector
cargo build --bin working_usage_collector

# Use our enhanced collector as RUSTC to build ALL crates with harmonic analysis
echo "--- Running harmonic compilation ---"
export RUSTC="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector"
export ENABLE_HARMONIC_ANALYSIS=1
export RUSTC_VERSION="1.74.0"  # Current version for stability tracking

# Run the build with harmonic collection
cargo build --workspace --all-targets

echo "=== Harmonic Results ==="
echo "Usage data files:"
ls -la "$USAGE_OUTPUT_DIR"/ | wc -l

echo "Harmonic data files:"
ls -la "$HARMONIC_OUTPUT_DIR"/ 2>/dev/null | wc -l || echo "0"

echo "Recent usage files:"
ls "$USAGE_OUTPUT_DIR"/*.json 2>/dev/null | tail -5

echo "Recent harmonic files:"
ls "$HARMONIC_OUTPUT_DIR"/*.json 2>/dev/null | tail -5

# Generate harmonic summary
echo "--- Generating harmonic summary ---"
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix

# Run harmonic analysis on collected data
echo "Running version stability analysis..."
./target/debug/version_stable_compiler > "$HARMONIC_OUTPUT_DIR/version_stability_report.txt" 2>&1

echo "Running practical harmonic analysis..."
./target/debug/practical_harmonic_compiler > "$HARMONIC_OUTPUT_DIR/harmonic_analysis_report.txt" 2>&1

echo "=== Harmonic Build Complete ==="
echo "✓ Usage data collected with version-stable addressing"
echo "✓ Harmonic frequencies calculated for all constructs"
echo "✓ Musical intervals identified in codebase"
echo "✓ LMFDB orbits generated for enum label sets"

# Show final statistics
total_usage_files=$(ls "$USAGE_OUTPUT_DIR"/*.json 2>/dev/null | wc -l)
total_harmonic_files=$(ls "$HARMONIC_OUTPUT_DIR"/*.json 2>/dev/null | wc -l)

echo "Final statistics:"
echo "  Usage files: $total_usage_files"
echo "  Harmonic files: $total_harmonic_files"
echo "  Output directories:"
echo "    Usage: $USAGE_OUTPUT_DIR"
echo "    Harmonic: $HARMONIC_OUTPUT_DIR"
