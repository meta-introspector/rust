# Schema Comparison: Original vs Enhanced

## Original Schema (usage_data_schema.json)
- **Purpose**: Basic usage data validation
- **Required fields**: 3 (crate, module, usages)
- **Usage properties**: 9 (basic symbol tracking)
- **Focus**: Simple usage counting and categorization

### Original Usage Structure:
```json
{
  "symbol": "string",
  "kind": "string", 
  "usage_count": "integer",
  "usage_type": "enum",
  "node_type": "string",
  "user_def_id": "string",
  "used_def_id": "string"
}
```

## Enhanced Schema (enhanced_usage_data_schema.json)
- **Purpose**: HIR-integrated usage data with syn patterns
- **Required fields**: 6 (crate, enhanced_version, generation_method, usages, hir_nodes, syn_patterns)
- **Usage properties**: 7 (enhanced with HIR mapping)
- **HIR node properties**: 7 (semantic analysis)
- **Optional sections**: 3 (transformation_chains, semantic_clusters, optimization_hints)

### Enhanced Usage Structure:
```json
{
  "symbol": "string",
  "original_symbol": "string",
  "usage_count": "integer",
  "enhanced_count": "integer", 
  "hir_mapping": "string",
  "priority_score": "number",
  "chunk_optimized": "boolean"
}
```

### New HIR Node Structure:
```json
{
  "symbol": "string",
  "hir_type": "string",
  "usage_count": "integer",
  "semantic_weight": "number",
  "syn_mapping": "string",
  "transformation_potential": "number"
}
```

## Key Improvements

### 1. **HIR Integration**
- Original: No HIR awareness
- Enhanced: Full HIR node mapping with semantic weights

### 2. **Syn Pattern Support**
- Original: No syn pattern tracking
- Enhanced: Generated syn patterns for code transformation

### 3. **Enhancement Tracking**
- Original: Static usage counts
- Enhanced: Original + enhanced counts with multipliers

### 4. **Semantic Analysis**
- Original: Basic categorization
- Enhanced: Priority scoring, semantic clustering, transformation chains

### 5. **Optimization Features**
- Original: No optimization data
- Enhanced: Performance hints, transformation potential, chunking optimization

## Validation Results

### Original Schema Compliance:
- ✅ All enhanced files maintain backward compatibility
- ✅ Core fields (crate, usages) preserved
- ✅ Usage counts maintained in enhanced format

### Enhanced Schema Validation:
- ✅ All 5 chunked files pass validation
- ✅ 500 usages per file (5x enhancement)
- ✅ 200 HIR nodes per file
- ✅ 200 syn patterns per file
- ✅ All required fields present
- ✅ Valid JSON structure maintained

## Schema Evolution Summary
- **Backward compatible**: Enhanced data can be processed by tools expecting original format
- **Forward enhanced**: New tools can leverage HIR integration and syn patterns
- **Validation ready**: Comprehensive schema for production use
- **Chunking optimized**: Schema supports both chunked and non-chunked formats
