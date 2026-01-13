#!/bin/bash

# RUSTC PARSER FUNCTION TRACING TOOLKIT
# =====================================
# 
# This script traces rustc parser function calls using multiple methods:
# 1. rustc self-profile (-Z self-profile) - Shows internal compiler phases
# 2. perf record - System-level function call tracing  
# 3. strace - System call tracing
# 4. zombie rustc - Our custom instrumented compiler
#
# The goal is to identify the call path from main() to parser functions
# so we can insert trampolines/wrappers to capture parsing data.
#
# Key parser functions we're looking for:
# - rustc_driver_impl::parse_crate_attrs
# - rustc_parse::parser::Parser methods
# - syn parsing functions
#
# Usage: ./trace_rustc_parser_calls.sh [rust_file]
# Output: Multiple trace files for analysis

set -e

RUST_FILE="${1:-test_parse.rs}"
OUTPUT_DIR="parser_traces_$(date +%Y%m%d_%H%M%S)"

echo "🔬 RUSTC PARSER CALL TRACING TOOLKIT"
echo "===================================="
echo "Target file: $RUST_FILE"
echo "Output directory: $OUTPUT_DIR"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Create test Rust file if not provided
if [ ! -f "$RUST_FILE" ]; then
    echo "📝 Creating test Rust file: $RUST_FILE"
    cat > "$RUST_FILE" << 'EOF'
// Test file for parser tracing
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("Hello parser trace! Map: {:?}", map);
}

#[derive(Debug)]
struct TestStruct {
    field1: String,
    field2: i32,
}

impl TestStruct {
    fn new(s: String, i: i32) -> Self {
        Self { field1: s, field2: i }
    }
}
EOF
fi

echo "📊 Starting multi-method tracing..."

# Method 1: rustc self-profile (shows compiler internal phases)
echo "🎯 Method 1: rustc self-profile tracing..."
echo "   Captures: Compiler phases, timing, function calls"
if rustc -Z self-profile="$OUTPUT_DIR/rustc_self_profile" "$RUST_FILE" -o "$OUTPUT_DIR/test_binary" 2>&1 | tee "$OUTPUT_DIR/self_profile.log"; then
    echo "   ✅ Self-profile completed"
    # Process self-profile data if available
    if command -v summarize &> /dev/null && [ -f "$OUTPUT_DIR/rustc_self_profile.mm_profdata" ]; then
        summarize summarize "$OUTPUT_DIR/rustc_self_profile.mm_profdata" > "$OUTPUT_DIR/self_profile_summary.txt"
        echo "   📋 Self-profile summary: $OUTPUT_DIR/self_profile_summary.txt"
    fi
else
    echo "   ❌ Self-profile failed (may need nightly rustc)"
fi

# Method 2: perf record (system-level function tracing)
echo "🎯 Method 2: perf function call tracing..."
echo "   Captures: System-level function calls, call graphs"
if command -v perf &> /dev/null; then
    if perf record -g --call-graph dwarf -o "$OUTPUT_DIR/perf.data" rustc "$RUST_FILE" -o "$OUTPUT_DIR/test_binary_perf" 2>&1 | tee "$OUTPUT_DIR/perf_record.log"; then
        echo "   ✅ Perf recording completed"
        # Generate perf script output
        perf script -i "$OUTPUT_DIR/perf.data" > "$OUTPUT_DIR/perf_script.txt" 2>/dev/null || echo "   ⚠️  Perf script generation failed"
        # Extract parser-related functions
        grep -E "(parse|Parse)" "$OUTPUT_DIR/perf_script.txt" > "$OUTPUT_DIR/parser_functions.txt" 2>/dev/null || echo "   ℹ️  No parser functions found in perf trace"
    else
        echo "   ❌ Perf recording failed (may need root or kernel.perf_event_paranoid=1)"
    fi
else
    echo "   ❌ perf not available"
fi

# Method 3: strace (system call tracing)
echo "🎯 Method 3: strace system call tracing..."
echo "   Captures: File operations, library loading"
if strace -f -e trace=openat,write -o "$OUTPUT_DIR/strace.log" rustc "$RUST_FILE" -o "$OUTPUT_DIR/test_binary_strace" 2>&1 | tee "$OUTPUT_DIR/strace_summary.log"; then
    echo "   ✅ strace completed"
    # Extract interesting file operations
    grep -E "(\.so|parse|rustc)" "$OUTPUT_DIR/strace.log" > "$OUTPUT_DIR/strace_libraries.txt" 2>/dev/null || echo "   ℹ️  No library loads found"
else
    echo "   ❌ strace failed"
fi

# Method 4: zombie rustc (our instrumented compiler)
echo "🎯 Method 4: zombie rustc tracing..."
echo "   Captures: Custom instrumentation data"
if [ -f "./zombie.sh" ]; then
    if strace -f -e trace=write,openat -o "$OUTPUT_DIR/zombie_strace.log" ./zombie.sh "$RUST_FILE" -o "$OUTPUT_DIR/test_binary_zombie" 2>&1 | tee "$OUTPUT_DIR/zombie_trace.log"; then
        echo "   ✅ Zombie rustc completed"
    else
        echo "   ❌ Zombie rustc failed"
    fi
else
    echo "   ❌ zombie.sh not found"
fi

# Method 5: Use our existing analysis tools
echo "🎯 Method 5: Existing zombie_driver2 analysis..."
echo "   Captures: Function addresses, call graphs from 2.8GB rustc_driver.so"

# Run call graph exporter if available
if [ -f "call_graph_exporter.rs" ]; then
    echo "   Running call_graph_exporter..."
    if cargo run --bin call_graph_exporter 2>&1 | tee "$OUTPUT_DIR/call_graph_export.log"; then
        echo "   ✅ Call graph export completed"
    else
        echo "   ❌ Call graph export failed"
    fi
fi

# Run live rustc caller if available  
if [ -f "live_rustc_caller.rs" ]; then
    echo "   Running live_rustc_caller..."
    if cargo run --bin live_rustc_caller 2>&1 | tee "$OUTPUT_DIR/live_rustc_call.log"; then
        echo "   ✅ Live rustc caller completed"
    else
        echo "   ❌ Live rustc caller failed"
    fi
fi

echo ""
echo "📊 TRACING COMPLETE - ANALYSIS RESULTS"
echo "======================================"
echo "Output directory: $OUTPUT_DIR"
echo ""
echo "📁 Generated files:"
ls -la "$OUTPUT_DIR/" | grep -v "^total" | while read -r line; do
    echo "   $line"
done

echo ""
echo "🔍 PARSER FUNCTION ANALYSIS"
echo "=========================="

# Analyze all logs for parser-related functions
echo "Parser functions found across all traces:"
find "$OUTPUT_DIR" -name "*.log" -o -name "*.txt" | xargs grep -h -E "(parse|Parse)" 2>/dev/null | sort | uniq | head -20 || echo "No parser functions found"

echo ""
echo "📋 NEXT STEPS FOR FUNCTION WRAPPING"
echo "=================================="
echo "1. Review self_profile.log for compiler phases"
echo "2. Check perf_script.txt for function call chains"  
echo "3. Analyze call_graph_export.log for rustc_driver.so functions"
echo "4. Look for 'rustc_driver_impl::parse_crate_attrs' in traces"
echo "5. Identify call path from main() to parser functions"
echo "6. Create trampolines/wrappers at identified call sites"

echo ""
echo "🛠️  TRAMPOLINE INSERTION STRATEGY"
echo "================================"
echo "Target functions for wrapping:"
echo "- rustc_driver_impl::parse_crate_attrs (address in rustc_addresses.rs)"
echo "- rustc_parse::parser::Parser::parse_* methods"
echo "- syn::parse_file and syn::parse_* functions"
echo ""
echo "Wrapper approach:"
echo "1. Use LD_PRELOAD to intercept function calls"
echo "2. Insert trampolines using binary patching"
echo "3. Hook dlsym calls to redirect parser functions"
echo "4. Use our existing live_rustc_caller.rs as base"

echo ""
echo "✅ Tracing toolkit complete. Use this data to identify parser call paths."
