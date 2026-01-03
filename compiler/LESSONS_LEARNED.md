# Usage Collector Updates - Lessons Learned

## What We Added
- **Constants Collection**: Captures `const` and `static` declarations + their literal values
- **Struct Collection**: Records struct definitions
- **Enum Collection**: Identifies enum declarations
- **Numeric Literals**: Extracts actual numeric values from const/static bodies

## Key Technical Lessons

### HIR Access Patterns
- Use `tcx.hir_body(body_id)` not `tcx.hir().body()`
- Use `tcx.hir_expect_item()` for item access
- Use `tcx.hir_crate_items()` for iteration

### Item Pattern Matching
```rust
// CORRECT patterns for const/static:
rustc_hir::ItemKind::Const(_, _, _, body_id) => // body_id is 4th param
rustc_hir::ItemKind::Static(_, _, _, body_id) => // body_id is 4th param

// Skip problematic items:
rustc_hir::ItemKind::Use(..) => continue,
rustc_hir::ItemKind::ExternCrate(..) => continue, 
rustc_hir::ItemKind::Impl(..) => continue,
```

### Literal Extraction
- Use HIR visitor pattern to walk expression trees
- Access literal values via `rustc_ast::LitKind` enum
- Extract numeric values: `LitKind::Int(i, _)` and `LitKind::Float(f, _)`

## Output Structure
Generated files:
- `*_constants.json` - const/static declarations
- `*_literals.json` - numeric literal values
- `*_structs.json` - struct definitions  
- `*_enums.json` - enum definitions

## Debugging Tips
- Use `DUMP_HIR=1` env var for HIR inspection
- Pattern match errors usually mean wrong field count
- Type errors often indicate wrong HIR access method
