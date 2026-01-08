#!/bin/bash

echo "🔧 Processing strace logs to fix missing files..."
echo "=============================================="

STRACE_FILE="compiler/zombie_driver2/strace.log"
DEPS_DIR="compiler/zombie_driver2/target/debug/deps"

if [ ! -f "$STRACE_FILE" ]; then
    echo "❌ Strace file not found: $STRACE_FILE"
    exit 1
fi

# Extract missing rustc files from strace
missing_files=$(grep "ENOENT" "$STRACE_FILE" | grep -E "(rustc_driver|rustc_interface|rustc_middle)" | grep -o '/[^"]*\(rustc_driver\|rustc_interface\|rustc_middle\)[^"]*' | sed 's|.*/||' | sort -u)

echo "📋 Missing files found:"
echo "$missing_files"
echo ""

# Process each missing file
for file in $missing_files; do
    echo "🔍 Processing: $file"
    
    case "$file" in
        *rustc_driver*.so)
            # Check if it exists in nix store
            nix_file="/nix/store/i6xakg19vy8vc2g211yr9d5nmb0wk7v0-rustc-1.91.1/lib/$file"
            if [ -f "$nix_file" ]; then
                echo "✅ Creating symlink from nix store: $file"
                cd "$DEPS_DIR" && ln -sf "$nix_file" "$file"
            else
                echo "❌ File not found in nix store: $file"
            fi
            ;;
        *rustc_interface*.rlib|*rustc_middle*.rlib)
            # Find similar file in deps directory
            base_name=$(echo "$file" | sed 's/-[a-f0-9]\{16\}\.rlib$//')
            found_file=$(ls "$DEPS_DIR" 2>/dev/null | grep "^${base_name}-[a-f0-9]\{16\}\.rlib$" | head -1)
            if [ -n "$found_file" ]; then
                echo "✅ Creating symlink: $found_file -> $file"
                cd "$DEPS_DIR" && ln -sf "$found_file" "$file"
            else
                echo "❌ No matching file found for: $file"
            fi
            ;;
        *)
            echo "⚠️  Unknown file pattern: $file"
            ;;
    esac
done

echo ""
echo "🔨 Testing build after fixing symlinks..."
cd compiler/zombie_driver2 && ./build_zombie.sh
