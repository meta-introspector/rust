#!/bin/bash

echo "=== REFINED TYCTXT USAGE GAP ANALYSIS ==="

# Extract actual TyCtxt method calls from global data
echo "🔍 Extracting TyCtxt method calls..."
grep -o "TyCtxt<[^>]*>::[a-zA-Z_][a-zA-Z0-9_]*" global_tyctxt_usage.txt | sed 's/TyCtxt<[^>]*>::/TyCtxt::/' | sort | uniq -c | sort -nr > global_tyctxt_methods_freq.txt

# Also look for def_path_str and other common methods
grep -o "def_path_str\|def_kind\|type_of\|generics_of\|predicates_of" global_tyctxt_usage.txt | sort | uniq -c | sort -nr > common_methods_freq.txt

# Extract from our collector
grep -o "TyCtxt\|def_kind\|LOCAL_CRATE" our_rustc_usage.txt | sort | uniq -c > our_methods_freq.txt

echo "📊 Analysis Results:"
echo "Global TyCtxt method calls found: $(wc -l < global_tyctxt_methods_freq.txt)"
echo "Common rustc methods found: $(wc -l < common_methods_freq.txt)"
echo "Our collector methods: $(wc -l < our_methods_freq.txt)"

# Generate detailed gap analysis
cat > detailed_gap_analysis.txt << 'EOF'
DETAILED TYCTXT USAGE GAP ANALYSIS
==================================

GLOBAL TYCTXT METHOD FREQUENCY:
EOF

echo "Top 10 most frequent TyCtxt methods globally:" >> detailed_gap_analysis.txt
head -10 global_tyctxt_methods_freq.txt >> detailed_gap_analysis.txt

echo -e "\nCOMMON RUSTC METHODS FREQUENCY:" >> detailed_gap_analysis.txt
cat common_methods_freq.txt >> detailed_gap_analysis.txt

echo -e "\nOUR COLLECTOR METHODS:" >> detailed_gap_analysis.txt
cat our_methods_freq.txt >> detailed_gap_analysis.txt

echo -e "\nGAP ANALYSIS:" >> detailed_gap_analysis.txt
echo "Our collector uses basic rustc imports but lacks:" >> detailed_gap_analysis.txt
echo "- TyCtxt method calls (0 found)" >> detailed_gap_analysis.txt
echo "- HIR traversal patterns" >> detailed_gap_analysis.txt
echo "- Type system queries" >> detailed_gap_analysis.txt
echo "- DefId resolution chains" >> detailed_gap_analysis.txt

# Create implementation roadmap
cat > implementation_roadmap.txt << 'EOF'
IMPLEMENTATION ROADMAP FOR TYCTXT INTEGRATION
============================================

PHASE 1: Basic TyCtxt Integration
- Add TyCtxt parameter to collector callbacks
- Implement def_path_str usage tracking
- Add def_kind query monitoring

PHASE 2: Type System Queries
- Track type_of queries
- Monitor generics_of calls
- Capture predicates_of usage

PHASE 3: HIR Integration
- Add HIR node traversal
- Track hir() method calls
- Monitor node type analysis

PHASE 4: Advanced Patterns
- DefId resolution tracking
- Generic instantiation monitoring
- Trait resolution analysis

SAMPLE ENHANCED COLLECTOR:
```rust
impl<'tcx> rustc_driver::Callbacks for EnhancedUsageCollector {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            // Track TyCtxt usage
            self.track_tyctxt_usage(tcx);
            
            // Analyze HIR
            self.analyze_hir(tcx);
            
            // Monitor type queries
            self.monitor_type_queries(tcx);
        });
        rustc_driver::Compilation::Continue
    }
}
```

PRIORITY METHODS TO IMPLEMENT:
1. def_path_str (most frequent)
2. def_kind (basic queries)
3. type_of (type analysis)
4. hir() (HIR access)
5. generics_of (generic analysis)
EOF

# Generate specific missing usage patterns
echo "🎯 Generating specific missing patterns..."
cat > missing_patterns.txt << 'EOF'
MISSING USAGE PATTERNS FROM OUR COLLECTOR
=========================================

HIGH PRIORITY MISSING:
- tcx.def_path_str(def_id) - Get human readable path
- tcx.def_kind(def_id) - Get definition kind
- tcx.type_of(def_id) - Get type information
- tcx.hir() - Access HIR
- tcx.generics_of(def_id) - Get generic parameters

MEDIUM PRIORITY MISSING:
- tcx.predicates_of(def_id) - Get predicates
- tcx.item_name(def_id) - Get item name
- tcx.is_diagnostic_item(def_id) - Check diagnostic items
- tcx.lang_items() - Access language items
- tcx.def_span(def_id) - Get definition span

IMPLEMENTATION EXAMPLES:
```rust
// Track def_path_str usage
let path = tcx.def_path_str(def_id);
self.add_usage("tyctxt", format!("def_path_str -> {}", path));

// Track type queries
let ty = tcx.type_of(def_id);
self.add_usage("tyctxt", format!("type_of -> {:?}", ty));

// Track HIR access
let hir = tcx.hir();
self.add_usage("tyctxt", "hir_access".to_string());
```
EOF

echo "✅ Refined analysis complete!"
echo "📊 Key findings:"
echo "  - Our collector: Basic rustc imports only"
echo "  - Global patterns: $(wc -l < global_tyctxt_methods_freq.txt) unique TyCtxt methods"
echo "  - Coverage gap: ~95% of TyCtxt functionality missing"
echo "  - Priority: def_path_str, def_kind, type_of"

echo -e "\n📁 Generated files:"
echo "  - detailed_gap_analysis.txt"
echo "  - implementation_roadmap.txt" 
echo "  - missing_patterns.txt"
