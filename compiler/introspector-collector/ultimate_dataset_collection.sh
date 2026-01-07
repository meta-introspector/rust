#!/bin/bash

echo "💎 === ULTIMATE DATASET COLLECTION: Rust Bootstrap with Perf + Self-Profile ==="

# Build the collector first
echo "=== Building working_usage_collector ==="
cargo build --bin working_usage_collector -j 20

echo "=== Copying mycelial spore to safe location ==="
mkdir -p ./collector_store
cp ../../target/debug/working_usage_collector ./collector_store/

echo "=== Cleaning for fresh hyphal growth ==="
cargo clean

# Create ultimate dataset directories
echo "=== Preparing ultimate dataset infrastructure ==="
mkdir -p ./ultimate_dataset/{profiling_data,perf_data,usage_data,dump_flags}
mkdir -p ./ultimate_dataset/glass_cube_etch

# Define dump flags to slice compiler features
DUMP_FLAGS=(
    ""  # baseline
    "-Z dump-mir"
    "-Z dump-mir-graphviz" 
    "-Z dump-hir"
    "-Z dump-hir-tree"
    "-Z dump-llvm-ir"
    "-Z print-type-sizes"
    "-Z print-mono-items"
    "-Z time-passes"
    "-Z verbose"
    "-Z macro-backtrace"
    "-Z borrowck=mir"
    "-Z polonius"
    "-Z chalk"
    "-Z trait-solver=next"
)

echo "🎯 === ULTIMATE DATASET COLLECTION: ${#DUMP_FLAGS[@]} compiler slices ==="

for i in "${!DUMP_FLAGS[@]}"; do
    flag="${DUMP_FLAGS[$i]}"
    slice_name="slice_$(printf "%02d" $i)_$(echo "$flag" | sed 's/[^a-zA-Z0-9]/_/g' | sed 's/__*/_/g' | sed 's/^_//;s/_$//')"
    
    if [ -z "$flag" ]; then
        slice_name="slice_00_baseline"
        echo "🔬 === Slice $i: BASELINE (no dump flags) ==="
    else
        echo "🔬 === Slice $i: $flag ==="
    fi
    
    # Create slice directory
    mkdir -p "./ultimate_dataset/glass_cube_etch/$slice_name"
    
    # Prepare flags
    RUSTFLAGS_BASE="-Z self-profile=./ultimate_dataset/profiling_data/${slice_name}_profile"
    if [ -n "$flag" ]; then
        RUSTFLAGS_FULL="$RUSTFLAGS_BASE $flag"
    else
        RUSTFLAGS_FULL="$RUSTFLAGS_BASE"
    fi
    
    echo "  🧬 Phase 1: Self-Analysis with Perf + Profiling"
    # Run with perf recording
    USAGE_OUTPUT_DIR="./ultimate_dataset/usage_data" \
    RUSTC="$(pwd)/collector_store/working_usage_collector" \
    RUSTFLAGS="$RUSTFLAGS_FULL" \
    perf record -g -o "./ultimate_dataset/perf_data/${slice_name}_collector.perf" \
    cargo build --bin working_usage_collector -j 20 2>&1 | tee "./ultimate_dataset/glass_cube_etch/$slice_name/build.log"
    
    echo "  🔄 Phase 2: Driver Dependencies with Perf + Profiling"
    USAGE_OUTPUT_DIR="./ultimate_dataset/usage_data" \
    RUSTC="$(pwd)/collector_store/working_usage_collector" \
    RUSTFLAGS="$RUSTFLAGS_FULL" \
    perf record -g -o "./ultimate_dataset/perf_data/${slice_name}_driver.perf" \
    cargo build -p rustc_driver -j 20 2>&1 | tee -a "./ultimate_dataset/glass_cube_etch/$slice_name/build.log"
    
    echo "  🧠 Phase 3: Core Components with Perf + Profiling"
    USAGE_OUTPUT_DIR="./ultimate_dataset/usage_data" \
    RUSTC="$(pwd)/collector_store/working_usage_collector" \
    RUSTFLAGS="$RUSTFLAGS_FULL" \
    perf record -g -o "./ultimate_dataset/perf_data/${slice_name}_core.perf" \
    cargo build -p rustc_middle -p rustc_hir -p rustc_ast -j 20 2>&1 | tee -a "./ultimate_dataset/glass_cube_etch/$slice_name/build.log"
    
    # Generate perf reports
    echo "  📊 Generating perf reports for slice $slice_name"
    perf report -i "./ultimate_dataset/perf_data/${slice_name}_collector.perf" --stdio > "./ultimate_dataset/glass_cube_etch/$slice_name/perf_collector.txt" 2>/dev/null
    perf report -i "./ultimate_dataset/perf_data/${slice_name}_driver.perf" --stdio > "./ultimate_dataset/glass_cube_etch/$slice_name/perf_driver.txt" 2>/dev/null
    perf report -i "./ultimate_dataset/perf_data/${slice_name}_core.perf" --stdio > "./ultimate_dataset/glass_cube_etch/$slice_name/perf_core.txt" 2>/dev/null
    
    # Convert self-profile data
    if command -v crox &> /dev/null; then
        profile_dir="./ultimate_dataset/profiling_data/${slice_name}_profile"
        if [ -d "$profile_dir" ]; then
            echo "  🔄 Converting self-profile data for $slice_name"
            crox --dir "$profile_dir" --output "./ultimate_dataset/glass_cube_etch/$slice_name/self_profile.json"
        fi
    fi
    
    # Create slice metadata
    cat > "./ultimate_dataset/glass_cube_etch/$slice_name/metadata.json" << EOF
{
    "slice_id": $i,
    "slice_name": "$slice_name",
    "dump_flag": "$flag",
    "timestamp": "$(date -Iseconds)",
    "hostname": "$(hostname)",
    "rustc_version": "$(rustc --version)",
    "perf_files": [
        "${slice_name}_collector.perf",
        "${slice_name}_driver.perf", 
        "${slice_name}_core.perf"
    ],
    "self_profile_dir": "${slice_name}_profile",
    "build_log": "build.log"
}
EOF
    
    echo "  ✅ Slice $slice_name complete"
    echo ""
done

echo "📊 === Generating Chrome Profiler Data for All Slices ==="
if command -v crox &> /dev/null; then
    for profile_dir in ./ultimate_dataset/profiling_data/*_profile; do
        if [ -d "$profile_dir" ]; then
            profile_name=$(basename "$profile_dir")
            echo "Converting $profile_name to Chrome format..."
            crox --dir "$profile_dir" --output "./ultimate_dataset/profiling_data/${profile_name}.json"
        fi
    done
fi

echo "💎 === ULTIMATE DATASET COMPLETE ==="
echo "Glass cube etch slices: $(ls -d ./ultimate_dataset/glass_cube_etch/slice_* | wc -l)"
echo "Perf data files: $(ls ./ultimate_dataset/perf_data/*.perf 2>/dev/null | wc -l)"
echo "Self-profile directories: $(ls -d ./ultimate_dataset/profiling_data/*_profile 2>/dev/null | wc -l)"
echo "Usage data files: $(ls ./ultimate_dataset/usage_data/*.json 2>/dev/null | wc -l)"

# Create ultimate dataset manifest
cat > "./ultimate_dataset/ULTIMATE_DATASET_MANIFEST.json" << EOF
{
    "dataset_name": "Rust Bootstrap Ultimate Dataset",
    "creation_date": "$(date -Iseconds)",
    "total_slices": ${#DUMP_FLAGS[@]},
    "dump_flags_tested": $(printf '%s\n' "${DUMP_FLAGS[@]}" | jq -R . | jq -s .),
    "data_types": [
        "perf_records",
        "self_profiles", 
        "usage_data",
        "build_logs",
        "compiler_dumps"
    ],
    "glass_cube_ready": true,
    "etching_complete": true
}
EOF

echo "🔮 Dataset manifest: ./ultimate_dataset/ULTIMATE_DATASET_MANIFEST.json"
echo "💎 Ready for glass cube etching!"
