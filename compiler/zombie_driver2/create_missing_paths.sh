#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Create missing target paths from strace

echo "🔧 Creating missing target paths..."

# Extract unique missing target paths
missing_paths=$(grep ENT zom*.log | cut -d, -f2- | grep target | grep -o '"[^"]*"' | sed 's/"//g' | sort -u)

echo "📋 Missing paths found:"
echo "$missing_paths"
echo ""

while IFS= read -r path; do
    [ -z "$path" ] && continue
    
    echo "📁 Creating directory for: $path"
    mkdir -p "$(dirname "$path")"
    
    filename=$(basename "$path")
    
    # Try to copy from existing locations
    if [ -f "target/debug/$filename" ]; then
        echo "📋 Copying: target/debug/$filename -> $path"
        cp "target/debug/$filename" "$path"
    elif [ -f "target/debug/codegen-backends/$filename" ]; then
        echo "📋 Copying: target/debug/codegen-backends/$filename -> $path"
        cp "target/debug/codegen-backends/$filename" "$path"
    else
        echo "⚠️  File not found: $filename"
    fi
    
done <<< "$missing_paths"

echo "✅ Path creation complete"
