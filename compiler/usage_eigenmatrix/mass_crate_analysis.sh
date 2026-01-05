#!/bin/bash

# Mass crate analysis with harmonic compiler
echo "=== MASS CRATE HARMONIC ANALYSIS ==="

# Setup
export USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data"
export HARMONIC_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/harmonic_data"
export RUSTC_VERSION="1.74.0"
export ENABLE_HARMONIC_ANALYSIS=1

mkdir -p "$USAGE_OUTPUT_DIR" "$HARMONIC_OUTPUT_DIR"

# Build harmonic collector
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix
cargo build --bin version_stable_compiler --bin practical_harmonic_compiler
cd ../introspector-collector
cargo build --bin working_usage_collector

export RUSTC="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector"

# Popular crates to analyze
CRATES=(
    "serde"
    "tokio" 
    "clap"
    "regex"
    "syn"
    "quote"
    "proc-macro2"
    "anyhow"
    "thiserror"
    "log"
    "env_logger"
    "chrono"
    "uuid"
    "rand"
    "itertools"
    "rayon"
    "crossbeam"
    "parking_lot"
    "dashmap"
    "reqwest"
)

# Create temp workspace
TEMP_DIR="/tmp/harmonic_crate_analysis"
rm -rf "$TEMP_DIR"
mkdir -p "$TEMP_DIR"
cd "$TEMP_DIR"

# Initialize cargo workspace
cat > Cargo.toml << 'EOF'
[workspace]
members = []
resolver = "2"
EOF

echo "=== ANALYZING ${#CRATES[@]} POPULAR CRATES ==="

for crate_name in "${CRATES[@]}"; do
    echo "--- Analyzing $crate_name ---"
    
    # Create project for this crate
    cargo new "$crate_name" --lib
    cd "$crate_name"
    
    # Add the crate as dependency
    cargo add "$crate_name" 2>/dev/null || {
        echo "⚠ Could not add $crate_name, skipping..."
        cd ..
        continue
    }
    
    # Create simple usage to trigger analysis
    cat > src/lib.rs << EOF
// Harmonic analysis trigger for $crate_name
#[allow(unused_imports)]
use $crate_name::*;

pub fn analyze_$crate_name() {
    // This will trigger the harmonic collector
}
EOF
    
    # Run harmonic analysis
    echo "  Running harmonic compilation..."
    timeout 60s cargo check 2>/dev/null || echo "  ⚠ Timeout or error for $crate_name"
    
    cd ..
done

echo "=== GENERATING AGGREGATE HARMONIC ANALYSIS ==="

cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix

# Create aggregate analysis tool
cat > src/bin/aggregate_harmonic_analysis.rs << 'EOF'
use std::collections::HashMap;
use serde_json::{json, Value};
use std::fs;

fn main() {
    println!("=== AGGREGATE HARMONIC ANALYSIS ===");
    
    let usage_dir = std::env::var("USAGE_OUTPUT_DIR")
        .unwrap_or_else(|_| "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data".to_string());
    
    let mut crate_stats = HashMap::new();
    let mut total_constructs = 0;
    let mut total_harmonics = 0;
    
    // Scan all usage files
    if let Ok(entries) = fs::read_dir(&usage_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") && !filename.contains("_enums_") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(data) = serde_json::from_str::<Value>(&content) {
                            if let Some(crate_name) = data.get("crate").and_then(|v| v.as_str()) {
                                if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                                    let count = usages.len();
                                    *crate_stats.entry(crate_name.to_string()).or_insert(0) += count;
                                    total_constructs += count;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("--- CRATE ANALYSIS RESULTS ---");
    let mut sorted_crates: Vec<_> = crate_stats.iter().collect();
    sorted_crates.sort_by(|a, b| b.1.cmp(a.1));
    
    for (crate_name, count) in sorted_crates.iter().take(20) {
        println!("  {}: {} constructs", crate_name, count);
    }
    
    println!("\n--- SUMMARY ---");
    println!("Total crates analyzed: {}", crate_stats.len());
    println!("Total constructs: {}", total_constructs);
    println!("Average constructs per crate: {:.1}", 
             total_constructs as f64 / crate_stats.len() as f64);
    
    // Generate aggregate report
    let report = json!({
        "analysis_type": "aggregate_harmonic_analysis",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total_crates": crate_stats.len(),
        "total_constructs": total_constructs,
        "crate_breakdown": crate_stats,
        "top_crates": sorted_crates.iter().take(10).map(|(name, count)| {
            json!({"crate": name, "constructs": count})
        }).collect::<Vec<_>>()
    });
    
    let report_path = format!("{}/aggregate_harmonic_report.json", 
                             std::env::var("HARMONIC_OUTPUT_DIR")
                                 .unwrap_or_else(|_| usage_dir));
    
    if let Ok(report_json) = serde_json::to_string_pretty(&report) {
        fs::write(&report_path, report_json).unwrap();
        println!("✓ Aggregate report saved to: {}", report_path);
    }
}
EOF

# Add chrono dependency for timestamps
echo 'chrono = { version = "0.4", features = ["serde"] }' >> Cargo.toml

# Build and run aggregate analysis
cargo build --bin aggregate_harmonic_analysis
./target/debug/aggregate_harmonic_analysis

echo "=== MASS ANALYSIS COMPLETE ==="
echo "Usage data: $USAGE_OUTPUT_DIR"
echo "Harmonic data: $HARMONIC_OUTPUT_DIR"
echo "Temp workspace: $TEMP_DIR"

# Show final statistics
echo "--- FINAL STATISTICS ---"
usage_files=$(ls "$USAGE_OUTPUT_DIR"/*.json 2>/dev/null | wc -l)
harmonic_files=$(ls "$HARMONIC_OUTPUT_DIR"/*.json 2>/dev/null | wc -l)

echo "Usage files generated: $usage_files"
echo "Harmonic files generated: $harmonic_files"
echo "Crates analyzed: ${#CRATES[@]}"

# Cleanup
echo "Cleaning up temp directory..."
rm -rf "$TEMP_DIR"

echo "✓ Mass crate harmonic analysis complete!"
