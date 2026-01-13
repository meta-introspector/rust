#!/bin/bash

echo "🚀 FAST SUBMODULES SCANNER"
echo "=========================="

cd compiler/zombie_driver2

echo "🔍 Discovering all submodules..."
find ../.. -name "*.rs" | head -100 | while read rs_file; do
    echo "Fast analyzing: $rs_file"
    ./target/release/zombie-rustc "$rs_file" 2>/dev/null || true
done

echo "📊 Collecting all analysis data..."
find ../.. -name "*.syn_analysis.json" > all_submodules_analysis.txt
echo "Found $(wc -l < all_submodules_analysis.txt) analysis files"

echo "🧮 Processing complete lattice..."
cargo run --bin positional-prime-encoder

echo "🔮 Soul analysis across thousands of crates..."
cargo run --bin rust-soul-finder

echo "✅ Fast scan complete!"
