#!/bin/bash

echo "🎵 SPECTRAL COMPILATION PIPELINE"
echo "================================"

RUSTC_DRIVER="../rustc_driver_impl/src/lib"

echo "📊 Compilation Results:"
echo "======================"

# Test each spectral component
for filter in "struct" "fn" "impl"; do
    echo -n "$filter: "
    
    # Try to compile with minimal dependencies
    if rustc --crate-name "rustc_driver_$filter" \
             --crate-type lib \
             --edition 2021 \
             --allow warnings \
             "$RUSTC_DRIVER.$filter.rs" \
             -o "rustc_driver_$filter.rlib" 2>/dev/null; then
        echo "✅ SUCCESS ($(stat -c%s rustc_driver_$filter.rlib) bytes)"
    else
        echo "❌ FAILED (missing dependencies)"
    fi
done

echo ""
echo "📈 Spectral Compilation Analysis:"
echo "================================="

# Count successful compilations
success_count=$(ls -1 rustc_driver_*.rlib 2>/dev/null | wc -l)
total_count=3

echo "Successful spectral bands: $success_count/$total_count"
echo "Compilation success rate: $((success_count * 100 / total_count))%"

if [ $success_count -gt 0 ]; then
    echo ""
    echo "🎯 Generated Libraries:"
    ls -la rustc_driver_*.rlib 2>/dev/null || echo "None"
fi
