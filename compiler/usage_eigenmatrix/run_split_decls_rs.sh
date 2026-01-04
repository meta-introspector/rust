#!/bin/bash

echo "🔍 Running collector on split-decls-rs with detailed output capture"
echo "=================================================="

cd ../../../split-decls-rs

echo "📊 Before collection:"
if [ -d "usage_data" ]; then
    echo "   Existing files: $(ls usage_data/*.json 2>/dev/null | wc -l)"
else
    echo "   No usage_data directory exists"
fi

echo ""
echo "🚀 Starting collection..."
echo "Using collector: /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector"

CFG_RELEASE_CHANNEL=dev RUSTC_INSTALL_BINDIR=/usr/bin RUSTC=/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector cargo build --workspace --all-targets 2>&1 | tee /tmp/split_decls_rs_full_output.log

echo ""
echo "📊 After collection:"
if [ -d "usage_data" ]; then
    echo "   Total files: $(ls usage_data/*.json 2>/dev/null | wc -l)"
    echo "   Directory size: $(du -sh usage_data 2>/dev/null | cut -f1)"
    echo "   Sample files:"
    ls usage_data/*.json 2>/dev/null | head -5 | while read file; do
        size=$(wc -c < "$file" 2>/dev/null || echo "0")
        echo "     $(basename "$file"): $size bytes"
    done
else
    echo "   No usage_data directory created"
fi

echo ""
echo "📝 Output log saved to: /tmp/split_decls_rs_output.log"
echo "✅ Collection complete"
