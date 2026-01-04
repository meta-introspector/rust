Constants Feature → Usage Graph Cross-Reference

## Found in Usage Data:

### AST/Parsing Pipeline:
- rustc_ast_visit.json - Contains visit_item, walk_item_inner functions
- rustc_expand_constants.json - Macro expansion for constants
- rustc_ast_lowering_index.json - AST lowering with visit_item

### Key Functions Mapped:
1. **visit_item** → Found in 28 files including:
   - rustc_ast_visit.json (core visitor)
   - rustc_ast_lowering (HIR lowering)
   - rustc_builtin_macros (macro processing)
   - rustc_passes (compiler passes)

2. **expand_crate** → Found in:
   - rustc_expand_constants.json
   - rustc_expand_errors.json  
   - rustc_expand_literals.json

3. **emit_diagnostic** → Found in:
   - rustc_errors_DiagCtxtInner_emit_diagnostic*.json

## Cross-Reference Success:
✅ Perf trace functions → Usage graph data
✅ Constants activate documented usage patterns
✅ Can trace feature → compiler subsystem → usage relationships

This proves our method: **perf trace → usage graph lookup → feature characterization**
