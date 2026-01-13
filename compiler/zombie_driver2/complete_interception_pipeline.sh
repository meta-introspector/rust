#!/bin/bash

# Complete Rustc Parser Interception Pipeline
# This script orchestrates the entire process from perf recording to function interception

set -e

echo "=== Rustc Parser Interception Pipeline ==="
echo "Starting comprehensive analysis and interception setup..."

# Step 1: Record performance data
echo
echo "Step 1: Recording performance data with perf..."
if ! command -v perf &> /dev/null; then
    echo "Error: perf not found. Install with: sudo apt-get install linux-tools-generic"
    exit 1
fi

cargo run --bin advanced_perf_recorder

# Step 2: Extract call graph from perf data
echo
echo "Step 2: Extracting call graph from performance data..."
cargo run --bin step2_extract_callgraph

# Step 3: Prepare target functions for interception
echo
echo "Step 3: Preparing target functions for interception..."
cargo run --bin step3_prepare_targets

# Step 4: Set up dynamic interception
echo
echo "Step 4: Setting up dynamic function interception..."
cargo run --bin dynamic_interceptor

# Step 5: Install trampolines (if available)
echo
echo "Step 5: Installing function trampolines..."
if [ -f "step4_install_trampolines.rs" ]; then
    cargo run --bin step4_install_trampolines
else
    echo "Trampoline installer not available, skipping..."
fi

# Step 6: Run comprehensive analysis
echo
echo "Step 6: Running comprehensive parser analysis..."
cargo run --bin perf_rustc_tracer

# Step 7: Generate final report
echo
echo "Step 7: Generating comprehensive analysis report..."

cat > final_report.md << 'EOF'
# Rustc Parser Interception Analysis Report

## Overview
This report summarizes the complete analysis of rustc parser function calls,
performance characteristics, and interception capabilities.

## Files Generated
- `perf_data/` - Performance recording data
- `interception_data/` - Function interception logs and analysis
- `parser_target_functions.json` - High-priority functions for interception
- `call_graph.json` - Complete function call relationships

## Key Findings

### Performance Hotspots
The following functions were identified as performance-critical during compilation:

EOF

# Add perf analysis results to report
if [ -f "perf_data/function_analysis.json" ]; then
    echo "### Top Functions by Sample Count" >> final_report.md
    echo '```json' >> final_report.md
    head -20 perf_data/function_analysis.json >> final_report.md
    echo '```' >> final_report.md
fi

# Add interception results
if [ -f "interception_data/interception_report.md" ]; then
    echo "" >> final_report.md
    cat interception_data/interception_report.md >> final_report.md
fi

cat >> final_report.md << 'EOF'

## Next Steps

### For Parser Function Wrapping
1. Use the identified target functions in `parser_target_functions.json`
2. Implement custom wrappers using the LD_PRELOAD library
3. Capture AST data during live parsing

### For Real-time Analysis
1. Deploy the interception library with rustc
2. Monitor function call patterns during compilation
3. Extract parsing decision trees for AI training

### For Trampoline Installation
1. Use the call graph data to identify injection points
2. Install trampolines at function entry/exit points
3. Capture live parsing state for next-token prediction

## Technical Details

### Interception Methods Used
- **perf recording**: Hardware performance counter sampling
- **LD_PRELOAD**: Dynamic library interposition
- **Symbol analysis**: ELF binary parsing with goblin
- **Call graph extraction**: Function relationship mapping

### Data Formats
- JSON for structured data exchange
- Parquet for large-scale analysis (if available)
- Markdown for human-readable reports

## Integration with Meta-Introspector

This analysis provides the foundation for:
- Real-time code parsing instrumentation
- AI training data generation from live compilation
- Next-character/token prediction model training
- Compiler decision tree extraction

EOF

echo
echo "=== Pipeline Complete ==="
echo "Generated files:"
echo "  - final_report.md (comprehensive analysis)"
echo "  - perf_data/ (performance data)"
echo "  - interception_data/ (interception results)"
echo "  - parser_target_functions.json (target functions)"
echo
echo "To use the interception library:"
echo "  LD_PRELOAD=./interceptor/target/release/librustc_interceptor.so rustc your_file.rs"
echo
echo "Next steps:"
echo "  1. Review final_report.md for analysis results"
echo "  2. Use parser_target_functions.json for custom instrumentation"
echo "  3. Deploy interception library for live parsing capture"
