#!/bin/bash

echo "🦀 Self-Analysis Feature Lattice Collection"
echo "=========================================="
echo "Compiling our own introspector-collector code with different compiler features"
echo "to compare profiles and analyze our own compilation patterns"

# Create self-analysis dataset directory
mkdir -p ./self_analysis_dataset/{feature_lattice,profiles,comparisons}

# Define focused feature lattice for self-analysis
SELF_ANALYSIS_FEATURES=(
    ""                          # baseline
    "-Z dump-hir"              # our HIR usage
    "-Z dump-mir"              # our MIR patterns  
    "-Z time-passes"           # compilation timing
    "-Z print-type-sizes"      # our type complexity
    "-Z borrowck=mir"          # our borrow patterns
    "-Z trait-solver=next"     # our trait usage
    "-Z verbose"               # detailed output
    "-Z macro-backtrace"       # our macro usage
)

echo "🔬 Self-Analysis Feature Lattice: ${#SELF_ANALYSIS_FEATURES[@]} configurations"
echo "📁 Target: introspector-collector library compilation"

# Build collector first
echo "=== Building working_usage_collector ==="
cargo build --bin working_usage_collector -j 20

mkdir -p ./collector_store
cp ../../target/debug/working_usage_collector ./collector_store/

echo "🧪 Starting Self-Analysis Collection..."

for i in "${!SELF_ANALYSIS_FEATURES[@]}"; do
    feature_flag="${SELF_ANALYSIS_FEATURES[$i]}"
    
    if [ -z "$feature_flag" ]; then
        feature_name="baseline"
        echo "🔬 Self-Analysis $i: BASELINE"
    else
        feature_name=$(echo "$feature_flag" | sed 's/[^a-zA-Z0-9]/_/g' | sed 's/__*/_/g' | sed 's/^_//;s/_$//')
        echo "🔬 Self-Analysis $i: $feature_flag"
    fi
    
    config_dir="./self_analysis_dataset/feature_lattice/self_$(printf "%02d" $i)_${feature_name}"
    mkdir -p "$config_dir"
    
    # Clean for fresh compilation
    cargo clean
    
    # Set RUSTFLAGS for this configuration
    if [ -z "$feature_flag" ]; then
        export RUSTFLAGS="-Z self-profile=${config_dir}/self_profile"
    else
        export RUSTFLAGS="-Z self-profile=${config_dir}/self_profile $feature_flag"
    fi
    
    # Set collection environment for our own code
    export USAGE_OUTPUT_DIR="$config_dir/usage_data"
    export RUSTC="$(pwd)/collector_store/working_usage_collector"
    
    mkdir -p "$config_dir/usage_data"
    
    echo "  📊 Self-compiling with perf recording..."
    
    # Compile our own library with perf recording
    perf record -g -o "$config_dir/self_perf.data" \
        cargo build --lib -j 1 2>&1 | tee "$config_dir/self_build.log"
    
    # Also compile our binaries
    echo "  🎯 Compiling our demo binaries..."
    perf record -g -o "$config_dir/bins_perf.data" \
        cargo build --bins -j 1 2>&1 | tee -a "$config_dir/self_build.log"
    
    # Generate perf reports
    echo "  📈 Generating perf analysis..."
    perf report -i "$config_dir/self_perf.data" --stdio > "$config_dir/self_perf_report.txt" 2>/dev/null
    perf report -i "$config_dir/bins_perf.data" --stdio > "$config_dir/bins_perf_report.txt" 2>/dev/null
    
    # Convert self-profile data
    if command -v crox &> /dev/null && [ -d "$config_dir/self_profile" ]; then
        echo "  🔄 Converting self-profile data..."
        crox --dir "$config_dir/self_profile" --output "$config_dir/self_profile.json"
    fi
    
    # Analyze our own compilation
    echo "  🔍 Analyzing our compilation patterns..."
    
    # Count our usage data files
    usage_files=$(find "$config_dir/usage_data" -name "*.json" 2>/dev/null | wc -l)
    
    # Extract key metrics from build log
    compile_time=$(grep "Finished" "$config_dir/self_build.log" | grep -o '[0-9.]*s' | head -1)
    warnings=$(grep "warning:" "$config_dir/self_build.log" | wc -l)
    
    # Create self-analysis metadata
    cat > "$config_dir/self_metadata.json" << EOF
{
    "config_id": $i,
    "config_name": "$feature_name",
    "rustflags": "$RUSTFLAGS",
    "feature_flag": "$feature_flag",
    "timestamp": "$(date -Iseconds)",
    "self_analysis": {
        "target": "introspector-collector library + binaries",
        "compile_time": "$compile_time",
        "warnings": $warnings,
        "usage_files_generated": $usage_files,
        "perf_recordings": 2,
        "self_profile_available": $([ -f "$config_dir/self_profile.json" ] && echo "true" || echo "false")
    },
    "data_files": {
        "self_build_log": "self_build.log",
        "self_perf_data": "self_perf.data",
        "bins_perf_data": "bins_perf.data", 
        "self_perf_report": "self_perf_report.txt",
        "bins_perf_report": "bins_perf_report.txt",
        "self_profile": "self_profile.json",
        "usage_data_dir": "usage_data/"
    }
}
EOF
    
    echo "  ✅ Self-Analysis $i complete"
done

# Generate comparison analysis
echo "📊 Generating Self-Analysis Comparisons..."

python3 << 'EOF'
import json
import os
from pathlib import Path

# Collect all self-analysis results
results = []
for config_dir in sorted(Path("./self_analysis_dataset/feature_lattice").glob("self_*")):
    metadata_file = config_dir / "self_metadata.json"
    if metadata_file.exists():
        with open(metadata_file) as f:
            results.append(json.load(f))

# Generate comparison report
comparison = {
    "self_analysis_comparison": {
        "total_configurations": len(results),
        "configurations": results,
        "insights": {
            "baseline_vs_features": "Compare baseline compilation against feature-enabled",
            "performance_impact": "Measure overhead of different compiler features",
            "our_code_patterns": "Analyze how our own code compiles under different settings",
            "feature_effectiveness": "Which features provide most insight into our code"
        }
    }
}

with open("./self_analysis_dataset/SELF_ANALYSIS_COMPARISON.json", "w") as f:
    json.dump(comparison, f, indent=2)

print("✅ Self-analysis comparison generated")
EOF

echo "🎉 Self-Analysis Feature Lattice Collection Complete!"
echo ""
echo "📊 Results:"
config_count=$(ls -d ./self_analysis_dataset/feature_lattice/self_* 2>/dev/null | wc -l)
total_size=$(du -sh ./self_analysis_dataset/ 2>/dev/null | cut -f1)

echo "  Configurations analyzed: $config_count"
echo "  Total dataset size: ${total_size:-0}"
echo "  Self-analysis comparison: ./self_analysis_dataset/SELF_ANALYSIS_COMPARISON.json"

echo ""
echo "🔍 Next Steps:"
echo "  1. Compare baseline vs feature-enabled compilation profiles"
echo "  2. Analyze which features reveal most about our code patterns"
echo "  3. Identify performance impact of different compiler features"
echo "  4. Use insights to optimize our ultimate dataset collection"

echo ""
echo "🚀 Ready to analyze how our own code compiles!"
