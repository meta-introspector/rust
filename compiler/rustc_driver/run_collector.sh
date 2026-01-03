#!/bin/bash

echo "=== Building usage collector ==="
rustc usage_collector.rs -o usage_collector || exit 1

echo "=== Running collection with error tracking ==="
./build_with_collector.sh > collection.log 2>&1

echo "=== Analyzing results ==="
echo "Total JSON files created: $(find usage_data/ -name "*.json" | wc -l)"
echo "Total crates processed: $(grep "COLLECTING USAGE DATA FOR CRATE" collection.log | wc -l)"
echo "Errors found: $(grep -i "error\|panic\|failed" collection.log | wc -l)"

echo "=== Error summary ==="
grep -i "error\|panic\|failed" collection.log | head -10

echo "=== Success summary ==="
grep "COLLECTION COMPLETE" collection.log | wc -l
echo "successful crate collections"
