#!/bin/bash
# Parallel crate scanning with enhanced scanner

echo "Starting parallel enhanced scan with 20 processes..."

# Create list of crates to scan
find ../.. -name "Cargo.toml" -path "*/compiler/*" | head -20 > crate_list.txt

# Function to scan a single crate
scan_crate() {
    local crate_path="$1"
    local crate_name=$(basename $(dirname "$crate_path"))
    local output_dir="scan_results_${crate_name}"
    
    echo "Scanning $crate_name..."
    mkdir -p "$output_dir"
    
    cd ../..
    RUSTC="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver/simple_scanner" \
    SCAN_OUTPUT_DIR="$output_dir" \
    cargo build -p "$crate_name" > "../compiler/rustc_driver/${output_dir}/build.log" 2>&1
    
    echo "Completed $crate_name"
}

export -f scan_crate

# Run parallel scanning
cat crate_list.txt | xargs -I {} -P 20 bash -c 'scan_crate "$@"' _ {}

echo "Parallel scan complete. Merging results..."

# Merge all scan_results_* directories
mkdir -p scan_results_merged
find . -name "scan_results_*" -type d | while read dir; do
    if [ "$dir" != "./scan_results_merged" ]; then
        cp -r "$dir"/* scan_results_merged/ 2>/dev/null || true
    fi
done

echo "Enhanced parallel scan complete. Results in scan_results_merged/"
