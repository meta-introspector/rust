# MEMO: The Hard Way is the Only Way

## Subject: Rust Symbol Usage Extraction - No Shortcuts Available

**Date:** January 3, 2026  
**From:** Development Team  
**To:** Project Stakeholders  

## Summary

After extensive investigation and implementation, we have conclusively determined that **there is no simpler approach** to extracting real symbol usage relationships from Rust code. The hard way is the only way.

## What We Tried (The "Simple" Approaches)

1. **Static AST parsing** - Only gives syntax, no resolution
2. **Cargo metadata** - Only shows crate dependencies, not symbol usage
3. **Text pattern matching** - Unreliable, misses qualified imports
4. **Third-party tools** - Don't provide the granular symbol-to-symbol relationships we need

## What Actually Works (The Hard Way)

**Using rustc's internal compiler infrastructure:**
- Hook into `rustc_interface::Callbacks::after_analysis`
- Access `TyCtxt::typeck()` for resolved type information
- Extract from `TypeckResults::type_dependent_defs()` 
- Parse `DefId` to full `crate::module::decl` paths

## Why This is Hard

1. **Requires rustc internals** - Unstable APIs, complex setup
2. **Must compile each crate** - Full type checking required
3. **Dependency resolution needed** - Can't shortcut the compiler pipeline
4. **Real symbol resolution** - Only available after full analysis phase

## The Reality

```rust
// This is what we need:
REAL_USES: crate::complex_test::root::process_data USES crate::std::collections::HashMap::<K, V>::new (AssocFn)

// This is what "simple" tools give us:
dependencies = ["std"]  // Useless for idiom extraction
```

## Conclusion

**There are no shortcuts.** To extract real Rust idioms and usage patterns, we must:

1. Build custom rustc drivers
2. Hook into the compiler's analysis phase  
3. Extract from resolved type information
4. Process the entire dependency graph

The tooling we've built (`usage_collector.rs`, `usage_prover.rs`) represents the **minimum viable complexity** for this task. Any simpler approach will not capture the real symbol-to-symbol relationships needed for meaningful idiom analysis.

## Recommendation

Proceed with the hard way. It's the only way that works.

---
*"In the kingdom of the blind, the one-eyed rustc driver is king."*
