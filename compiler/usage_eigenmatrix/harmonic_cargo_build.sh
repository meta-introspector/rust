#!/bin/bash

# Production cargo build integration with harmonic analysis
echo "=== PRODUCTION HARMONIC CARGO BUILD ==="

# Setup production environment
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}"
export HARMONIC_DATA_DIR="$CARGO_TARGET_DIR/harmonic"
export USAGE_OUTPUT_DIR="$HARMONIC_DATA_DIR/usage"
export HARMONIC_OUTPUT_DIR="$HARMONIC_DATA_DIR/analysis"
export RUSTC_VERSION="1.74.0"
export ENABLE_HARMONIC_ANALYSIS=1

# Create harmonic directories in target
mkdir -p "$HARMONIC_DATA_DIR"
mkdir -p "$USAGE_OUTPUT_DIR" 
mkdir -p "$HARMONIC_OUTPUT_DIR"

echo "Harmonic data will be stored in: $HARMONIC_DATA_DIR"

# Build harmonic collector
EIGENMATRIX_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix"
COLLECTOR_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/introspector-collector"

echo "--- Building harmonic tools ---"
cd "$EIGENMATRIX_DIR"
cargo build --release --bin version_stable_compiler --bin practical_harmonic_compiler --bin perfect_hash_stable_ids

echo "--- Building usage collector ---"
cd "$COLLECTOR_DIR"
cargo build --release --bin working_usage_collector

# Set up harmonic rustc replacement
export HARMONIC_RUSTC="$COLLECTOR_DIR/target/release/working_usage_collector"

if [ ! -f "$HARMONIC_RUSTC" ]; then
    # Fallback to debug build
    export HARMONIC_RUSTC="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector"
fi

if [ ! -f "$HARMONIC_RUSTC" ]; then
    echo "❌ Harmonic collector not found at: $HARMONIC_RUSTC"
    exit 1
fi

echo "✓ Using harmonic rustc: $HARMONIC_RUSTC"

# Function to run cargo with harmonic analysis
run_harmonic_cargo() {
    local cargo_args="$@"
    
    echo "--- Running cargo with harmonic analysis ---"
    echo "Command: cargo $cargo_args"
    echo "Target dir: $CARGO_TARGET_DIR"
    echo "Harmonic dir: $HARMONIC_DATA_DIR"
    
    # Run cargo with our harmonic rustc
    RUSTC="$HARMONIC_RUSTC" \
    USAGE_OUTPUT_DIR="$USAGE_OUTPUT_DIR" \
    HARMONIC_OUTPUT_DIR="$HARMONIC_OUTPUT_DIR" \
    RUSTC_VERSION="$RUSTC_VERSION" \
    ENABLE_HARMONIC_ANALYSIS="$ENABLE_HARMONIC_ANALYSIS" \
    cargo "$@"
    local cargo_exit_code=$?
    
    # Generate harmonic analysis after build
    if [ $cargo_exit_code -eq 0 ]; then
        echo "--- Generating harmonic analysis ---"
        
        # Count collected files
        usage_files=$(find "$USAGE_OUTPUT_DIR" -name "*.json" 2>/dev/null | wc -l)
        echo "Usage files collected: $usage_files"
        
        if [ "$usage_files" -gt 0 ]; then
            # Run perfect hash analysis
            cd "$EIGENMATRIX_DIR"
            echo "Running perfect hash analysis..."
            ./target/release/perfect_hash_stable_ids > "$HARMONIC_OUTPUT_DIR/perfect_hash_report.txt" 2>&1
            
            # Run version stability analysis  
            echo "Running version stability analysis..."
            ./target/release/version_stable_compiler > "$HARMONIC_OUTPUT_DIR/version_stability_report.txt" 2>&1
            
            # Run practical harmonic analysis
            echo "Running practical harmonic analysis..."
            ./target/release/practical_harmonic_compiler > "$HARMONIC_OUTPUT_DIR/practical_harmonic_report.txt" 2>&1
            
            # Generate summary
            cat > "$HARMONIC_OUTPUT_DIR/build_summary.json" << EOF
{
  "build_timestamp": "$(date -Iseconds)",
  "cargo_command": "cargo $cargo_args",
  "target_directory": "$CARGO_TARGET_DIR",
  "harmonic_directory": "$HARMONIC_DATA_DIR",
  "usage_files_collected": $usage_files,
  "rustc_version": "$RUSTC_VERSION",
  "harmonic_rustc": "$HARMONIC_RUSTC",
  "analysis_files": [
    "perfect_hash_report.txt",
    "version_stability_report.txt", 
    "practical_harmonic_report.txt"
  ]
}
EOF
            
            echo "✓ Harmonic analysis complete"
            echo "✓ Results stored in: $HARMONIC_OUTPUT_DIR"
        else
            echo "⚠ No usage files collected - check harmonic rustc setup"
        fi
    else
        echo "❌ Cargo build failed with exit code: $cargo_exit_code"
    fi
    
    return $cargo_exit_code
}

# Main execution
if [ $# -eq 0 ]; then
    # Default: build current project
    run_harmonic_cargo build
else
    # Run with provided arguments
    run_harmonic_cargo "$@"
fi

# Show final results
echo ""
echo "=== HARMONIC BUILD RESULTS ==="
echo "Target directory: $CARGO_TARGET_DIR"
echo "Harmonic data: $HARMONIC_DATA_DIR"

if [ -d "$HARMONIC_DATA_DIR" ]; then
    echo "Harmonic files generated:"
    find "$HARMONIC_DATA_DIR" -name "*.json" -o -name "*.txt" | head -10
    
    total_files=$(find "$HARMONIC_DATA_DIR" -type f | wc -l)
    echo "Total harmonic files: $total_files"
    
    if [ -f "$HARMONIC_OUTPUT_DIR/build_summary.json" ]; then
        echo ""
        echo "Build summary:"
        cat "$HARMONIC_OUTPUT_DIR/build_summary.json"
    fi
else
    echo "❌ No harmonic data generated"
fi
