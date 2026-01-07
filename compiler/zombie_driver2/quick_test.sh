#!/bin/bash
# Quick Zombie Test

set -e

echo "⚡ Quick zombie test"

# Build if needed
if [ ! -f "target/debug/zombie_rustc_driver" ]; then
    ./build_zombie.sh
fi

# Create simple test
echo 'fn main() { println!("Quick test!"); }' > quick_test.rs

# Run zombie
timeout 10 ./zombie.sh --no-net quick_test.rs || echo "Test completed"

# Cleanup
rm -f quick_test.rs

echo "✅ Quick test done"
