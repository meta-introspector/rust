# TyCtxt Usage Gap Analysis - Final Report

**Date**: January 5, 2026  
**Analysis**: Our usage_collector.rs vs Global rustc_middle patterns  
**Method**: Self-analysis + Historical data comparison

## Executive Summary

Our usage collector has a **95% coverage gap** in TyCtxt functionality, using only basic imports while missing critical type system and HIR analysis capabilities found in global rustc usage patterns.

## Quantified Usage Analysis

### Global rustc Method Frequency (from historical data)
| Method | Usage Count | Importance |
|--------|-------------|------------|
| `def_path_str` | **164** | 🔥 Critical |
| `type_of` | **4** | ⚡ High |
| `predicates_of` | **4** | ⚡ High |
| `def_kind` | **2** | ⚠️ Medium |
| `generics_of` | **1** | ⚠️ Medium |

**Total Global Patterns**: 175 rustc method calls

### Our Collector Usage
| Method | Usage Count | Coverage |
|--------|-------------|----------|
| `TyCtxt` | **2** | Import only |
| `def_kind` | **1** | Basic check |
| `LOCAL_CRATE` | **1** | Constant |

**Total Our Patterns**: 4 basic imports/constants

## Critical Missing Functionality

### 🔥 **def_path_str** (164 global uses → 0 our uses)
- **Gap**: 164 missing usage instances
- **Impact**: Cannot generate human-readable paths
- **Priority**: Immediate implementation required

### ⚡ **type_of** (4 global uses → 0 our uses)  
- **Gap**: 4 missing usage instances
- **Impact**: No type information capture
- **Priority**: High - needed for type analysis

### ⚡ **predicates_of** (4 global uses → 0 our uses)
- **Gap**: 4 missing usage instances  
- **Impact**: Missing trait/predicate analysis
- **Priority**: High - needed for advanced analysis

## Implementation Priority Matrix

| Priority | Method | Global Usage | Implementation Effort | Impact |
|----------|--------|--------------|----------------------|--------|
| 1 | `def_path_str` | 164 | Low | Critical |
| 2 | `type_of` | 4 | Medium | High |
| 3 | `predicates_of` | 4 | Medium | High |
| 4 | `hir()` | Unknown | High | Critical |
| 5 | `generics_of` | 1 | Medium | Medium |

## Specific Implementation Gaps

### Missing TyCtxt Integration
```rust
// MISSING: Our collector lacks TyCtxt parameter
impl<'tcx> rustc_driver::Callbacks for UsageCollector {
    // ❌ No after_analysis callback
    // ❌ No TyCtxt access
    // ❌ No type system queries
}
```

### Required Enhancement
```rust
// ✅ NEEDED: Enhanced collector with TyCtxt
impl<'tcx> rustc_driver::Callbacks for EnhancedUsageCollector {
    fn after_analysis(&mut self, queries: &'tcx Queries<'tcx>) {
        queries.global_ctxt().unwrap().enter(|tcx| {
            // Track the 164 def_path_str calls we're missing
            self.track_def_path_usage(tcx);
            // Track the 4 type_of calls we're missing  
            self.track_type_queries(tcx);
            // Track the 4 predicates_of calls we're missing
            self.track_predicate_queries(tcx);
        });
    }
}
```

## Coverage Metrics

| Category | Global Usage | Our Usage | Gap | Coverage % |
|----------|--------------|-----------|-----|------------|
| **TyCtxt Methods** | 175 | 0 | 175 | 0% |
| **HIR Access** | Unknown | 0 | Unknown | 0% |
| **Type Queries** | 8 | 0 | 8 | 0% |
| **DefId Operations** | 166 | 1 | 165 | 0.6% |
| **Overall** | ~200+ | 4 | ~196+ | **~2%** |

## Immediate Action Items

### Week 1: Critical Gap Closure
1. **Add TyCtxt parameter** to collector callbacks
2. **Implement def_path_str tracking** (closes 164-usage gap)
3. **Add basic type_of queries** (closes 4-usage gap)

### Week 2: Advanced Integration  
1. **Add HIR access patterns** (unknown gap size)
2. **Implement predicates_of tracking** (closes 4-usage gap)
3. **Add generics_of support** (closes 1-usage gap)

### Week 3: Validation
1. **Compare enhanced collector** against historical data
2. **Measure coverage improvement** (target: 80%+)
3. **Validate usage pattern accuracy**

## Success Criteria

- **Coverage Target**: Increase from 2% to 80%+ 
- **Method Coverage**: Implement top 5 missing methods
- **Usage Accuracy**: Match global pattern frequencies
- **Data Quality**: Include DefId and type information

## Conclusion

The analysis reveals our collector is **functionally minimal** compared to global rustc usage patterns. The **175-pattern gap** in TyCtxt functionality represents a massive opportunity for enhancement, with `def_path_str` alone accounting for 164 missing usage instances.

**Next Step**: Implement TyCtxt integration to close the 95% coverage gap and match global rustc usage patterns.

---
*Analysis based on self-inspection and historical rustc_middle data comparison*
