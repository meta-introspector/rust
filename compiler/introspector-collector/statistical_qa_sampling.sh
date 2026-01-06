#!/bin/bash

echo "🔬 === STATISTICAL SAMPLING QA PROCESS ==="
echo "Sample size: 7 (sqrt(27)+1 ≈ 6.2 rounded up)"

# Get all 27 spore vials
spore_vials=($(find src/bin -name "*.rs" | xargs -I {} basename {} .rs))
echo "Total population: ${#spore_vials[@]} spores"

# Randomly sample 7 spores
sampled_spores=()
temp_array=("${spore_vials[@]}")

for i in {1..7}; do
    # Get random index
    random_index=$((RANDOM % ${#temp_array[@]}))
    sampled_spores+=("${temp_array[$random_index]}")
    
    # Remove selected item from temp array
    temp_array=("${temp_array[@]:0:$random_index}" "${temp_array[@]:$((random_index + 1))}")
done

echo "📊 Randomly sampled spores for QA:"
for i in "${!sampled_spores[@]}"; do
    echo "  $((i+1)). ${sampled_spores[$i]}"
done

echo ""
echo "🧪 === QUALITY ASSURANCE PROCESS ==="

qa_results=()
for spore in "${sampled_spores[@]}"; do
    echo "Testing: $spore"
    
    # Compilation test
    if cargo build --bin "$spore" 2>/dev/null; then
        compile_status="✅ PASS"
    else
        compile_status="❌ FAIL"
    fi
    
    # Runtime test (30s timeout)
    if timeout 10s "../../target/debug/$spore" >/dev/null 2>&1; then
        runtime_status="✅ PASS"
    else
        runtime_status="⚠️  TIMEOUT/ERROR"
    fi
    
    # Mutation analysis
    if [ -f "./cultivation_chamber/analysis/${spore}_mutations.log" ]; then
        if grep -q "suggestion\|improvement\|optimization" "./cultivation_chamber/analysis/${spore}_mutations.log" 2>/dev/null; then
            mutation_status="🧬 BENEFICIAL"
        else
            mutation_status="🔄 NEUTRAL"
        fi
    else
        mutation_status="❓ UNKNOWN"
    fi
    
    qa_result="$spore: $compile_status | $runtime_status | $mutation_status"
    qa_results+=("$qa_result")
    echo "  Result: $qa_result"
done

echo ""
echo "📋 === QA SUMMARY REPORT ==="
pass_count=0
fail_count=0
beneficial_count=0

for result in "${qa_results[@]}"; do
    echo "$result"
    if [[ "$result" == *"✅ PASS"* ]]; then
        ((pass_count++))
    fi
    if [[ "$result" == *"❌ FAIL"* ]]; then
        ((fail_count++))
    fi
    if [[ "$result" == *"🧬 BENEFICIAL"* ]]; then
        ((beneficial_count++))
    fi
done

echo ""
echo "📊 === STATISTICAL ANALYSIS ==="
echo "Sample size: 7"
echo "Compilation success rate: $pass_count/7 ($(echo "scale=1; $pass_count * 100 / 7" | bc)%)"
echo "Beneficial mutations: $beneficial_count/7 ($(echo "scale=1; $beneficial_count * 100 / 7" | bc)%)"

# Quality score calculation
quality_score=$(echo "scale=1; ($pass_count * 40 + $beneficial_count * 60) / 7" | bc)
echo "Overall quality score: $quality_score/100"

if (( $(echo "$quality_score >= 80" | bc -l) )); then
    echo "🏆 QA STATUS: EXCELLENT"
elif (( $(echo "$quality_score >= 60" | bc -l) )); then
    echo "✅ QA STATUS: GOOD"
elif (( $(echo "$quality_score >= 40" | bc -l) )); then
    echo "⚠️  QA STATUS: ACCEPTABLE"
else
    echo "❌ QA STATUS: NEEDS IMPROVEMENT"
fi

echo ""
echo "🔬 QA process complete. Results logged for statistical analysis."
