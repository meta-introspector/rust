#!/bin/bash

echo "🔍 === ENHANCED DATA SCHEMA VALIDATION ==="

# Check if ajv-cli is available, if not use basic jq validation
if command -v ajv &> /dev/null; then
    echo "📋 Using ajv-cli for JSON Schema validation"
    VALIDATOR="ajv"
else
    echo "📋 Using jq for basic JSON validation (ajv-cli not available)"
    VALIDATOR="jq"
fi

SCHEMA_FILE="enhanced_usage_data_schema.json"
DATA_DIR="chunked_enhanced_data"

echo ""
echo "🔍 Validating schema file..."
if jq . "$SCHEMA_FILE" >/dev/null 2>&1; then
    echo "✅ Schema file is valid JSON"
else
    echo "❌ Schema file has JSON errors"
    exit 1
fi

echo ""
echo "📊 Validating enhanced data files..."

for data_file in "$DATA_DIR"/enhanced_*.json; do
    if [ -f "$data_file" ]; then
        filename=$(basename "$data_file")
        echo -n "  📁 $filename: "
        
        # Basic JSON validation
        if jq . "$data_file" >/dev/null 2>&1; then
            echo -n "✅ Valid JSON"
            
            # Check required fields
            if jq -e '.crate and .enhanced_version and .generation_method and .usages and .hir_nodes and .syn_patterns' "$data_file" >/dev/null 2>&1; then
                echo " ✅ Required fields present"
                
                # Additional structure checks
                usage_count=$(jq '.usages | length' "$data_file")
                hir_count=$(jq '.hir_nodes | length' "$data_file")
                syn_count=$(jq '.syn_patterns | length' "$data_file")
                
                echo "    📊 Usages: $usage_count, HIR nodes: $hir_count, Syn patterns: $syn_count"
                
                # Validate usage structure
                if jq -e '.usages[0] | has("symbol") and has("original_symbol") and has("usage_count") and has("hir_mapping")' "$data_file" >/dev/null 2>&1; then
                    echo "    ✅ Usage structure valid"
                else
                    echo "    ⚠️  Usage structure incomplete"
                fi
                
                # Validate HIR node structure
                if jq -e '.hir_nodes[0] | has("symbol") and has("hir_type") and has("usage_count")' "$data_file" >/dev/null 2>&1; then
                    echo "    ✅ HIR node structure valid"
                else
                    echo "    ⚠️  HIR node structure incomplete"
                fi
                
            else
                echo " ❌ Missing required fields"
            fi
        else
            echo "❌ Invalid JSON"
        fi
        echo ""
    fi
done

echo "🎯 === SCHEMA VALIDATION SUMMARY ==="
echo "📋 Schema: $SCHEMA_FILE"
echo "📁 Data directory: $DATA_DIR"
echo "🔍 Validation method: $VALIDATOR"

# Count files
total_files=$(ls "$DATA_DIR"/enhanced_*.json 2>/dev/null | wc -l)
echo "📊 Total files validated: $total_files"

# Show schema statistics
echo ""
echo "📈 Schema Statistics:"
echo "  • Required fields: $(jq '.required | length' "$SCHEMA_FILE")"
echo "  • Usage properties: $(jq '.properties.usages.items.properties | keys | length' "$SCHEMA_FILE")"
echo "  • HIR node properties: $(jq '.properties.hir_nodes.items.properties | keys | length' "$SCHEMA_FILE")"
echo "  • Optional sections: $(jq '.properties | keys | map(select(. | test("transformation_chains|semantic_clusters|optimization_hints"))) | length' "$SCHEMA_FILE")"

echo ""
echo "✅ Schema validation complete!"
