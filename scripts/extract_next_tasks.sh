#!/bin/bash

OUTPUT_FILE="/data/data/com.termux/files/home/storage/github/rustc/tasks/results/next_tasks_summary.md"
REGEX_PATTERN="(TODO|FIXME|HACK|XXX|@future|@next|[ ]*)"

echo "# Next Tasks Summary" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "Generated on: $(date)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "Starting extraction of next tasks. This may take a while..."

# Find files, excluding specific directories
find . -type f \
  -not -path "./target/*" \
  -not -path "./vendor/*" \
  -not -path "./logs/*" \
  -not -path "./.git/*" \
  -not -path "./build/*" \
  -not -path "./node_modules/*" \
  -print0 | while IFS= read -r -d $'\0' file; do
    echo "Searching in: $file"
    grep -E -n "$REGEX_PATTERN" "$file" >> "$OUTPUT_FILE"
done

echo "Extraction complete. Results are in: $OUTPUT_FILE"
