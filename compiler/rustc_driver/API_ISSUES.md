# API Compatibility Issues

## Current Status
We have a working `usage_collector.rs` that successfully extracts symbol usage data from rustc. However, when trying to create an enhanced version, we hit multiple rustc API compatibility issues.

## Working Approach
The current `usage_collector.rs` works and has generated:
- **8,642 files processed** from rustc codebase  
- **13,516 unique symbols** identified
- **144,703 total usages** captured
- Complete ontology in JSON/TOML/RDF formats

## API Issues Encountered

### 1. rustc_driver API Changes
```rust
// Old API (doesn't work)
rustc_driver::RunCompiler::new(&args, &mut collector).run()

// Current API (unknown - needs investigation)
rustc_driver::run_compiler(&args, &mut callbacks)
```

### 2. Queries Type Location
```rust
// Tried: rustc_interface::Queries<'tcx> - not found
// Tried: rustc_interface::interface::Queries<'tcx> - not found
// Need to find correct import path
```

### 3. TypeckResults Iteration
```rust
// Doesn't work - not directly iterable
for (hir_id, def_id_opt) in typeck_results.type_dependent_defs() {

// Need to find correct iteration method
```

### 4. HIR API Changes
```rust
// Old API (doesn't work)
tcx.hir().get(hir_id)
tcx.hir().items()

// New API (from usage data analysis)
tcx.hir_node(hir_id)           // 18 uses
tcx.hir_crate_items(())        // 27 uses  
tcx.hir_expect_item(def_id)    // 22 uses
```

### 5. ItemKind Pattern Changes
```rust
// Old patterns have wrong field counts
ItemKind::Const(ty, generics, _) => // Expected 4 fields, found 3
ItemKind::Static(ty, mutability, _) => // Expected 4 fields, found 3
ItemKind::Enum(enum_def, _) => // Expected 3 fields, found 2
```

## Solution Strategy
1. **Keep using the working `usage_collector.rs`** for now
2. **Use the ontology data** to guide what to collect (we have the frequency data)
3. **Gradually update APIs** one at a time using the usage patterns we discovered
4. **Focus on the high-value symbols** we identified:
   - `TyCtxt::dcx` (373 uses)
   - `TyCtxt::def_span` (274 uses) 
   - `TypeckResults::node_type` (15 uses)
   - `Level::TRACE/DEBUG` (3307/2798 uses)

## Next Steps
1. Use the working collector to get more data
2. Analyze the ontology to find the most important patterns
3. Create targeted collectors for specific high-value symbols
4. Build the semantic web incrementally rather than trying to fix all APIs at once

The **eigenform feedback loop** is working - we're learning from the system to improve the system!
