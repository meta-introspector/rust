#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Iterative strace and fix script

set -e

iteration=1
max_iterations=10

while [ $iteration -le $max_iterations ]; do
    echo "🔍 Iteration $iteration: Running zombie with strace..."
    
    # Run zombie with strace
    strace -f -e trace=file -o "zombie_iter_${iteration}_strace.log" ./zombie.sh --no-net hello_zombie.rs --crate-type bin -o hello_zombie 2>&1 | head -10
    
    # Check for NOENT errors
    missing_files=$(grep NOENT "zombie_iter_${iteration}_strace.log" | grep target | head -5)
    
    if [ -z "$missing_files" ]; then
        echo "✅ No missing target files found!"
        break
    fi
    
    echo "📋 Missing files found:"
    echo "$missing_files"
    echo ""
    
    # Extract first missing file path
    first_missing=$(echo "$missing_files" | head -1 | grep -o '"/[^"]*"' | sed 's/"//g')
    
    if [ -n "$first_missing" ]; then
        echo "🔧 Fixing: $first_missing"
        
        # Create directory
        mkdir -p "$(dirname "$first_missing")"
        
        # Get filename
        filename=$(basename "$first_missing")
        
        # Try to create/copy the file
        if [[ "$filename" == *.rs ]]; then
            echo "// Dummy $filename" > "$first_missing"
            echo "✅ Created dummy $filename"
        elif [[ "$filename" == *.so ]]; then
            # Try to find and copy .so file
            if [ -f "target/debug/$filename" ]; then
                cp "target/debug/$filename" "$first_missing"
                echo "✅ Copied $filename"
            else
                echo "⚠️  Could not find $filename"
            fi
        elif [[ "$first_missing" == *"codegen-backends" ]]; then
            # Copy codegen backends
            if [ -d "target/debug/codegen-backends" ]; then
                cp target/debug/codegen-backends/* "$first_missing/"
                echo "✅ Copied codegen backends"
            fi
        else
            # Create directory or dummy file
            if [[ "$first_missing" == */ ]]; then
                mkdir -p "$first_missing"
                echo "✅ Created directory"
            else
                touch "$first_missing"
                echo "✅ Created empty file"
            fi
        fi
    fi
    
    echo ""
    iteration=$((iteration + 1))
done

if [ $iteration -gt $max_iterations ]; then
    echo "⚠️  Reached maximum iterations ($max_iterations)"
else
    echo "🎉 Fixed all missing files in $((iteration-1)) iterations!"
fi
