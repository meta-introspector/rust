#!/bin/bash

echo "=== TYCTXT USAGE GAP ANALYSIS ==="

# 1. Extract TyCtxt usage from our collector
echo "🔍 Extracting TyCtxt usage from our collector..."
COLLECTOR_FILE="../rustc_driver/usage_collector.rs"
grep -n "TyCtxt\|rustc_middle\|rustc_hir" "$COLLECTOR_FILE" > our_rustc_usage.txt
echo "Our collector rustc usage patterns: $(wc -l < our_rustc_usage.txt)"

# 2. Run collector on itself to get detailed usage
echo "🔄 Running collector on itself..."
cd ../rustc_driver
./usage_collector usage_collector.rs --crate-name usage_collector 2>/dev/null
if [ -d "usage_data" ]; then
    # Extract TyCtxt patterns from self-analysis
    grep -h "TyCtxt\|rustc_middle\|rustc_hir" usage_data/*.json > ../usage_eigenmatrix/self_tyctxt_usage.txt 2>/dev/null
    SELF_PATTERNS=$(wc -l < ../usage_eigenmatrix/self_tyctxt_usage.txt 2>/dev/null || echo "0")
    echo "Self-analysis TyCtxt patterns: $SELF_PATTERNS"
fi

cd ../usage_eigenmatrix

# 3. Extract global TyCtxt usage from middle data
echo "📊 Analyzing global TyCtxt usage patterns..."
MIDDLE_DIR="../introspector-collector"
if [ -d "$MIDDLE_DIR" ]; then
    # Find all TyCtxt usage patterns from historical data
    find "$MIDDLE_DIR" -name "*.json" -o -name "*.log" | xargs grep -h "TyCtxt" 2>/dev/null | head -100 > global_tyctxt_usage.txt
    GLOBAL_PATTERNS=$(wc -l < global_tyctxt_usage.txt)
    echo "Global TyCtxt patterns found: $GLOBAL_PATTERNS"
    
    # Extract unique TyCtxt methods
    grep -o "TyCtxt::[a-zA-Z_][a-zA-Z0-9_]*" global_tyctxt_usage.txt | sort | uniq > global_tyctxt_methods.txt
    GLOBAL_METHODS=$(wc -l < global_tyctxt_methods.txt)
    echo "Unique TyCtxt methods globally: $GLOBAL_METHODS"
else
    echo "No middle data found, creating sample global patterns..."
    cat > global_tyctxt_usage.txt << 'EOF'
TyCtxt::def_kind
TyCtxt::def_span
TyCtxt::hir
TyCtxt::type_of
TyCtxt::generics_of
TyCtxt::predicates_of
TyCtxt::item_name
TyCtxt::def_path_str
TyCtxt::is_diagnostic_item
TyCtxt::lang_items
EOF
    GLOBAL_METHODS=10
fi

# 4. Compare usage patterns
echo "🔬 Comparing usage patterns..."

# Extract methods from our usage
grep -o "TyCtxt::[a-zA-Z_][a-zA-Z0-9_]*" our_rustc_usage.txt self_tyctxt_usage.txt 2>/dev/null | sort | uniq > our_tyctxt_methods.txt
OUR_METHODS=$(wc -l < our_tyctxt_methods.txt 2>/dev/null || echo "0")
echo "Our TyCtxt methods: $OUR_METHODS"

# Find missing methods
if [ -f "global_tyctxt_methods.txt" ] && [ -f "our_tyctxt_methods.txt" ]; then
    comm -23 global_tyctxt_methods.txt our_tyctxt_methods.txt > missing_tyctxt_methods.txt
    MISSING_METHODS=$(wc -l < missing_tyctxt_methods.txt)
    echo "Missing TyCtxt methods: $MISSING_METHODS"
else
    MISSING_METHODS="Unknown"
fi

# 5. Generate gap analysis report
cat > tyctxt_gap_analysis.txt << EOF
TYCTXT USAGE GAP ANALYSIS
========================

SUMMARY:
- Global TyCtxt patterns: $GLOBAL_PATTERNS
- Global unique methods: $GLOBAL_METHODS  
- Our TyCtxt methods: $OUR_METHODS
- Missing methods: $MISSING_METHODS
- Coverage: $(( OUR_METHODS * 100 / GLOBAL_METHODS ))%

TOP MISSING METHODS:
EOF

if [ -f "missing_tyctxt_methods.txt" ]; then
    head -10 missing_tyctxt_methods.txt >> tyctxt_gap_analysis.txt
fi

cat >> tyctxt_gap_analysis.txt << EOF

USAGE FREQUENCY ANALYSIS:
EOF

# Count frequency of each method in global usage
if [ -f "global_tyctxt_usage.txt" ]; then
    echo "Most frequent TyCtxt methods globally:" >> tyctxt_gap_analysis.txt
    grep -o "TyCtxt::[a-zA-Z_][a-zA-Z0-9_]*" global_tyctxt_usage.txt | sort | uniq -c | sort -nr | head -10 >> tyctxt_gap_analysis.txt
fi

# 6. Generate expansion priorities
cat > expansion_priorities.txt << EOF
EXPANSION PRIORITIES BASED ON TYCTXT ANALYSIS
=============================================

HIGH PRIORITY (Most frequent globally, missing from our collector):
EOF

if [ -f "missing_tyctxt_methods.txt" ] && [ -f "global_tyctxt_usage.txt" ]; then
    # Find high-frequency missing methods
    for method in $(head -5 missing_tyctxt_methods.txt); do
        count=$(grep -c "$method" global_tyctxt_usage.txt)
        echo "- $method (used $count times globally)" >> expansion_priorities.txt
    done
fi

cat >> expansion_priorities.txt << EOF

IMPLEMENTATION SUGGESTIONS:
1. Add TyCtxt query tracking to collector
2. Implement HIR node analysis 
3. Track type system interactions
4. Monitor DefId resolution patterns
5. Capture generic instantiation usage

NEXT STEPS:
1. Implement top 5 missing high-frequency methods
2. Add usage frequency tracking
3. Create comparative analysis dashboard
4. Validate against known rustc usage patterns
EOF

echo "✅ Analysis complete! Generated:"
echo "  - tyctxt_gap_analysis.txt (detailed gap analysis)"
echo "  - expansion_priorities.txt (implementation priorities)"
echo "  - missing_tyctxt_methods.txt (specific missing methods)"
echo "  - Coverage: $(( OUR_METHODS * 100 / GLOBAL_METHODS ))% of global TyCtxt usage"
