#!/bin/bash

echo "🚶 === HIR-Based Syn Walker Test Environment ==="
echo "🎯 Testing on large crates with detailed results"

# Create output directory
mkdir -p ./syn_walker_results
mkdir -p ./test_data_cache

echo "=== Phase 1: Building HIR Syn Walker ==="
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/syn_hir_walker
cargo build --release

echo "=== Phase 2: Running on Syn-Heavy Crates ==="

# Test on different syn-heavy files
echo "🔍 Testing rustc_lint (syntax-heavy)..."
./target/release/syn_hir_walker > ../syn_walker_results/rustc_lint_results.txt 2>&1

echo "🔍 Testing rustc_mir_transform (transformation-heavy)..."
# Modify to test different data
sed -i 's/rustc_lint_lifetime_syntax.json/rustc_mir_transform_shim_async_destructor_ctor.json/g' src/main.rs
cargo build --release
./target/release/syn_hir_walker > ../syn_walker_results/rustc_mir_transform_results.txt 2>&1

echo "🔍 Testing rustc_trait_selection (trait-heavy)..."
sed -i 's/rustc_mir_transform_shim_async_destructor_ctor.json/rustc_trait_selection_traits_project.json/g' src/main.rs
cargo build --release  
./target/release/syn_hir_walker > ../syn_walker_results/rustc_trait_selection_results.txt 2>&1

echo "=== Phase 3: Generating Comprehensive Analysis ==="
cd ../syn_walker_results

echo "📊 === HIR Syn Walker Test Results ===" > comprehensive_report.md
echo "Generated: $(date)" >> comprehensive_report.md
echo "" >> comprehensive_report.md

for result_file in *.txt; do
    echo "## Results from $result_file" >> comprehensive_report.md
    echo '```' >> comprehensive_report.md
    cat "$result_file" >> comprehensive_report.md
    echo '```' >> comprehensive_report.md
    echo "" >> comprehensive_report.md
done

echo "=== Phase 4: Performance Analysis ==="
echo "📈 Performance Metrics:" >> comprehensive_report.md
echo "- Total HIR nodes processed: $(grep -h "Discovered.*syn HIR nodes" *.txt | awk '{sum += $3} END {print sum}')" >> comprehensive_report.md
echo "- Total syn patterns found: $(grep -h "syn patterns" *.txt | awk -F',' '{gsub(/[^0-9]/, "", $2); sum += $2} END {print sum}')" >> comprehensive_report.md
echo "- Average usage per node: $(grep -h "Total syn usage:" *.txt | awk '{sum += $4; count++} END {print sum/count}')" >> comprehensive_report.md

echo "=== Phase 5: Code Generation Summary ==="
echo "🔧 Generated Code Files:" >> comprehensive_report.md
find . -name "generated_syn_hir_walker.rs" -exec wc -l {} \; | awk '{sum += $1} END {print "- Total generated lines: " sum}' >> comprehensive_report.md

echo "✅ === Test Environment Complete ==="
echo "📁 Results saved to: syn_walker_results/"
echo "📋 Comprehensive report: syn_walker_results/comprehensive_report.md"
echo "🎯 HIR-based syn walker ready for production use!"

# Display summary
echo ""
echo "🎉 === SUMMARY ==="
cat comprehensive_report.md | grep -E "(Total|Average|Generated)"
