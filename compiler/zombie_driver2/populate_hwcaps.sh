#!/bin/bash
# Populate hwcaps paths script

set -e

echo "🔧 Populating hwcaps paths..."

# Read paths and create directories/copy files
while IFS= read -r line; do
    # Remove quotes and leading space
    path=$(echo "$line" | sed 's/^ *"//' | sed 's/"$//')
    
    # Skip if empty
    [ -z "$path" ] && continue
    
    # Get directory and filename
    dir=$(dirname "$path")
    file=$(basename "$path")
    
    echo "📁 Creating: $dir"
    mkdir -p "$dir"
    
    # Copy file if it exists in target/debug
    if [ -f "target/debug/$file" ]; then
        echo "📋 Copying: target/debug/$file -> $path"
        cp "target/debug/$file" "$path"
    else
        echo "⚠️  Missing: target/debug/$file"
    fi
done < hwcaps_paths.txt

echo "✅ Hwcaps population complete"
