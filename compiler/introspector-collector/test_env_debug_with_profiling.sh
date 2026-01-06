#!/bin/bash

echo "🍄 === Enhanced Mycelial Network: Building with Profiling & Usage Collection ==="

# Build the collector first
echo "=== Building working_usage_collector ==="
cargo build --bin working_usage_collector -j 20

echo "=== Copying mycelial spore to safe location ==="
mkdir -p ./collector_store
cp ../../target/debug/working_usage_collector ./collector_store/

echo "=== Cleaning for fresh hyphal growth ==="
cargo clean

# Create profiling directories
echo "=== Preparing profiling infrastructure ==="
mkdir -p ./profiling_data
mkdir -p ./test_usage_data

echo "🌐 === Phase 1: Self-Analysis with Profiling (Collector compiling itself) ==="
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
RUSTFLAGS="-Z self-profile=./profiling_data/collector_self_profile" \
cargo build --bin working_usage_collector -j 20 2>&1 | tee build_with_profiling.log

echo "🔄 === Phase 2: Expanding to Driver Dependencies with Profiling ==="
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
RUSTFLAGS="-Z self-profile=./profiling_data/driver_deps_profile" \
cargo build -p rustc_driver -j 20 2>&1 | tee -a build_with_profiling.log

echo "🧠 === Phase 3: Core Compiler Components with Profiling ==="
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
RUSTFLAGS="-Z self-profile=./profiling_data/core_components_profile" \
cargo build -p rustc_middle -p rustc_hir -p rustc_ast -j 20 2>&1 | tee -a build_with_profiling.log

echo "🎯 === Phase 4: Analysis Tools with Profiling ==="
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
RUSTFLAGS="-Z self-profile=./profiling_data/analysis_tools_profile" \
cargo build --bin embedded_graph_advisor --bin self_improving_driver --bin enhanced_dwim_corrector --bin predictive_analyzer -j 20 2>&1 | tee -a build_with_profiling.log

echo "📊 === Generating Chrome Profiler Data ==="
# Check if measureme tools are available
if command -v crox &> /dev/null; then
    echo "Converting profiling data to Chrome format..."
    for profile_dir in ./profiling_data/*_profile; do
        if [ -d "$profile_dir" ]; then
            profile_name=$(basename "$profile_dir")
            echo "Processing $profile_name..."
            crox --dir "$profile_dir" --output "./profiling_data/${profile_name}.json"
        fi
    done
else
    echo "⚠️  measureme tools not found. Install with: cargo install measureme"
    echo "   Raw profiling data available in ./profiling_data/"
fi

echo "✨ === Enhanced Mycelial Network Status ==="
echo "Usage data files created:"
ls -la ./test_usage_data/*.json 2>/dev/null | wc -l
echo "Profiling directories created:"
ls -d ./profiling_data/*_profile 2>/dev/null | wc -l
echo "Chrome profiler files:"
ls -la ./profiling_data/*.json 2>/dev/null | wc -l
echo "Latest hyphal growth patterns:"
ls -t ./test_usage_data/*.json 2>/dev/null | head -5

echo "🔮 === Running Predictive Analysis ==="
if [ -f "../../target/debug/predictive_analyzer" ]; then
    ../../target/debug/predictive_analyzer
else
    echo "⚠️  Predictive analyzer not built yet"
fi

echo "🍄 Build log saved to build_with_profiling.log"
echo "📊 Profiling data available in ./profiling_data/"
echo "🔍 Usage data available in ./test_usage_data/"
