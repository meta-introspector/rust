#!/bin/bash
# Full rustc build with internal crate scanning

echo "Starting full rustc build with internal crate scanning..."

# Set our scanner as rustc replacement
export RUSTC="$(pwd)/simple_scanner"

# Build rustc itself to capture internal crates
echo "Building rustc with internal crate capture..."
cd ../../..
cargo build --bin rustc 2>&1 | tee ../rustc_driver/internal_build_$(date +%Y%m%d_%H%M%S).log

echo "Internal rustc build complete."
