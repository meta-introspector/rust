📊 === HIR Syn Walker Test Results ===
Generated: Tue Jan  6 11:37:59 AM EST 2026

## Results from rustc_lint_results.txt
```
🔍 Discovering syn-related HIR data...
📁 Loaded rustc_lint_lifetime_syntax.json: 218 usages, 1 syn patterns
📁 Loaded rustc_lint_literals.json: 1123 usages, 3 syn patterns
📁 Loaded rustc_mir_transform_shim_async_destructor_ctor.json: 250 usages, 1 syn patterns
📁 Loaded rustc_trait_selection_traits_project.json: 1277 usages, 0 syn patterns
📁 Loaded rustc_mir_transform_coroutine_drop.json: 254 usages, 0 syn patterns
✅ Discovered 5 syn HIR nodes
🚶 Walking syn HIR data with detailed analysis...
📊 Total syn usage: 250, 90% cutoff: 225

🎯 Node 1: lifetime_syntax (usage: 50, accumulated: 50)
  📋 Top HIR symbols:
    • default_fields_for_macro (1)
    • None (1)
    • Some (1)
    • collect (1)
    • next (1)
  🔍 Syn patterns found:
    • lifetime_syntax_category

🎯 Node 2: literals (usage: 50, accumulated: 100)
  📋 Top HIR symbols:
    • "CLOSURE_RETURNING_ASYNC_BLOCK" (1)
    • "closure that returns `async {}` could be rewritten as an async closure" (1)
    • false (1)
    • "ASYNC_FN_IN_TRAIT" (1)
    • "use of `async fn` in definition of a publicly-reachable trait" (1)
  🔍 Syn patterns found:
    • "closure that returns `async {}` could be rewritten as an async closure"
    • "use of `async fn` in definition of a publicly-reachable trait"
    • "enabling track_caller on an async fn is a no-op unless the async_fn_track_caller feature is enabled"

🎯 Node 3: async_destructor (usage: 50, accumulated: 150)
  📋 Top HIR symbols:
    • le (1)
    • DEBUG (1)
    • le (1)
    • DEBUG (1)
    • current (1)
  🔍 Syn patterns found:
    • async_drop_in_place_fn

🎯 Node 4: trait_project (usage: 50, accumulated: 200)
  📋 Top HIR symbols:
    • le (1)
    • DEBUG (1)
    • le (1)
    • DEBUG (1)
    • current (1)

🎯 Node 5: coroutine_drop (usage: 50, accumulated: 250)
  📋 Top HIR symbols:
    • require_lang_item (1)
    • new_fn_def (1)
    • new (1)
    • zero_sized (1)
    • clone (1)
🎯 Cutoff reached at 100% of total usage

🔧 Generating HIR-based syn walker code...
✅ Generated HIR-based syn walker: generated_syn_hir_walker.rs
```

## Results from rustc_mir_transform_results.txt
```
🔍 Discovering syn-related HIR data...
📁 Loaded rustc_mir_transform_shim_async_destructor_ctor.json: 250 usages, 1 syn patterns
📁 Loaded rustc_lint_literals.json: 1123 usages, 3 syn patterns
📁 Loaded rustc_mir_transform_shim_async_destructor_ctor.json: 250 usages, 1 syn patterns
📁 Loaded rustc_trait_selection_traits_project.json: 1277 usages, 0 syn patterns
📁 Loaded rustc_mir_transform_coroutine_drop.json: 254 usages, 0 syn patterns
✅ Discovered 5 syn HIR nodes
🚶 Walking syn HIR data with detailed analysis...
📊 Total syn usage: 250, 90% cutoff: 225

🎯 Node 1: lifetime_syntax (usage: 50, accumulated: 50)
  📋 Top HIR symbols:
    • le (1)
    • DEBUG (1)
    • le (1)
    • DEBUG (1)
    • current (1)
  🔍 Syn patterns found:
    • async_drop_in_place_fn

🎯 Node 2: literals (usage: 50, accumulated: 100)
  📋 Top HIR symbols:
    • "CLOSURE_RETURNING_ASYNC_BLOCK" (1)
    • "closure that returns `async {}` could be rewritten as an async closure" (1)
    • false (1)
    • "ASYNC_FN_IN_TRAIT" (1)
    • "use of `async fn` in definition of a publicly-reachable trait" (1)
  🔍 Syn patterns found:
    • "closure that returns `async {}` could be rewritten as an async closure"
    • "use of `async fn` in definition of a publicly-reachable trait"
    • "enabling track_caller on an async fn is a no-op unless the async_fn_track_caller feature is enabled"

🎯 Node 3: async_destructor (usage: 50, accumulated: 150)
  📋 Top HIR symbols:
    • le (1)
    • DEBUG (1)
    • le (1)
    • DEBUG (1)
    • current (1)
  🔍 Syn patterns found:
    • async_drop_in_place_fn

🎯 Node 4: trait_project (usage: 50, accumulated: 200)
  📋 Top HIR symbols:
    • le (1)
    • DEBUG (1)
    • le (1)
    • DEBUG (1)
    • current (1)

🎯 Node 5: coroutine_drop (usage: 50, accumulated: 250)
  📋 Top HIR symbols:
    • require_lang_item (1)
    • new_fn_def (1)
    • new (1)
    • zero_sized (1)
    • clone (1)
🎯 Cutoff reached at 100% of total usage

🔧 Generating HIR-based syn walker code...
✅ Generated HIR-based syn walker: generated_syn_hir_walker.rs
```

## Results from rustc_trait_selection_results.txt
```
🔍 Discovering syn-related HIR data...
📁 Loaded rustc_trait_selection_traits_project.json: 1277 usages, 0 syn patterns
📁 Loaded rustc_lint_literals.json: 1123 usages, 3 syn patterns
📁 Loaded rustc_trait_selection_traits_project.json: 1277 usages, 0 syn patterns
📁 Loaded rustc_trait_selection_traits_project.json: 1277 usages, 0 syn patterns
📁 Loaded rustc_mir_transform_coroutine_drop.json: 254 usages, 0 syn patterns
✅ Discovered 5 syn HIR nodes
🚶 Walking syn HIR data with detailed analysis...
📊 Total syn usage: 250, 90% cutoff: 225

🎯 Node 1: lifetime_syntax (usage: 50, accumulated: 50)
  📋 Top HIR symbols:
    • le (1)
    • DEBUG (1)
    • le (1)
    • DEBUG (1)
    • current (1)

🎯 Node 2: literals (usage: 50, accumulated: 100)
  📋 Top HIR symbols:
    • "CLOSURE_RETURNING_ASYNC_BLOCK" (1)
    • "closure that returns `async {}` could be rewritten as an async closure" (1)
    • false (1)
    • "ASYNC_FN_IN_TRAIT" (1)
    • "use of `async fn` in definition of a publicly-reachable trait" (1)
  🔍 Syn patterns found:
    • "closure that returns `async {}` could be rewritten as an async closure"
    • "use of `async fn` in definition of a publicly-reachable trait"
    • "enabling track_caller on an async fn is a no-op unless the async_fn_track_caller feature is enabled"

🎯 Node 3: async_destructor (usage: 50, accumulated: 150)
  📋 Top HIR symbols:
    • le (1)
    • DEBUG (1)
    • le (1)
    • DEBUG (1)
    • current (1)

🎯 Node 4: trait_project (usage: 50, accumulated: 200)
  📋 Top HIR symbols:
    • le (1)
    • DEBUG (1)
    • le (1)
    • DEBUG (1)
    • current (1)

🎯 Node 5: coroutine_drop (usage: 50, accumulated: 250)
  📋 Top HIR symbols:
    • require_lang_item (1)
    • new_fn_def (1)
    • new (1)
    • zero_sized (1)
    • clone (1)
🎯 Cutoff reached at 100% of total usage

🔧 Generating HIR-based syn walker code...
✅ Generated HIR-based syn walker: generated_syn_hir_walker.rs
```

📈 Performance Metrics:
- Total HIR nodes processed: 15
- Total syn patterns found: 13
- Average usage per node: 0
🔧 Generated Code Files:
- Total generated lines: 
