#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Zombie Strace Diagnostics - Find missing libraries

set -e

export RUSTC_WRAPPER=~/.cargo/bin/sccache
export CFG_COMPILER_HOST_TRIPLE=x86_64-unknown-linux-gnu
export LD_LIBRARY_PATH=target/debug:$LD_LIBRARY_PATH

# Build if needed
if [ ! -f target/debug/zombie_rustc_driver ]; then
    echo "🧟 Building zombie driver..."
    cargo build
fi

# Create test victim
echo 'fn main() { println!("Hello zombie!"); }' > test_victim.rs

echo "🔍 Running zombie with strace diagnostics..."
echo "📋 Tracing file operations to find missing libraries..."

# Run with strace in diagnose mode
strace -f -e trace=openat,stat,access -o zombie_strace.log \
    ./target/debug/zombie_rustc_driver --diagnose --no-net test_victim.rs --crate-type bin 2>&1

echo "🧠 Strace complete. Analyzing results..."

# Search for missing files
echo "❌ Missing files:"
grep -E "(ENOENT|No such file)" zombie_strace.log | grep -v "/proc\|/sys\|/dev" | head -10

echo ""
echo "🔍 Codegen backend searches:"
grep -E "(codegen|rustlib)" zombie_strace.log | head -10

echo ""
echo "📚 Library searches:"
grep -E "\.so.*ENOENT" zombie_strace.log | head -10

echo ""
echo "🎯 Sysroot searches:"
grep -E "(sysroot|rustlib.*lib)" zombie_strace.log | head -5

echo ""
echo "📄 Full strace log saved to: zombie_strace.log"
echo "🧟 Use: grep 'pattern' zombie_strace.log to search for specific issues"
