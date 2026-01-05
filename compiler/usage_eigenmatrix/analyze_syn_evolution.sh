#!/bin/bash

# Analyze syn crate evolution using our prime-complexity analyzer
echo "=== Syn Crate Prime-Complexity Analysis ==="

SYN_REPO_DIR="/tmp/syn_analysis"
SYN_VERSIONS=("1.0.0" "1.0.109" "2.0.0" "2.0.106")
ANALYZER_PATH="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix/target/debug/syn_prime_analyzer"

# Clone syn repo if not exists
if [ ! -d "$SYN_REPO_DIR" ]; then
    echo "Cloning syn repository..."
    git clone https://github.com/dtolnay/syn.git "$SYN_REPO_DIR"
fi

cd "$SYN_REPO_DIR"

echo "Building our analyzer first..."
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix
cargo build --bin syn_prime_analyzer

for version in "${SYN_VERSIONS[@]}"; do
    echo "--- Analyzing syn version $version ---"
    
    cd "$SYN_REPO_DIR"
    git checkout "$version" 2>/dev/null || {
        echo "Skipping $version - not found"
        continue
    }
    
    # Create output directory
    OUTPUT_DIR="/tmp/syn_analysis_results/syn_$version"
    mkdir -p "$OUTPUT_DIR"
    
    # Find Rust files in src/
    echo "Finding Rust files in syn $version..."
    find src/ -name "*.rs" -type f | head -10
    
    # Copy our analyzer to work with syn files
    echo "Analyzing syn $version structure..."
    
    # Create a simple analysis script for this version
    cat > analyze_syn_version.rs << 'EOF'
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Syn Crate Analysis");
    println!("═══════════════════════");
    
    let src_files: Vec<_> = fs::read_dir("src")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "rs")
                .unwrap_or(false)
        })
        .take(5)
        .collect();
    
    println!("Found {} Rust files:", src_files.len());
    for entry in src_files {
        let path = entry.path();
        let size = fs::metadata(&path)?.len();
        println!("  {}: {} bytes", path.display(), size);
    }
    
    Ok(())
}
EOF
    
    # Compile and run the analysis
    rustc analyze_syn_version.rs -o analyze_syn_version
    ./analyze_syn_version > "$OUTPUT_DIR/analysis.txt" 2>&1
    
    # Save version info
    echo "Version: $version" > "$OUTPUT_DIR/version_info.txt"
    echo "Date: $(date)" >> "$OUTPUT_DIR/version_info.txt"
    echo "Files analyzed: $(find src/ -name "*.rs" | wc -l)" >> "$OUTPUT_DIR/version_info.txt"
    
    echo "Results saved to $OUTPUT_DIR"
done

echo "=== Analysis Complete ==="
echo "Results in /tmp/syn_analysis_results/"
