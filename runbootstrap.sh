#!/bin/bash

# Rust Bootstrap Audit Script
# This script runs the Rust bootstrap process with comprehensive tracing and logging

set -e

# Create audit directory with timestamp
AUDIT_DIR="audit_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$AUDIT_DIR"

echo "Starting Rust Bootstrap Audit - $(date)"
echo "Audit directory: $AUDIT_DIR"

# Save environment variables
echo "=== Environment Variables ===" > "$AUDIT_DIR/environment.txt"
env | sort >> "$AUDIT_DIR/environment.txt"

# Save system information
echo "=== System Information ===" > "$AUDIT_DIR/system_info.txt"
uname -a >> "$AUDIT_DIR/system_info.txt"
echo "" >> "$AUDIT_DIR/system_info.txt"
cat /etc/os-release >> "$AUDIT_DIR/system_info.txt" 2>/dev/null || echo "No /etc/os-release found" >> "$AUDIT_DIR/system_info.txt"
echo "" >> "$AUDIT_DIR/system_info.txt"
gcc --version >> "$AUDIT_DIR/system_info.txt" 2>/dev/null || echo "gcc not found" >> "$AUDIT_DIR/system_info.txt"
echo "" >> "$AUDIT_DIR/system_info.txt"
g++ --version >> "$AUDIT_DIR/system_info.txt" 2>/dev/null || echo "g++ not found" >> "$AUDIT_DIR/system_info.txt"

# Save Nix environment info
echo "=== Nix Environment ===" > "$AUDIT_DIR/nix_info.txt"
echo "LLVM_CONFIG: $LLVM_CONFIG" >> "$AUDIT_DIR/nix_info.txt"
echo "NIX_GLIBC_DEV: $NIX_GLIBC_DEV" >> "$AUDIT_DIR/nix_info.txt"
echo "NIX_GCC_PATH: $NIX_GCC_PATH" >> "$AUDIT_DIR/nix_info.txt"
echo "CPATH: $CPATH" >> "$AUDIT_DIR/nix_info.txt"
echo "CXXFLAGS: $CXXFLAGS" >> "$AUDIT_DIR/nix_info.txt"

# Test LLVM config
echo "=== LLVM Configuration ===" > "$AUDIT_DIR/llvm_config.txt"
if [ -n "$LLVM_CONFIG" ] && [ -x "$LLVM_CONFIG" ]; then
    echo "LLVM Config found at: $LLVM_CONFIG" >> "$AUDIT_DIR/llvm_config.txt"
    $LLVM_CONFIG --version >> "$AUDIT_DIR/llvm_config.txt" 2>&1 || echo "Failed to get LLVM version" >> "$AUDIT_DIR/llvm_config.txt"
    $LLVM_CONFIG --bindir >> "$AUDIT_DIR/llvm_config.txt" 2>&1 || echo "Failed to get LLVM bindir" >> "$AUDIT_DIR/llvm_config.txt"
    $LLVM_CONFIG --includedir >> "$AUDIT_DIR/llvm_config.txt" 2>&1 || echo "Failed to get LLVM includedir" >> "$AUDIT_DIR/llvm_config.txt"
    $LLVM_CONFIG --libdir >> "$AUDIT_DIR/llvm_config.txt" 2>&1 || echo "Failed to get LLVM libdir" >> "$AUDIT_DIR/llvm_config.txt"
    $LLVM_CONFIG --cppflags >> "$AUDIT_DIR/llvm_config.txt" 2>&1 || echo "Failed to get LLVM cppflags" >> "$AUDIT_DIR/llvm_config.txt"
    $LLVM_CONFIG --ldflags >> "$AUDIT_DIR/llvm_config.txt" 2>&1 || echo "Failed to get LLVM ldflags" >> "$AUDIT_DIR/llvm_config.txt"
else
    echo "LLVM Config not found or not executable: $LLVM_CONFIG" >> "$AUDIT_DIR/llvm_config.txt"
fi

# Save current bootstrap.toml
echo "=== Bootstrap Configuration ===" > "$AUDIT_DIR/bootstrap_config.txt"
if [ -f "bootstrap.toml" ]; then
    cat bootstrap.toml >> "$AUDIT_DIR/bootstrap_config.txt"
else
    echo "No bootstrap.toml found" >> "$AUDIT_DIR/bootstrap_config.txt"
fi

# Save directory structure
echo "=== Directory Structure ===" > "$AUDIT_DIR/directory_structure.txt"
find . -maxdepth 3 -type d | sort >> "$AUDIT_DIR/directory_structure.txt"

# Set up environment for build
export CXX="g++ -isystem ${NIX_GLIBC_DEV}/include"
export CC="gcc -isystem ${NIX_GLIBC_DEV}/include"

echo "Starting bootstrap build with strace..."
echo "Build started at: $(date)" > "$AUDIT_DIR/build_start.txt"

# Run the build with comprehensive strace
strace -f -o "$AUDIT_DIR/trace.txt" \
       -e trace=openat,open,stat,lstat,access,execve,clone,fork,vfork \
       ./x.py build --verbose > "$AUDIT_DIR/build_output.txt" 2> "$AUDIT_DIR/build_errors.txt"

BUILD_EXIT_CODE=$?

echo "Build finished at: $(date)" > "$AUDIT_DIR/build_end.txt"
echo "Exit code: $BUILD_EXIT_CODE" >> "$AUDIT_DIR/build_end.txt"

# Analyze the strace output for missing files
echo "=== Analyzing strace for missing files ===" > "$AUDIT_DIR/missing_files_analysis.txt"
echo "Looking for failed file access attempts..." >> "$AUDIT_DIR/missing_files_analysis.txt"

# Find failed openat/open calls
grep -E "(openat|open).*ENOENT" "$AUDIT_DIR/trace.txt" | head -50 >> "$AUDIT_DIR/missing_files_analysis.txt" 2>/dev/null || echo "No ENOENT errors found in trace" >> "$AUDIT_DIR/missing_files_analysis.txt"

# Look specifically for stdlib.h and other standard headers
echo "" >> "$AUDIT_DIR/missing_files_analysis.txt"
echo "=== Looking for standard library headers ===" >> "$AUDIT_DIR/missing_files_analysis.txt"
grep -i "stdlib.h\|stdio.h\|string.h\|cstdlib\|iostream" "$AUDIT_DIR/trace.txt" | head -20 >> "$AUDIT_DIR/missing_files_analysis.txt" 2>/dev/null || echo "No standard library header accesses found" >> "$AUDIT_DIR/missing_files_analysis.txt"

# Extract unique missing files
echo "" >> "$AUDIT_DIR/missing_files_analysis.txt"
echo "=== Unique missing files ===" >> "$AUDIT_DIR/missing_files_analysis.txt"
grep -E "(openat|open).*ENOENT" "$AUDIT_DIR/trace.txt" | sed 's/.*"\([^"]*\)".*/\1/' | sort -u | head -20 >> "$AUDIT_DIR/missing_files_analysis.txt" 2>/dev/null || echo "No missing files extracted" >> "$AUDIT_DIR/missing_files_analysis.txt"

# Find all g++ and gcc invocations
echo "=== Compiler Invocations ===" > "$AUDIT_DIR/compiler_invocations.txt"
grep -E "execve.*g\+\+|execve.*gcc" "$AUDIT_DIR/trace.txt" >> "$AUDIT_DIR/compiler_invocations.txt" 2>/dev/null || echo "No compiler invocations found" >> "$AUDIT_DIR/compiler_invocations.txt"

# Create summary report
echo "=== Bootstrap Audit Summary ===" > "$AUDIT_DIR/SUMMARY.txt"
echo "Date: $(date)" >> "$AUDIT_DIR/SUMMARY.txt"
echo "Exit Code: $BUILD_EXIT_CODE" >> "$AUDIT_DIR/SUMMARY.txt"
echo "Audit Directory: $AUDIT_DIR" >> "$AUDIT_DIR/SUMMARY.txt"
echo "" >> "$AUDIT_DIR/SUMMARY.txt"
echo "Files created:" >> "$AUDIT_DIR/SUMMARY.txt"
ls -la "$AUDIT_DIR/" >> "$AUDIT_DIR/SUMMARY.txt"
echo "" >> "$AUDIT_DIR/SUMMARY.txt"
echo "Trace file size: $(wc -l < "$AUDIT_DIR/trace.txt") lines" >> "$AUDIT_DIR/SUMMARY.txt"
echo "Build output size: $(wc -l < "$AUDIT_DIR/build_output.txt") lines" >> "$AUDIT_DIR/SUMMARY.txt"
echo "Build errors size: $(wc -l < "$AUDIT_DIR/build_errors.txt") lines" >> "$AUDIT_DIR/SUMMARY.txt"

echo ""
echo "Bootstrap audit completed!"
echo "Results saved in: $AUDIT_DIR"
echo "Exit code: $BUILD_EXIT_CODE"
echo ""
echo "Key files:"
echo "  - $AUDIT_DIR/SUMMARY.txt - Overview of the audit"
echo "  - $AUDIT_DIR/trace.txt - Complete strace output"
echo "  - $AUDIT_DIR/missing_files_analysis.txt - Analysis of missing files"
echo "  - $AUDIT_DIR/build_output.txt - Build stdout"
echo "  - $AUDIT_DIR/build_errors.txt - Build stderr"

exit $BUILD_EXIT_CODE
