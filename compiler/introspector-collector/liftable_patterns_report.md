🔍 LIFTABLE USAGE PATTERNS FOUND IN EXISTING DATA
================================================

## 🎯 PATTERN MATCHING & CONTROL FLOW (27 total occurrences)
- `add_impl_generic` (16 uses) - Generic implementation patterns
- `lifetimes` (6 uses) - Lifetime parameter usage  
- `inner_type` (3 uses) - Type extraction patterns
- `Mismatch` (1 use) - Pattern matching failure
- `QueryModifiers` (1 use) - Query pattern modifiers

**Implementation Priority: HIGH**
```rust
// Add to working_usage_collector.rs:
rustc_hir::ExprKind::Match(expr, arms, _) => {
    self.add_usage(module, "match_expr".to_string(), "pattern_match".to_string(),
                   "PatternMatch".to_string(), "Expression".to_string(), ...);
}
```

## 🔧 TYPE SYSTEM PATTERNS (8 total occurrences)  
- `TypeUsageVisitor` (2 uses) - Type visitor patterns
- `UsageClassification` (1 use) - Type classification
- `TypeVisitable`, `TypeFoldable` (2 uses) - Type traversal traits
- `Newtype` (1 use) - Newtype pattern usage

**Implementation Priority: HIGH**
```rust
// Add to working_usage_collector.rs:
rustc_hir::TyKind::Path(_) => {
    self.add_usage(module, ty_name, "type_usage".to_string(),
                   "TypeAnnotation".to_string(), "Type".to_string(), ...);
}
```

## 🏗️ FIELD ACCESS PATTERNS (71 total occurrences)
- `index` (18 uses) - Array/collection indexing
- `get_unchecked_mut` (15 uses) - Unsafe field access
- `__FieldVisitor`, `FIELDS`, `__Field` (30 uses) - Field enumeration
- `FieldInfo` (2 uses) - Field metadata
- `IndexVec`, `IndexSlice` (2 uses) - Indexed data structures

**Implementation Priority: MEDIUM**
```rust
// Add to working_usage_collector.rs:
rustc_hir::ExprKind::Field(expr, field) => {
    self.add_usage(module, field.name.to_string(), "field_access".to_string(),
                   "FieldAccess".to_string(), "Expression".to_string(), ...);
}
```

## 🔒 MEMORY SAFETY PATTERNS (31 total occurrences)
- `get_unchecked_mut` (15 uses) - Unsafe mutable access
- `as_ref` (6 uses) - Reference conversion
- `iter_mut`, `deref_mut`, `borrow_mut` (5 uses) - Mutable operations
- `call_mut`, `deref` (2 uses) - Mutation patterns

**Implementation Priority: MEDIUM**
```rust
// Add to working_usage_collector.rs:
rustc_hir::ExprKind::Call(func, args) if is_unsafe_call(func) => {
    self.add_usage(module, "unsafe_call".to_string(), "memory_safety".to_string(),
                   "UnsafeUsage".to_string(), "Expression".to_string(), ...);
}
```

## 📊 MACRO & ATTRIBUTE PATTERNS (28 total occurrences)
- `custom_derive` (20 uses) - Custom derive macros
- `DiagnosticDerive*` (5 uses) - Diagnostic derive patterns  
- `attr` (1 use) - Attribute usage
- `Attributes` (1 use) - Attribute collections

**Implementation Priority: LOW (already partially covered)**
```rust
// Expand existing macro coverage in working_usage_collector.rs:
rustc_ast::AttrKind::Normal(attr) => {
    self.add_usage(module, attr_name, "attribute_usage".to_string(),
                   "AttributeUsage".to_string(), "Item".to_string(), ...);
}
```

## 💡 IMMEDIATE ACTIONABLE ITEMS:

### 1. **Add Pattern Matching Support** (27 occurrences waiting)
- Track `match` expressions and pattern complexity
- Capture `if let` and destructuring patterns

### 2. **Expand Type System Coverage** (8 occurrences waiting)  
- Add type annotation tracking
- Implement generic parameter analysis

### 3. **Implement Field Access Tracking** (71 occurrences waiting)
- Track field access patterns
- Monitor unsafe field operations

### 4. **Add Memory Safety Analysis** (31 occurrences waiting)
- Track unsafe operations
- Monitor mutable reference patterns

**Total Liftable Patterns: 165 occurrences**
**Current Coverage Gap: ~33% of potential usage patterns**

These patterns are already present in the codebase but not being captured by working_usage_collector.rs. Adding support for them would significantly expand the correlation data available for runtime performance analysis.
