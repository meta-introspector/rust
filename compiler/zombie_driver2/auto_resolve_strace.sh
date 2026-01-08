#!/bin/bash

echo "🔧 Auto-resolving strace build errors..."
echo "========================================"

DEPS_DIR="compiler/zombie_driver2/target/debug/deps"
mkdir -p "$DEPS_DIR"

# Function to find and symlink missing files
resolve_missing_file() {
    local missing_file="$1"
    local search_pattern="$2"
    
    echo "🔍 Looking for: $missing_file"
    
    # First try to find exact match in current deps
    if [ -f "$DEPS_DIR/$missing_file" ]; then
        echo "✅ Already exists: $missing_file"
        return 0
    fi
    
    # Try to find similar file in deps directory
    local found_file=$(ls "$DEPS_DIR" 2>/dev/null | grep "$search_pattern" | head -1)
    if [ -n "$found_file" ]; then
        echo "🔗 Creating symlink: $found_file -> $missing_file"
        cd "$DEPS_DIR" && ln -sf "$found_file" "$missing_file"
        return 0
    fi
    
    # Try to find in nix store
    local nix_file=$(find /nix/store -name "$missing_file" 2>/dev/null | head -1)
    if [ -n "$nix_file" ]; then
        echo "🔗 Creating symlink from nix store: $nix_file -> $missing_file"
        cd "$DEPS_DIR" && ln -sf "$nix_file" "$missing_file"
        return 0
    fi
    
    # Try to find similar pattern in build directory
    local build_file=$(find build/ -name "*$search_pattern*" 2>/dev/null | head -1)
    if [ -n "$build_file" ]; then
        echo "🔗 Creating symlink from build: $build_file -> $missing_file"
        cd "$DEPS_DIR" && ln -sf "$(realpath "$build_file")" "$missing_file"
        return 0
    fi
    
    echo "❌ Could not find: $missing_file"
    return 1
}

# Run strace build and capture missing files
echo "🔍 Running strace to detect missing files..."
cd compiler/zombie_driver2
strace_output=$(strace -e trace=openat -f cargo build 2>&1 | grep "ENOENT" | grep -E "(rustc_driver|rustc_interface|rustc_middle)")

# Parse strace output for missing files
missing_files=$(echo "$strace_output" | grep -o '/[^"]*\(rustc_driver\|rustc_interface\|rustc_middle\)[^"]*' | sed 's|.*/||' | sort -u)

if [ -z "$missing_files" ]; then
    echo "ℹ️  No missing rustc files detected in strace output"
    exit 0
fi

echo "📋 Missing files detected:"
echo "$missing_files"
echo ""

# Resolve each missing file
for file in $missing_files; do
    case "$file" in
        *rustc_driver*.so)
            resolve_missing_file "$file" "rustc_driver.*\.so"
            ;;
        *rustc_interface*.rlib)
            resolve_missing_file "$file" "rustc_interface.*\.rlib"
            ;;
        *rustc_middle*.rlib)
            resolve_missing_file "$file" "rustc_middle.*\.rlib"
            ;;
        *)
            echo "⚠️  Unknown file pattern: $file"
            ;;
    esac
done

echo ""
echo "🔨 Attempting build after resolving symlinks..."
cargo build

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build still failing, may need manual intervention"
fi
