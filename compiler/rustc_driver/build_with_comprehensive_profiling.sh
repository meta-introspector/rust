#!/bin/bash

# Build rustc_driver with comprehensive profiling and usage collection
echo "=== Building rustc_driver with comprehensive profiling ==="

# Create output directories
mkdir -p ./profiling_output
mkdir -p ../../usage_data

# Clean first
echo "Cleaning build artifacts..."
cargo clean

# Build working_usage_collector first
echo "Building working_usage_collector..."
cd ../introspector-collector && cargo build --bin working_usage_collector
cd ../rustc_driver

# Phase 1: Build with self-profiling enabled
echo "Phase 1: Building with self-profile..."
RUSTFLAGS="-Z self-profile=./profiling_output/rustc_driver_build_profile" \
rustc --crate-type=bin src/main.rs -o rustc_driver_traced 2>&1 | tee build_profiling.log

# Phase 2: Build with usage collector and profiling
echo "Phase 2: Building with usage collector and profiling..."
export CFG_RELEASE_CHANNEL=dev
USAGE_OUTPUT_DIR="../../usage_data" \
CFG_RELEASE_CHANNEL=dev \
RUSTC_INSTALL_BINDIR=/usr/bin \
RUSTC="../introspector-collector/target/debug/working_usage_collector" \
RUSTFLAGS="-Z self-profile=./profiling_output/usage_collection_profile" \
cargo build --workspace --all-targets -j 20 2>&1 | tee -a build_profiling.log

# Phase 3: Build specific analysis tools with detailed profiling
echo "Phase 3: Building analysis tools with detailed profiling..."
USAGE_OUTPUT_DIR="../../usage_data" \
RUSTC="../introspector-collector/target/debug/working_usage_collector" \
RUSTFLAGS="-Z self-profile=./profiling_output/analysis_tools_profile -Z time-passes" \
cargo build --bin embedded_graph_advisor --bin self_improving_driver --bin enhanced_dwim_corrector -j 20 2>&1 | tee -a build_profiling.log

# Generate Chrome profiler data
echo "=== Generating Chrome profiler data ==="
if command -v crox &> /dev/null; then
    for profile_dir in ./profiling_output/*_profile; do
        if [ -d "$profile_dir" ]; then
            profile_name=$(basename "$profile_dir")
            echo "Converting $profile_name to Chrome format..."
            crox --dir "$profile_dir" --output "./profiling_output/${profile_name}.json"
        fi
    done
else
    echo "⚠️  measureme tools not found. Install with: cargo install measureme"
    echo "   Raw profiling data available in ./profiling_output/"
fi

echo "=== Comprehensive Profiling Results ==="
echo "Self-profile directories:"
ls -d ./profiling_output/*_profile 2>/dev/null | wc -l
echo "Chrome profiler files:"
ls -la ./profiling_output/*.json 2>/dev/null | wc -l
echo "Usage data files:"
ls -la ../../usage_data/*.json 2>/dev/null | wc -l
echo "Build log size:"
ls -lh build_profiling.log 2>/dev/null

echo "=== Analysis Ready ==="
echo "📊 Chrome profiler data: ./profiling_output/*.json"
echo "📈 Usage data: ../../usage_data/"
echo "📝 Build log: build_profiling.log"
echo "🔍 View *.json files in Chrome DevTools Performance tab"
