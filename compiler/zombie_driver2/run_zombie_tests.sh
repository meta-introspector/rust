#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Zombie Rustc Driver Test Suite

set -e

# Build zombie driver with sccache
echo "🧟 Building zombie driver..."
export RUSTC_WRAPPER=~/.cargo/bin/sccache
cargo build

# Setup environment
export CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu
export LD_LIBRARY_PATH=target/debug:$LD_LIBRARY_PATH

# Create test victim
echo 'fn main() { println!("Hello zombie!"); }' > test_victim.rs

echo "🧟♂️ Running zombie tests..."

# Test 1: Network mode only
echo "Test 1: Network mode"
timeout 3 ./target/debug/zombie_rustc_driver --network-mode &
sleep 1
pkill -f zombie_rustc_driver || true

# Test 2: No network compilation
echo "Test 2: No network compilation"
./target/debug/zombie_rustc_driver --no-net test_victim.rs --crate-type bin

# Test 3: Diagnose mode
echo "Test 3: Diagnose mode"  
./target/debug/zombie_rustc_driver --diagnose test_victim.rs --crate-type bin

# Test 4: Full network compilation
echo "Test 4: Network compilation"
timeout 5 ./target/debug/zombie_rustc_driver test_victim.rs --crate-type bin || true

echo "🧠 Zombie tests complete"
