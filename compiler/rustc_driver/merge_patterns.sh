#!/bin/bash

echo "=== Merging all usage patterns ==="

# Create merged output
echo '{"all_patterns": [' > merged_patterns.json

# Process all JSON files
first=true
for file in usage_data/*.json; do
    if [ "$first" = true ]; then
        first=false
    else
        echo "," >> merged_patterns.json
    fi
    
    # Extract usages and add metadata
    echo "{" >> merged_patterns.json
    echo "  \"file\": \"$(basename "$file")\"," >> merged_patterns.json
    cat "$file" | jq -c '.' >> merged_patterns.json
    echo "}" >> merged_patterns.json
done

echo ']}' >> merged_patterns.json

echo "=== Creating frequency analysis ==="
# Extract all usage patterns and count frequencies
cat usage_data/*.json | jq -r '.usages[]?' | sort | uniq -c | sort -rn > pattern_frequencies.txt

echo "=== Top 50 most common patterns ==="
head -50 pattern_frequencies.txt

echo "=== Pattern analysis complete ==="
echo "Total unique patterns: $(wc -l < pattern_frequencies.txt)"
echo "Files processed: $(ls usage_data/*.json | wc -l)"
