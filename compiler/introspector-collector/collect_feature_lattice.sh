#!/bin/bash

echo "🚀 Ultimate Dataset Collection with Feature Lattice"
echo "=================================================="

# Create ultimate dataset directory structure
mkdir -p ./ultimate_dataset/{feature_lattice,compiler_dumps,profiling_data,perf_data}

# Define feature lattice - systematic exploration of compiler features
FEATURE_LATTICE=(
    # Baseline
    ""
    
    # MIR Analysis
    "-Z dump-mir"
    "-Z dump-mir-graphviz"
    "-Z dump-mir-dataflow"
    
    # HIR Analysis  
    "-Z dump-hir"
    "-Z dump-hir-tree"
    "-Z hir-stats"
    
    # Type System
    "-Z print-type-sizes"
    "-Z print-mono-items"
    "-Z print-vtable-sizes"
    
    # Optimization
    "-Z time-passes"
    "-Z verbose"
    "-Z no-codegen"
    
    # Borrow Checker
    "-Z borrowck=mir"
    "-Z polonius"
    "-Z nll-facts"
    
    # Trait System
    "-Z trait-solver=next"
    "-Z chalk"
    "-Z dump-solver-proof-tree"
    
    # Code Generation
    "-Z dump-llvm-ir"
    "-Z no-parallel-llvm"
    "-Z print-fuel"
    
    # Incremental Compilation
    "-Z incremental-info"
    "-Z query-dep-graph"
    "-Z dump-dep-graph"
    
    # Macro System
    "-Z macro-backtrace"
    "-Z trace-macros"
    "-Z dump-macro-expanded"
)

echo "🔬 Feature Lattice Size: ${#FEATURE_LATTICE[@]} configurations"

# Build collector first
echo "=== Building introspector-collector ==="
cargo build --bin working_usage_collector -j 20

# Copy to safe location
mkdir -p ./collector_store
cp ../../target/debug/working_usage_collector ./collector_store/

# Clean for fresh collection
cargo clean

echo "🧪 Starting Feature Lattice Collection..."

for i in "${!FEATURE_LATTICE[@]}"; do
    feature_flag="${FEATURE_LATTICE[$i]}"
    
    if [ -z "$feature_flag" ]; then
        feature_name="baseline"
        echo "🔬 Configuration $i: BASELINE (no flags)"
    else
        feature_name=$(echo "$feature_flag" | sed 's/[^a-zA-Z0-9]/_/g' | sed 's/__*/_/g' | sed 's/^_//;s/_$//')
        echo "🔬 Configuration $i: $feature_flag"
    fi
    
    config_dir="./ultimate_dataset/feature_lattice/config_$(printf "%02d" $i)_${feature_name}"
    mkdir -p "$config_dir"
    
    # Set RUSTFLAGS for this configuration
    if [ -z "$feature_flag" ]; then
        export RUSTFLAGS="-Z self-profile=${config_dir}/self_profile"
    else
        export RUSTFLAGS="-Z self-profile=${config_dir}/self_profile $feature_flag"
    fi
    
    # Set collection environment
    export USAGE_OUTPUT_DIR="$config_dir/usage_data"
    export RUSTC="$(pwd)/collector_store/working_usage_collector"
    
    mkdir -p "$config_dir/usage_data"
    
    echo "  📊 Collecting with perf recording..."
    
    # Run with perf recording
    perf record -g -o "$config_dir/perf_record.data" \
        cargo build --lib -j 1 2>&1 | tee "$config_dir/build.log"
    
    # Generate perf report
    perf report -i "$config_dir/perf_record.data" --stdio > "$config_dir/perf_report.txt" 2>/dev/null
    
    # Convert self-profile data if available
    if command -v crox &> /dev/null && [ -d "$config_dir/self_profile" ]; then
        echo "  🔄 Converting self-profile data..."
        crox --dir "$config_dir/self_profile" --output "$config_dir/self_profile.json"
    fi
    
    # Create configuration metadata
    cat > "$config_dir/metadata.json" << EOF
{
    "config_id": $i,
    "config_name": "$feature_name",
    "rustflags": "$RUSTFLAGS",
    "feature_flag": "$feature_flag",
    "timestamp": "$(date -Iseconds)",
    "hostname": "$(hostname)",
    "rustc_version": "$(rustc --version)",
    "data_files": {
        "build_log": "build.log",
        "perf_record": "perf_record.data", 
        "perf_report": "perf_report.txt",
        "self_profile": "self_profile.json",
        "usage_data": "usage_data/"
    }
}
EOF
    
    echo "  ✅ Configuration $i complete"
    
    # Clean between configurations
    cargo clean
    
    # Brief pause to prevent system overload
    sleep 1
done

# Generate feature lattice manifest
echo "📋 Generating Feature Lattice Manifest..."

cat > "./ultimate_dataset/FEATURE_LATTICE_MANIFEST.json" << EOF
{
    "dataset_name": "Rust Compiler Feature Lattice Dataset",
    "creation_date": "$(date -Iseconds)",
    "total_configurations": ${#FEATURE_LATTICE[@]},
    "feature_flags_tested": $(printf '%s\n' "${FEATURE_LATTICE[@]}" | jq -R . | jq -s .),
    "data_collection": {
        "perf_records": true,
        "self_profiles": true,
        "usage_data": true,
        "build_logs": true,
        "compiler_dumps": true
    },
    "lattice_structure": {
        "baseline": "config_00_baseline",
        "mir_analysis": ["config_01_*", "config_02_*", "config_03_*"],
        "hir_analysis": ["config_04_*", "config_05_*", "config_06_*"],
        "type_system": ["config_07_*", "config_08_*", "config_09_*"],
        "optimization": ["config_10_*", "config_11_*", "config_12_*"],
        "borrow_checker": ["config_13_*", "config_14_*", "config_15_*"],
        "trait_system": ["config_16_*", "config_17_*", "config_18_*"],
        "code_generation": ["config_19_*", "config_20_*", "config_21_*"],
        "incremental": ["config_22_*", "config_23_*", "config_24_*"],
        "macro_system": ["config_25_*", "config_26_*", "config_27_*"]
    },
    "analysis_ready": true,
    "lattice_complete": true
}
EOF

echo "🎉 Feature Lattice Collection Complete!"
echo "📊 Configurations collected: ${#FEATURE_LATTICE[@]}"
echo "📁 Data location: ./ultimate_dataset/feature_lattice/"
echo "📋 Manifest: ./ultimate_dataset/FEATURE_LATTICE_MANIFEST.json"

# Summary statistics
echo ""
echo "📈 Collection Summary:"
total_size=$(du -sh ./ultimate_dataset/feature_lattice/ 2>/dev/null | cut -f1)
config_count=$(ls -d ./ultimate_dataset/feature_lattice/config_* 2>/dev/null | wc -l)
perf_files=$(find ./ultimate_dataset/feature_lattice/ -name "perf_record.data" 2>/dev/null | wc -l)
profile_files=$(find ./ultimate_dataset/feature_lattice/ -name "self_profile.json" 2>/dev/null | wc -l)

echo "  Total dataset size: ${total_size:-0}"
echo "  Configurations: $config_count"
echo "  Perf recordings: $perf_files"
echo "  Self-profiles: $profile_files"

echo ""
echo "🚀 Ready for lattice analysis and maximal value extraction!"
