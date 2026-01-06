#!/bin/bash

echo "🍄 === Mycelial Network Expansion: Building Profile-Driven Compiler ==="
cargo build --bin working_usage_collector --bin profile_driven_compiler -j 20

echo "=== Copying mycelial spores to safe location ==="
mkdir -p ./collector_store
cp ../../target/debug/working_usage_collector ./collector_store/
cp ../../target/debug/profile_driven_compiler ./collector_store/ 2>/dev/null || echo "Profile driver not built yet"

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

echo "🤖 === Phase 5: Profile-Driven Compiler Driver ==="
# Build the profile-driven compiler with collected data
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
cargo build --bin profile_driven_compiler -j 20 2>&1 | tee -a build_with_collector.log

echo "📊 === Phase 6: Data Integration & Analysis ==="
# Copy the profile-driven compiler to collector store
cp ../../target/debug/profile_driven_compiler ./collector_store/ 2>/dev/null || echo "Profile driver build failed"

# Run profile analysis on collected data
echo "Running profile-driven analysis..."
if [ -f "./collector_store/profile_driven_compiler" ]; then
    ./collector_store/profile_driven_compiler 2>&1 | tee profile_analysis.log
else
    echo "⚠️  Profile-driven compiler not available"
fi

echo "🔬 === Phase 7: Enhanced Compilation with Profile Data ==="
# Use profile data to guide next compilation
if [ -f "./collector_store/profile_driven_compiler" ]; then
    echo "Building with profile-driven suggestions..."
    
    # Set up profile-driven environment
    export PROFILE_DATA_DIR="./test_usage_data"
    export CRATE_USAGE_DIR="../../../mycelial-usage-data/crate_usage_data"
    export RUSTC_PROFILE_DRIVEN="$(pwd)/collector_store/profile_driven_compiler"
    
    # Build with profile guidance
    USAGE_OUTPUT_DIR="./test_usage_data" \
    RUSTC="$(pwd)/collector_store/working_usage_collector" \
    cargo build --bin merge_usage_report --bin liftable_examples --bin formal_gap_analysis -j 20 2>&1 | tee -a build_with_profile.log
fi

echo "🧬 === Phase 8: Merge and Analyze All Usage Data ==="
# Run our analysis tools on the collected data
echo "Merging usage reports..."
if [ -f "../../target/debug/merge_usage_report" ]; then
    ../../target/debug/merge_usage_report 2>&1 | tee merge_analysis.log
fi

echo "Extracting liftable examples..."
if [ -f "../../target/debug/liftable_examples" ]; then
    ../../target/debug/liftable_examples 2>&1 | tee liftable_analysis.log
fi

echo "Running formal gap analysis..."
if [ -f "../../target/debug/formal_gap_analysis" ]; then
    ../../target/debug/formal_gap_analysis 2>&1 | tee gap_analysis.log
fi

echo "✨ === Mycelial Network Status ==="
echo "Usage data files created:"
ls -la ./test_usage_data/*.json 2>/dev/null | wc -l

echo "Profile analysis results:"
[ -f "profile_analysis.log" ] && echo "  - Profile analysis: $(wc -l < profile_analysis.log) lines"
[ -f "merge_analysis.log" ] && echo "  - Merge analysis: $(wc -l < merge_analysis.log) lines"
[ -f "liftable_analysis.log" ] && echo "  - Liftable analysis: $(wc -l < liftable_analysis.log) lines"
[ -f "gap_analysis.log" ] && echo "  - Gap analysis: $(wc -l < gap_analysis.log) lines"

echo "Latest hyphal growth patterns:"
ls -t ./test_usage_data/*.json 2>/dev/null | head -5

echo "Generated reports:"
[ -f "merged_usage_report.json" ] && echo "  - Merged usage report: $(wc -c < merged_usage_report.json) bytes"

echo "🍄 Build logs saved:"
echo "  - build_with_collector.log"
echo "  - build_with_profile.log (if profile-driven build ran)"
echo "  - profile_analysis.log (if profile analysis ran)"
echo "  - merge_analysis.log (if merge analysis ran)"
echo "  - liftable_analysis.log (if liftable analysis ran)"
echo "  - gap_analysis.log (if gap analysis ran)"

echo "🚀 === Profile-Driven Compiler Ready ==="
echo "Next steps:"
echo "  1. Review profile_analysis.log for usage patterns"
echo "  2. Check merged_usage_report.json for comprehensive data"
echo "  3. Use profile-driven suggestions for next compilation"
echo "  4. Iterate with enhanced compiler intelligence"
