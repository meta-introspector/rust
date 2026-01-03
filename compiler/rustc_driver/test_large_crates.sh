#!/bin/bash

# Test usage collector on rustc source files directly
echo "=== Testing Usage Collector on Rustc Source Files ==="

# Build the collector
rustc usage_collector.rs -o usage_collector

# Test on a rustc source file
echo "Testing on rustc_hir lib.rs..."
./usage_collector --crate-type lib ../rustc_hir/src/lib.rs 2>&1 | head -10

echo "Testing on rustc_middle lib.rs..."  
./usage_collector --crate-type lib ../rustc_middle/src/lib.rs 2>&1 | head -10

# Show results
echo "=== Results ==="
ls -la usage_data/
echo "=== Sample file content ==="
head -20 usage_data/*.json | head -20
