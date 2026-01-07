#!/bin/bash

echo "🧟 === Zombie LibP2P Compiler Driver Network ==="

# Build the zombie driver first
echo "=== Building zombie_rustc_driver ==="
cargo build --bin zombie_rustc_driver -j 20

echo "=== Storing zombie spore ==="
mkdir -p ./zombie_store
cp ../../target/debug/zombie_rustc_driver ./zombie_store/

echo "=== Cleaning for zombie infection ==="
cargo clean

# Create zombie network directories
mkdir -p ./zombie_network
mkdir -p ./zombie_output

echo "🌐 === Phase 1: Zombie Self-Replication ==="
USAGE_OUTPUT_DIR="./zombie_output" \
RUSTC="$(pwd)/zombie_store/zombie_rustc_driver" \
cargo build --bin zombie_rustc_driver -j 20

echo "🧠 === Phase 2: Infecting Core Components ==="
USAGE_OUTPUT_DIR="./zombie_output" \
RUSTC="$(pwd)/zombie_store/zombie_rustc_driver" \
cargo build -p rustc_middle -p rustc_hir -j 20

echo "📡 === Starting LibP2P Zombie Network ==="
./zombie_store/zombie_rustc_driver --network-mode &
ZOMBIE_PID=$!

echo "🧟‍♂️ === Zombie network PID: $ZOMBIE_PID ==="
echo "Kill with: kill $ZOMBIE_PID"
