#!/bin/bash

# Build and test the live parser interceptor

set -e

echo "🧟 Building Zombie Parser Interceptor..."

# Create interceptor library directory
mkdir -p parser_interceptor/src

# Create Cargo.toml for the interceptor
cat > parser_interceptor/Cargo.toml << 'EOF'
[package]
name = "parser_interceptor"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
libc = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ctor = "0.2"
EOF

# Copy the interceptor source
cp live_parser_interceptor.rs parser_interceptor/src/lib.rs

# Build the interceptor library
echo "📦 Building interceptor library..."
cd parser_interceptor
cargo build --release
cd ..

INTERCEPTOR_LIB="parser_interceptor/target/release/libparser_interceptor.so"

if [ ! -f "$INTERCEPTOR_LIB" ]; then
    echo "❌ Failed to build interceptor library"
    exit 1
fi

echo "✅ Interceptor built: $INTERCEPTOR_LIB"

# Create test Rust file
cat > test_parsing.rs << 'EOF'
// Test file for parser interception
use std::collections::HashMap;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

struct Calculator {
    memory: HashMap<String, f64>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }
    
    fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = a + b;
        self.memory.insert("last_result".to_string(), result);
        result
    }
}

fn main() {
    let mut calc = Calculator::new();
    
    for i in 0..5 {
        let fib = fibonacci(i);
        let sum = calc.add(fib as f64, i as f64);
        println!("fib({}) + {} = {}", i, i, sum);
    }
    
    // Test various syntax constructs
    let vec: Vec<i32> = (0..10).collect();
    let filtered: Vec<_> = vec.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    
    println!("Filtered: {:?}", filtered);
}
EOF

echo "🔍 Testing parser interception..."
echo "Running: LD_PRELOAD=$INTERCEPTOR_LIB rustc test_parsing.rs"

# Run rustc with our interceptor
LD_PRELOAD="$INTERCEPTOR_LIB" rustc test_parsing.rs --emit=llvm-ir,obj -O

echo
echo "📊 Parser interception results:"

if [ -f "live_parser_capture.jsonl" ]; then
    echo "✅ Capture file created: live_parser_capture.jsonl"
    echo "📈 Lines captured: $(wc -l < live_parser_capture.jsonl)"
    
    echo
    echo "🔍 Sample captured events:"
    head -5 live_parser_capture.jsonl | jq -r '.event + " | " + .function + " | " + (.data | tostring)'
    
    echo
    echo "📋 Event summary:"
    jq -r '.event' live_parser_capture.jsonl | sort | uniq -c | sort -nr
    
    echo
    echo "🎯 Functions intercepted:"
    jq -r '.function' live_parser_capture.jsonl | sort | uniq -c | sort -nr
    
    # Create analysis report
    cat > parser_capture_analysis.md << 'EOF'
# Parser Interception Analysis

## Overview
This report analyzes the live parser data captured during rustc compilation.

## Captured Events
EOF
    
    echo "### Event Types" >> parser_capture_analysis.md
    echo '```' >> parser_capture_analysis.md
    jq -r '.event' live_parser_capture.jsonl | sort | uniq -c | sort -nr >> parser_capture_analysis.md
    echo '```' >> parser_capture_analysis.md
    
    echo "" >> parser_capture_analysis.md
    echo "### Functions Intercepted" >> parser_capture_analysis.md
    echo '```' >> parser_capture_analysis.md
    jq -r '.function' live_parser_capture.jsonl | sort | uniq -c | sort -nr >> parser_capture_analysis.md
    echo '```' >> parser_capture_analysis.md
    
    echo "" >> parser_capture_analysis.md
    echo "### Sample AST Nodes" >> parser_capture_analysis.md
    echo '```json' >> parser_capture_analysis.md
    jq 'select(.event == "ast_node") | .data' live_parser_capture.jsonl | head -10 >> parser_capture_analysis.md
    echo '```' >> parser_capture_analysis.md
    
    echo "📄 Analysis saved to: parser_capture_analysis.md"
    
else
    echo "❌ No capture file found - interception may not have worked"
    echo "💡 This might be because the actual function names in rustc are different"
    echo "🔧 Run the perf analysis first to find the real function names"
fi

echo
echo "🎉 Parser interception test complete!"
echo
echo "Next steps:"
echo "  1. Check live_parser_capture.jsonl for captured data"
echo "  2. Run advanced_perf_recorder to find actual function names"
echo "  3. Update interceptor with real rustc function signatures"
echo "  4. Use captured data for next-token prediction training"
