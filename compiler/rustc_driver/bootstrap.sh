#!/bin/bash

# Bootstrap build using our custom scanner as rustc driver
export RUSTC="$(pwd)/simple_scanner"
export RUSTC_WRAPPER=""

echo "Starting bootstrap build with custom scanner..."
echo "RUSTC=$RUSTC"

# Build the entire rustc project using our scanner
cd ../..
cargo build --verbose 2>&1 | tee bootstrap_scan.log

echo "Bootstrap scan complete. Results in bootstrap_scan.log"
