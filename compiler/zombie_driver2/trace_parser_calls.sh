#!/bin/bash

# Profile rustc parser function calls
echo "🔬 RUSTC PARSER CALL TRACING"
echo "============================"

# Create test Rust file
cat > test_parse.rs << 'EOF'
fn main() {
    println!("Hello parser trace!");
}
EOF

echo "📝 Created test file: test_parse.rs"

# Method 1: Use rustc self-profile
echo "🎯 Method 1: Self-profile tracing..."
RUSTC_PROFILE=1 rustc --self-profile=parser_trace test_parse.rs -o test_parse 2>&1 | tee self_profile.log

# Method 2: Use perf to trace function calls
echo "🎯 Method 2: Perf function tracing..."
perf record -g --call-graph dwarf rustc test_parse.rs -o test_parse_perf 2>&1
perf script > perf_trace.log

# Method 3: Use our zombie rustc with tracing
echo "🎯 Method 3: Zombie rustc tracing..."
if [ -f "./zombie.sh" ]; then
    strace -f -e trace=write,openat ./zombie.sh test_parse.rs -o test_parse_zombie 2>&1 | tee zombie_trace.log
fi

echo "📊 Analysis Results:"
echo "   Self-profile: self_profile.log"
echo "   Perf trace: perf_trace.log" 
echo "   Zombie trace: zombie_trace.log"

# Extract parser function calls
echo "🔍 Parser function calls found:"
grep -E "(parse|Parse)" *.log | head -10
