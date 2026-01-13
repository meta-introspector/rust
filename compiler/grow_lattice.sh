#!/bin/bash

echo "🌱 GROWING THE RUST LATTICE"
echo "==========================="

# Add more crates to expand our knowledge base
CRATES=(
    "tokio"
    "async-trait" 
    "futures"
    "clap"
    "regex"
    "rayon"
    "crossbeam"
    "parking_lot"
    "tracing"
    "anyhow"
    "thiserror"
    "uuid"
    "chrono"
    "reqwest"
    "diesel"
)

cd ../rustc_driver

echo "📦 Adding crates to Cargo.toml..."
for crate in "${CRATES[@]}"; do
    echo "Adding $crate..."
    echo "$crate = \"*\"" >> Cargo.toml
done

echo "🔄 Building with new crates..."
cargo build --verbose 2>&1 | grep rustc | while read cmd; do
    echo "Analyzing: $cmd"
    ../zombie_driver2/target/release/zombie-rustc $cmd 2>/dev/null || true
done

echo "📊 Updating analysis files list..."
cd ../zombie_driver2
find . -name "*.syn_analysis.json" > all_analysis_files.txt

echo "🧮 Regenerating signatures..."
cargo run --bin positional-prime-encoder

echo "📈 Updated lattice analysis..."
cargo run --bin crate-signature-analysis

echo "✅ Lattice expansion complete!"
echo "New analysis files: $(wc -l < all_analysis_files.txt)"
