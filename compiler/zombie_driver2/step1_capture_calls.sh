#!/bin/bash
# Step 1: Capture rustc parser function calls using self-profile

echo "🎯 RUSTC PARSER CALL CAPTURE PIPELINE"
echo "====================================="

# Create test Rust file to parse
cat > test_parse.rs << 'EOF'
fn main() {
    println!("Hello, world!");
    let x = 42;
    match x {
        42 => println!("Found it!"),
        _ => println!("Not found"),
    }
}
EOF

echo "📝 Created test_parse.rs"

# Step 1: Capture with rustc self-profile
echo "🔍 Step 1: Capturing parser calls with self-profile..."
rustc -Z self-profile=parser_trace test_parse.rs -o test_parse 2>&1 | tee rustc_output.log

# Check if profile data was generated
if [ -f "parser_trace.mm_profdata" ]; then
    echo "✅ Self-profile data captured: parser_trace.mm_profdata"
else
    echo "❌ No self-profile data found, trying alternative approach..."
    
    # Alternative: Use perf to capture function calls
    echo "🔄 Using perf as fallback..."
    perf record -g rustc test_parse.rs -o test_parse_perf 2>&1 | tee perf_output.log
    
    if [ -f "perf.data" ]; then
        echo "✅ Perf data captured: perf.data"
        perf script > perf_trace.txt
        echo "✅ Perf trace exported to perf_trace.txt"
    fi
fi

echo "📊 Step 1 complete - call data captured"
