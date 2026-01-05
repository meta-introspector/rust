#!/bin/bash

echo "=== COLLECTOR SELF-ANALYSIS & COVERAGE REPORT ==="

# 1. Run our prime-complexity analyzer on the usage collector itself
echo "🔬 Analyzing usage_collector.rs with prime-complexity analyzer..."
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_eigenmatrix
cargo run --bin syn_prime_analyzer -- ../rustc_driver/usage_collector.rs > collector_analysis.txt 2>&1

# 2. Run the collector on itself (meta-collection)
echo "🔄 Running collector on itself (meta-analysis)..."
cd ../rustc_driver
./usage_collector usage_collector.rs --crate-name usage_collector > self_collection.log 2>&1

# 3. Compare with existing middle data
echo "📊 Comparing with existing rustc_middle data..."
MIDDLE_DATA_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/introspector-collector/ty_changes"
if [ -d "$MIDDLE_DATA_DIR" ]; then
    echo "Found middle data versions:"
    ls "$MIDDLE_DATA_DIR" | head -5
    
    # Count coverage
    COLLECTOR_PATTERNS=$(find usage_data/ -name "*.json" -exec grep -h "USES" {} \; 2>/dev/null | wc -l)
    echo "Collector captured: $COLLECTOR_PATTERNS usage patterns"
    
    # Find gaps
    echo "🔍 Identifying coverage gaps..."
    echo "Missing patterns analysis:" > coverage_gaps.txt
    echo "- TyCtxt usage patterns" >> coverage_gaps.txt
    echo "- HIR node traversal patterns" >> coverage_gaps.txt
    echo "- DefId resolution patterns" >> coverage_gaps.txt
else
    echo "No middle data found for comparison"
fi

# 4. Generate expansion suggestions
echo "💡 Generating expansion suggestions..."
cat > expansion_suggestions.txt << 'EOF'
COLLECTOR EXPANSION OPPORTUNITIES
=================================

Based on analysis, suggest adding:

1. HIR Visitor Patterns
   - Track hir::visit::Visitor implementations
   - Capture node type transitions
   - Monitor traversal depth patterns

2. Type System Integration
   - TyCtxt method usage tracking
   - TypeckResults access patterns
   - DefId resolution chains

3. Macro Expansion Tracking
   - Macro invocation patterns
   - Expansion context tracking
   - Hygiene scope analysis

4. Error Handling Patterns
   - Result/Option usage chains
   - Error propagation patterns
   - Diagnostic emission tracking

5. Memory Usage Patterns
   - Arena allocation tracking
   - Interning usage patterns
   - Reference counting analysis

IMPLEMENTATION PRIORITY:
- High: HIR visitor patterns (most common)
- Medium: Type system integration
- Low: Memory usage patterns
EOF

echo "✅ Analysis complete! Check:"
echo "  - collector_analysis.txt (prime-complexity analysis)"
echo "  - self_collection.log (meta-collection results)"
echo "  - coverage_gaps.txt (identified gaps)"
echo "  - expansion_suggestions.txt (improvement opportunities)"
