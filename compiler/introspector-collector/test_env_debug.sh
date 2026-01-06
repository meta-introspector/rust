#!/bin/bash

echo "🍄 === Mycelial Network Expansion: Building Driver Dependencies ==="
cargo build --bin working_usage_collector -j 20

echo "=== Copying mycelial spore to safe location ==="
mkdir -p ./collector_store
cp ../../target/debug/working_usage_collector ./collector_store/

echo "=== Cleaning for fresh hyphal growth ==="
cargo clean

echo "🌐 === Phase 1: Self-Analysis (Collector compiling itself) ==="
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
cargo build --bin working_usage_collector -j 20 2>&1 | tee build_with_collector.log

echo "🔄 === Phase 2: Expanding to Driver Dependencies ==="
# Build rustc_driver dependencies that our collector needs
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
cargo build -p rustc_driver -j 20 2>&1 | tee -a build_with_collector.log

echo "🧠 === Phase 3: Core Compiler Components ==="
# Build essential compiler crates
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
cargo build -p rustc_middle -p rustc_hir -p rustc_ast -j 20 2>&1 | tee -a build_with_collector.log

echo "🎯 === Phase 4: Analysis Tools ==="
# Build our analysis binaries
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
cargo build --bin embedded_graph_advisor --bin self_improving_driver --bin enhanced_dwim_corrector -j 20 2>&1 | tee -a build_with_collector.log

echo "✨ === Mycelial Network Status ==="
echo "Usage data files created:"
ls -la ./test_usage_data/*.json 2>/dev/null | wc -l
echo "Latest hyphal growth patterns:"
ls -t ./test_usage_data/*.json 2>/dev/null | head -5
echo "🍄 Build log saved to build_with_collector.log"
