#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Test all codegen backends with strace

set -e

# Build zombie first
./build_zombie.sh

# Create test victim
echo 'fn main() { println!("Hello zombie!"); }' > test_victim.rs

# List of backends to test
BACKENDS=(
    "llvm"
    "cranelift" 
    "gcc"
    ""  # default
)

# Create logs directory
mkdir -p backend_logs

echo "🧟 Testing codegen backends with strace..."

for backend in "${BACKENDS[@]}"; do
    if [ -z "$backend" ]; then
        backend_name="default"
        backend_flag=""
    else
        backend_name="$backend"
        backend_flag="-Z codegen-backend=$backend"
    fi
    
    echo "🔍 Testing backend: $backend_name"
    
    # Run with strace
    timeout 30 strace -f -e trace=openat,stat -o "backend_logs/strace_${backend_name}.log" \
        ./zombie.sh --no-net --diagnose --sysroot=/nix/store/i6xakg19vy8vc2g211yr9d5nmb0wk7v0-rustc-1.91.1 \
        $backend_flag test_victim.rs --crate-type bin \
        > "backend_logs/output_${backend_name}.log" 2>&1 || true
    
    # Analyze results
    echo "📋 Backend $backend_name results:"
    echo "   Exit code: $(tail -1 backend_logs/output_${backend_name}.log | grep -o 'exit code: [0-9]*' || echo 'unknown')"
    
    # Look for missing codegen files
    missing_codegen=$(grep -E "(codegen.*ENOENT|librustc_codegen.*ENOENT)" "backend_logs/strace_${backend_name}.log" | wc -l)
    echo "   Missing codegen files: $missing_codegen"
    
    # Look for backend-specific errors
    if grep -q "unsupported builtin codegen backend" "backend_logs/output_${backend_name}.log"; then
        echo "   Status: ❌ Backend not supported"
    elif grep -q "error:" "backend_logs/output_${backend_name}.log"; then
        echo "   Status: ⚠️  Compilation error"
    else
        echo "   Status: ✅ Backend available"
    fi
    
    echo ""
done

echo "🧠 Analysis complete. Logs saved in backend_logs/"
echo "📄 Check individual logs:"
ls -la backend_logs/
