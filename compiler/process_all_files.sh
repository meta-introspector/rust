#!/bin/bash

echo "🚀 PROCESSING ALL 161,820 RUST FILES"
echo "===================================="

cd compiler/zombie_driver2

echo "📊 Starting systematic processing..."
echo "Files to process: $(find ../.. -name '*.rs' | wc -l)"

# Create batches of 1000 files each
find ../.. -name "*.rs" | split -l 1000 - batch_

echo "📦 Created $(ls batch_* | wc -l) batches of 1000 files each"

# Process each batch with 20 parallel workers
for batch in batch_*; do
    echo "Processing batch: $batch"
    cat "$batch" | xargs -n 1 -P 20 -I {} timeout 30 ./target/release/zombie-rustc {} 2>/dev/null || true
    
    # Show progress
    echo "Progress: $(find ../.. -name '*.syn_analysis.json' | wc -l) files analyzed"
    
    # Clean up batch file
    rm "$batch"
done

echo "✅ All batches processed!"
echo "Final count: $(find ../.. -name '*.syn_analysis.json' | wc -l) analysis files"
