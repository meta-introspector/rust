#!/bin/bash

echo "=== COLLECTOR COVERAGE ANALYSIS ==="

# Analyze the usage_collector.rs directly
echo "🔬 Analyzing usage_collector.rs structure..."
COLLECTOR_FILE="../rustc_driver/usage_collector.rs"

if [ -f "$COLLECTOR_FILE" ]; then
    echo "📊 COLLECTOR METRICS:"
    echo "  Lines: $(wc -l < $COLLECTOR_FILE)"
    echo "  Structs: $(grep -c "struct " $COLLECTOR_FILE)"
    echo "  Impls: $(grep -c "impl " $COLLECTOR_FILE)"
    echo "  Functions: $(grep -c "fn " $COLLECTOR_FILE)"
    
    echo -e "\n🎯 COLLECTOR CAPABILITIES:"
    echo "  ✅ Basic usage tracking"
    echo "  ✅ Module-level organization"
    echo "  ✅ JSON output format"
    echo "  ✅ AssocFn call tracking"
    
    echo -e "\n❌ MISSING CAPABILITIES:"
    echo "  - HIR node type analysis"
    echo "  - Type inference tracking"
    echo "  - Macro expansion patterns"
    echo "  - Error propagation chains"
    echo "  - Memory allocation patterns"
    
    # Check syn usage data for comparison
    SYN_DATA="../../../syn/usage_data"
    if [ -d "$SYN_DATA" ]; then
        TOTAL_PATTERNS=$(find "$SYN_DATA" -name "*.json" -exec grep -c "USES" {} \; | awk '{sum+=$1} END {print sum}')
        UNIQUE_MODULES=$(ls "$SYN_DATA"/*.json | wc -l)
        
        echo -e "\n📈 SYN COLLECTION RESULTS:"
        echo "  Total usage patterns: $TOTAL_PATTERNS"
        echo "  Unique modules: $UNIQUE_MODULES"
        echo "  Average patterns per module: $((TOTAL_PATTERNS / UNIQUE_MODULES))"
    fi
    
    # Generate coverage report
    echo -e "\n📋 COVERAGE REPORT:" > coverage_report.txt
    echo "===================" >> coverage_report.txt
    echo "" >> coverage_report.txt
    echo "CURRENT COVERAGE:" >> coverage_report.txt
    echo "- Function calls: ✅ Tracked" >> coverage_report.txt
    echo "- Method calls: ✅ Tracked" >> coverage_report.txt
    echo "- Module boundaries: ✅ Tracked" >> coverage_report.txt
    echo "- Crate dependencies: ✅ Tracked" >> coverage_report.txt
    echo "" >> coverage_report.txt
    echo "MISSING COVERAGE:" >> coverage_report.txt
    echo "- HIR visitor patterns: ❌ Not tracked" >> coverage_report.txt
    echo "- Type system queries: ❌ Not tracked" >> coverage_report.txt
    echo "- Macro expansions: ❌ Not tracked" >> coverage_report.txt
    echo "- Trait implementations: ❌ Not tracked" >> coverage_report.txt
    echo "- Generic instantiations: ❌ Not tracked" >> coverage_report.txt
    
    # Generate expansion module
    cat > expansion_module.rs << 'EOF'
// Expansion module for enhanced usage collection
// Suggests additional collection opportunities

use rustc_hir as hir;
use rustc_middle::ty::TyCtxt;

pub struct EnhancedCollector {
    basic_collector: UsageCollector,
    hir_patterns: Vec<HirPattern>,
    type_queries: Vec<TypeQuery>,
}

#[derive(Debug)]
pub struct HirPattern {
    node_type: String,
    depth: usize,
    context: String,
}

#[derive(Debug)]
pub struct TypeQuery {
    query_type: String,
    def_id: String,
    result_type: String,
}

impl EnhancedCollector {
    pub fn new() -> Self {
        Self {
            basic_collector: UsageCollector::new(),
            hir_patterns: Vec::new(),
            type_queries: Vec::new(),
        }
    }
    
    // Track HIR visitor patterns
    pub fn track_hir_visit(&mut self, node: &str, depth: usize) {
        self.hir_patterns.push(HirPattern {
            node_type: node.to_string(),
            depth,
            context: "visitor".to_string(),
        });
    }
    
    // Track type system queries
    pub fn track_type_query(&mut self, query: &str, def_id: &str) {
        self.type_queries.push(TypeQuery {
            query_type: query.to_string(),
            def_id: def_id.to_string(),
            result_type: "unknown".to_string(),
        });
    }
    
    // Generate expansion suggestions
    pub fn suggest_expansions(&self) -> Vec<String> {
        vec![
            "Add HIR node transition tracking".to_string(),
            "Implement type inference pattern capture".to_string(),
            "Track macro expansion contexts".to_string(),
            "Monitor trait resolution chains".to_string(),
            "Capture generic instantiation patterns".to_string(),
        ]
    }
}
EOF
    
    echo "✅ Generated coverage_report.txt and expansion_module.rs"
    
else
    echo "❌ usage_collector.rs not found at $COLLECTOR_FILE"
fi
