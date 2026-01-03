#!/bin/bash
# Clean all scan data and logs

echo "🧹 CLEANING SCAN DATA"

# Remove logs
rm -f *.log
echo "  ✅ Removed log files"

# Remove scan results
rm -rf scan_results/
echo "  ✅ Removed scan results"

# Clean cargo
cd ../..
cargo clean > /dev/null 2>&1
echo "  ✅ Cargo cleaned"

cd compiler/rustc_driver
echo "🧹 Clean complete"
