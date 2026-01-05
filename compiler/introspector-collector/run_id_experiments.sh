#!/bin/bash

echo "=== SETTING UP ID CALCULATION EXPERIMENTS ==="

# Use existing captured data
USAGE_DATA_DIR="/home/mdupont/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f"
OUTPUT_DIR="./id_experiments"

mkdir -p "$OUTPUT_DIR"

echo "Found $(find "$USAGE_DATA_DIR" -name "*.json" -path "*/test_usage_data/*" | wc -l) JSON files to process"

# Change to usage_eigenmatrix directory to run the analysis tools
cd ../usage_eigenmatrix

# Run the aggregation analysis on existing data
echo "=== Running Harmonic Analysis Aggregator ==="
USAGE_OUTPUT_DIR="$USAGE_DATA_DIR" cargo run --bin aggregate_harmonic_analysis > "../introspector-collector/$OUTPUT_DIR/harmonic_analysis.txt"

# Run the perfect hash ID generator on existing data  
echo "=== Running Perfect Hash Stable ID Generator ==="
USAGE_OUTPUT_DIR="$USAGE_DATA_DIR" cargo run --bin perfect_hash_stable_ids > "../introspector-collector/$OUTPUT_DIR/stable_ids.txt"

cd ../introspector-collector

echo "=== Experiments complete. Results in $OUTPUT_DIR ==="
ls -la "$OUTPUT_DIR"
