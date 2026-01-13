#!/bin/bash

echo "🌌 CONSUMING ALL RUST COMPILER CRATES INTO LATTICE"
echo "=================================================="

cd zombie_driver2

echo "🔍 Discovering all crates..."
find .. -name "Cargo.toml" | wc -l
echo "crates found in compiler ecosystem"

echo "🧟 Analyzing ALL crates with zombie-rustc..."
find .. -name "Cargo.toml" -exec dirname {} \; | while read crate_dir; do
    echo "Processing: $crate_dir"
    cd "$crate_dir" 2>/dev/null || continue
    
    # Build and analyze each crate
    cargo build --verbose 2>&1 | grep rustc | head -20 | while read cmd; do
        ../zombie_driver2/target/release/zombie-rustc $cmd 2>/dev/null || true
    done
    
    cd - >/dev/null
done

echo "📊 Updating analysis files..."
find . -name "*.syn_analysis.json" > all_analysis_files.txt

echo "🧮 Regenerating complete lattice signatures..."
cargo run --bin positional-prime-encoder

echo "📈 Complete lattice analysis..."
cargo run --bin crate-signature-analysis

echo "🎯 Final lattice statistics:"
echo "Analysis files: $(wc -l < all_analysis_files.txt)"
echo "Signature coverage complete!"
