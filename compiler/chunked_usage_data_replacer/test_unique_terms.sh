#!/bin/bash

echo "🔍 === UNIQUE TERMS PRESERVATION TEST ==="

OLD_DATA_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data"
NEW_DATA_DIR="chunked_enhanced_data"

echo "📊 Testing unique term preservation between original and enhanced data..."
echo ""

# Test files that we enhanced
test_files=(
    "rustc_query_impl_literals.json"
    "rustc_metadata_literals.json" 
    "rustc_target_constants.json"
    "rustc_lint_literals.json"
    "rustc_mir_transform_shim_async_destructor_ctor.json"
)

total_old_unique=0
total_new_unique=0
total_preserved=0

for file in "${test_files[@]}"; do
    old_file="$OLD_DATA_DIR/$file"
    new_file="$NEW_DATA_DIR/enhanced_$file"
    
    # Check if we have chunked files instead
    if [ ! -f "$new_file" ]; then
        # Look for chunked files
        chunk_pattern="$NEW_DATA_DIR/enhanced_${file%.json}_chunk_*.json"
        if ls $chunk_pattern 1> /dev/null 2>&1; then
            echo "🔍 Testing: $file (chunked)"
            
            # Extract unique symbols from old data
            old_unique=$(jq -r '.usages[].symbol' "$old_file" 2>/dev/null | sort -u | wc -l)
            
            # Extract unique original_symbols from all chunks
            if [ -f /tmp/all_new_symbols.txt ]; then rm /tmp/all_new_symbols.txt; fi
            for chunk_file in $chunk_pattern; do
                jq -r '.usages[].original_symbol' "$chunk_file" 2>/dev/null >> /tmp/all_new_symbols.txt
            done
            new_unique=$(sort -u /tmp/all_new_symbols.txt | wc -l)
            
            # Check preservation
            if [ -f /tmp/old_symbols.txt ]; then rm /tmp/old_symbols.txt; fi
            jq -r '.usages[].symbol' "$old_file" 2>/dev/null | sort -u > /tmp/old_symbols.txt
            sort -u /tmp/all_new_symbols.txt > /tmp/new_symbols.txt
            
            preserved=$(comm -12 /tmp/old_symbols.txt /tmp/new_symbols.txt | wc -l)
            preservation_rate=$(echo "scale=1; $preserved * 100 / $old_unique" | bc -l 2>/dev/null || echo "0")
            
            # Count enhanced symbols from all chunks
            if [ -f /tmp/all_enhanced_symbols.txt ]; then rm /tmp/all_enhanced_symbols.txt; fi
            for chunk_file in $chunk_pattern; do
                jq -r '.usages[].symbol' "$chunk_file" 2>/dev/null >> /tmp/all_enhanced_symbols.txt
            done
            enhanced_symbols=$(sort -u /tmp/all_enhanced_symbols.txt | wc -l)
            enhancement_ratio=$(echo "scale=1; $enhanced_symbols / $old_unique" | bc -l 2>/dev/null || echo "0")
            
            chunk_count=$(ls $chunk_pattern | wc -l)
            echo "  📦 Chunks: $chunk_count"
            echo "  📈 Old unique symbols: $old_unique"
            echo "  📈 New unique symbols: $new_unique" 
            echo "  ✅ Preserved symbols: $preserved ($preservation_rate%)"
            echo "  🚀 Enhanced symbols: $enhanced_symbols (${enhancement_ratio}x)"
            
            # Accumulate totals
            total_old_unique=$((total_old_unique + old_unique))
            total_new_unique=$((total_new_unique + new_unique))
            total_preserved=$((total_preserved + preserved))
            
            echo ""
            continue
        fi
    fi
    
    if [ -f "$old_file" ] && [ -f "$new_file" ]; then
        echo "🔍 Testing: $file"
        
        # Extract unique symbols from old data
        old_unique=$(jq -r '.usages[].symbol' "$old_file" 2>/dev/null | sort -u | wc -l)
        
        # Extract unique original_symbols from new data (these should match old symbols)
        new_unique=$(jq -r '.usages[].original_symbol' "$new_file" 2>/dev/null | sort -u | wc -l)
        
        # Check how many old symbols are preserved in new data
        if [ -f /tmp/old_symbols.txt ]; then rm /tmp/old_symbols.txt; fi
        if [ -f /tmp/new_symbols.txt ]; then rm /tmp/new_symbols.txt; fi
        
        jq -r '.usages[].symbol' "$old_file" 2>/dev/null | sort -u > /tmp/old_symbols.txt
        jq -r '.usages[].original_symbol' "$new_file" 2>/dev/null | sort -u > /tmp/new_symbols.txt
        
        preserved=$(comm -12 /tmp/old_symbols.txt /tmp/new_symbols.txt | wc -l)
        preservation_rate=$(echo "scale=1; $preserved * 100 / $old_unique" | bc -l 2>/dev/null || echo "0")
        
        echo "  📈 Old unique symbols: $old_unique"
        echo "  📈 New unique symbols: $new_unique" 
        echo "  ✅ Preserved symbols: $preserved ($preservation_rate%)"
        
        # Check if we have the expected enhancement
        enhanced_symbols=$(jq -r '.usages[].symbol' "$new_file" 2>/dev/null | sort -u | wc -l)
        enhancement_ratio=$(echo "scale=1; $enhanced_symbols / $old_unique" | bc -l 2>/dev/null || echo "0")
        echo "  🚀 Enhanced symbols: $enhanced_symbols (${enhancement_ratio}x)"
        
        # Accumulate totals
        total_old_unique=$((total_old_unique + old_unique))
        total_new_unique=$((total_new_unique + new_unique))
        total_preserved=$((total_preserved + preserved))
        
        echo ""
    else
        echo "⚠️  Missing files for $file"
        echo ""
    fi
done

# Calculate overall preservation rate
if [ $total_old_unique -gt 0 ]; then
    overall_preservation=$(echo "scale=1; $total_preserved * 100 / $total_old_unique" | bc -l)
    echo "🎯 === OVERALL PRESERVATION RESULTS ==="
    echo "📊 Total old unique symbols: $total_old_unique"
    echo "📊 Total new unique symbols: $total_new_unique"
    echo "✅ Total preserved symbols: $total_preserved"
    echo "📈 Overall preservation rate: $overall_preservation%"
    
    if (( $(echo "$overall_preservation >= 95.0" | bc -l) )); then
        echo "✅ PASS: Excellent preservation (≥95%)"
    elif (( $(echo "$overall_preservation >= 90.0" | bc -l) )); then
        echo "⚠️  ACCEPTABLE: Good preservation (≥90%)"
    else
        echo "❌ FAIL: Poor preservation (<90%)"
    fi
else
    echo "❌ No data to analyze"
fi

echo ""
echo "🔍 === SAMPLE SYMBOL COMPARISON ==="
if [ -f /tmp/old_symbols.txt ] && [ -f /tmp/new_symbols.txt ]; then
    echo "📋 First 5 original symbols:"
    head -5 /tmp/old_symbols.txt | sed 's/^/  • /'
    echo ""
    echo "📋 First 5 preserved symbols:"
    head -5 /tmp/new_symbols.txt | sed 's/^/  • /'
    
    # Check for any lost symbols
    lost_symbols=$(comm -23 /tmp/old_symbols.txt /tmp/new_symbols.txt | wc -l)
    if [ $lost_symbols -gt 0 ]; then
        echo ""
        echo "⚠️  Lost symbols: $lost_symbols"
        echo "📋 Sample lost symbols:"
        comm -23 /tmp/old_symbols.txt /tmp/new_symbols.txt | head -3 | sed 's/^/  • /'
    else
        echo ""
        echo "✅ No symbols lost!"
    fi
fi

# Cleanup
rm -f /tmp/old_symbols.txt /tmp/new_symbols.txt

echo ""
echo "✅ Unique terms preservation test complete!"
