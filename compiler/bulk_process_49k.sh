#!/bin/bash

echo "🚀 BULK PROCESSING 49K+ RUST FILES"
echo "=================================="

cd compiler/zombie_driver2

echo "📊 Total .rs files to process: $(find ../.. -name '*.rs' | wc -l)"

echo "🧟 Running zombie analysis on ALL 49k+ files with 20 cores..."
find ../.. -name "*.rs" | xargs -n 1 -P 20 -I {} sh -c 'echo "Processing: {}" >> bulk_progress.log; timeout 5 ./target/release/zombie-rustc {} >> bulk_output.log 2>> bulk_errors.log || true'

echo "📊 Analysis files generated: $(find ../.. -name '*.syn_analysis.json' | wc -l)"

echo "🧮 Processing complete lattice..."
cargo run --bin global-ast-collector

echo "✅ Bulk processing complete!"
