#!/bin/bash
# Collect strace logs from zombie driver compilation

cd compiler/zombie_driver2
export LD_LIBRARY_PATH="target/debug:$LD_LIBRARY_PATH"

echo "🔍 Compiling test.rs with zombie driver under strace..."
strace -o zombie_compile_strace.log -f -e trace=all ./target/debug/zombie_rustc_driver test.rs -o test_zombie 2>&1 | tee zombie_compile_output.log

echo "📊 Compilation strace analysis:"
echo "Total syscalls: $(wc -l < zombie_compile_strace.log)"
echo "Network calls: $(grep -c -E "(socket|bind|listen|accept|connect)" zombie_compile_strace.log)"
echo "File operations: $(grep -c -E "(openat|read|write)" zombie_compile_strace.log)"
echo "Process operations: $(grep -c -E "(clone|fork|exec)" zombie_compile_strace.log)"

echo "🧟 Network activity:"
grep -E "(socket|bind|listen)" zombie_compile_strace.log | head -3
