#!/bin/bash
# Zombie Cleanup Script

echo "🧹 Cleaning zombie artifacts..."

# Remove build artifacts
rm -rf target/
echo "   Removed target/"

# Remove logs
rm -f *.log
rm -rf backend_logs/
echo "   Removed logs"

# Remove test files
rm -f test_victim.rs
echo "   Removed test files"

# Clean cargo cache
cargo clean 2>/dev/null || true
echo "   Cleaned cargo cache"

echo "✅ Zombie cleanup complete"
