#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Single shot: populate sysroot, run strace, scan output

set -e

echo "🔄 Single shot test cycle..."

# 1. Populate sysroot
echo "📦 Step 1: Populating sysroot..."
./populate_sysroot.sh > /dev/null 2>&1

# 2. Run zombie with strace
echo "🔍 Step 2: Running zombie with strace..."
strace -f -e trace=file -o zombie_single_shot.log ./zombie.sh --no-net hello_zombie.rs --crate-type bin -o hello_zombie 2>&1 | head -5

# 3. Scan for NOENT errors
echo "📊 Step 3: Scanning for missing files..."

# Save all NOENT errors to file
grep NOENT zombie_single_shot.log > noent_errors.txt 2>/dev/null || touch noent_errors.txt

# Report target-related NOENT errors
target_noent=$(grep target noent_errors.txt | wc -l)
echo "   Target NOENT errors: $target_noent"

if [ "$target_noent" -gt 0 ]; then
    echo "📋 Missing target files:"
    grep target noent_errors.txt | cut -d, -f2- | grep -o '"/[^"]*"' | sed 's/"//g' | while read path; do
        echo "   ❌ $path"
    done
else
    echo "✅ No missing target files found!"
fi

echo "🏁 Single shot complete"
