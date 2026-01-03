#!/bin/bash
# Scan rustc main compilation with parallel processing

set -e

echo "🔥 RUSTC MAIN SCANNER 🔥"
echo ""

# Build minimal rustc
echo "Building minimal rustc..."
rustc minimal_rustc.rs -o minimal_rustc

# Create output directory
mkdir -p scan_results

# Get timestamp
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="rustc_main_scan_${TIMESTAMP}.log"

echo "Starting rustc main compilation with scanner..."
echo "  Jobs: 20 parallel"
echo "  Scanner: ./minimal_rustc" 
echo "  Log: $LOG_FILE"
echo ""

# Run compilation with scanner
cd ../..
RUSTC=/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver/minimal_rustc \
cargo build -j20 -p rustc_driver > /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver/$LOG_FILE 2>&1 &

BUILD_PID=$!
cd compiler/rustc_driver

echo "✅ Build started (PID: $BUILD_PID)"
echo ""
echo "📊 Monitor progress:"
echo "  tail -f $LOG_FILE"
echo "  watch 'find scan_results/ -name \"*.json\" | wc -l'"
echo "  htop"
echo ""
echo "🎯 Will capture complete rustc symbol dependency graph!"
