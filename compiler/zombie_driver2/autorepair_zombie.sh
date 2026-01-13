#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Zombie Autorepair Script - Fixes missing .so files from strace logs

set -e

echo "🔧 Zombie Autorepair System"
echo "=========================="

# Function to find and copy a library
copy_library() {
    local lib_name="$1"
    local target_path="$2"
    
    echo "🔍 Searching for $lib_name..."
    
    # Try multiple search methods
    local found_lib=""
    
    # Method 1: locate command
    if command -v locate >/dev/null 2>&1; then
        found_lib=$(locate "$lib_name" 2>/dev/null | head -1)
    fi
    
    # Method 2: find in common locations
    if [ -z "$found_lib" ]; then
        for search_path in "/nix/store" "/usr/lib" "/lib" "/usr/local/lib"; do
            if [ -d "$search_path" ]; then
                found_lib=$(find "$search_path" -name "$lib_name" 2>/dev/null | head -1)
                [ -n "$found_lib" ] && break
            fi
        done
    fi
    
    if [ -n "$found_lib" ] && [ -f "$found_lib" ]; then
        echo "✅ Found: $found_lib"
        mkdir -p "$(dirname "$target_path")"
        cp "$found_lib" "$target_path"
        echo "📋 Copied to: $target_path"
        return 0
    else
        echo "❌ Not found: $lib_name"
        return 1
    fi
}

# Create target directories
mkdir -p target/debug/glibc-hwcaps/x86-64-v3
mkdir -p target/debug/glibc-hwcaps/x86-64-v2

# Extract missing libraries from strace logs
echo "📊 Analyzing strace logs..."
missing_libs=$(grep -h "target/debug.*\.so.*ENOENT" *.log backend_logs/*.log 2>/dev/null | \
    grep -o 'target/debug/[^"]*\.so[^"]*' | \
    sort -u)

if [ -z "$missing_libs" ]; then
    echo "ℹ️  No missing target/debug libraries found in strace logs"
    exit 0
fi

echo "🎯 Missing libraries found:"
echo "$missing_libs"
echo ""

# Process each missing library
fixed_count=0
total_count=0

while IFS= read -r missing_lib; do
    [ -z "$missing_lib" ] && continue
    total_count=$((total_count + 1))
    
    # Extract just the library name
    lib_name=$(basename "$missing_lib")
    
    echo "🔧 Processing: $missing_lib"
    
    if copy_library "$lib_name" "$missing_lib"; then
        fixed_count=$((fixed_count + 1))
    fi
    echo ""
done <<< "$missing_libs"

# Summary
echo "📈 Repair Summary:"
echo "   Total missing: $total_count"
echo "   Fixed: $fixed_count"
echo "   Remaining: $((total_count - fixed_count))"

if [ $fixed_count -gt 0 ]; then
    echo "✅ Autorepair completed successfully!"
    echo "🧟 Try running your zombie driver again"
else
    echo "⚠️  No libraries could be automatically fixed"
    echo "💡 You may need to build missing libraries manually"
fi
