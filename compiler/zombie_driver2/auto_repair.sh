#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Auto repair missing files from strace (DRY RUN)

set -e

echo "🔧 Auto-repairing missing files from strace (DRY RUN)..."

# Extract missing .so files from latest strace
missing_files=$(grep "ENOENT" zombie_compile_strace.log | grep -E "\.(so|rmeta)" | grep -o '"[^"]*"' | sed 's/"//g' | sort -u)

if [ -z "$missing_files" ]; then
    echo "✅ No missing files found"
    exit 0
fi

echo "📋 Missing files found:"
echo "$missing_files"
echo ""

failed=0
fixed=0

while IFS= read -r missing_file; do
    [ -z "$missing_file" ] && continue
    
    echo "🔍 Processing: $missing_file"
    
    # Get filename
    filename=$(basename "$missing_file")
    
    # Create directory
    echo "📁 Would create: $(dirname "$missing_file")"
    
    # Try to find and copy the file
    found=false
    
    # Check if it exists in target/debug
    if [ -f "target/debug/$filename" ]; then
        echo "✅ Would copy from target/debug/$filename"
        fixed=$((fixed + 1))
        found=true
    # Check in nix store
    elif nix_file=$(find /nix/store -name "$filename" 2>/dev/null | head -1); then
        if [ -n "$nix_file" ] && [ -f "$nix_file" ]; then
            echo "✅ Would copy from $nix_file"
            fixed=$((fixed + 1))
            found=true
        fi
    fi
    
    if [ "$found" = false ]; then
        echo "❌ FAILED: Cannot find $filename"
        failed=$((failed + 1))
    fi
    
done <<< "$missing_files"

echo ""
echo "📊 Repair Summary:"
echo "   Fixed: $fixed"
echo "   Failed: $failed"

if [ $failed -gt 0 ]; then
    echo "💥 Auto-repair failed - $failed files could not be found"
    exit 1
else
    echo "✅ Auto-repair completed successfully"
    exit 0
fi
