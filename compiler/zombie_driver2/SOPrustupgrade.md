# SOP: Rust Compiler API Upgrade Protocol

## SOP-RUST-001: Rustc Interface Migration

### Problem Statement
The zombie rustc driver encountered compilation failures due to API changes in the Rust compiler internals between versions. The `rustc_interface::Queries` type was moved/renamed, breaking existing callback implementations.

### Root Cause Analysis
- **Issue**: `rustc_interface::Queries<'tcx>` type not found
- **Cause**: Rust compiler internal APIs evolved between versions
- **Impact**: Zombie driver cannot compile, blocking P2P compilation network

### Resolution Protocol

#### Step 1: API Discovery
```bash
# Search for correct Queries type location
grep -r "Queries" ../introspector-collector/src/ --include="*.rs"
```

#### Step 2: Reference Implementation Analysis
Examine working collector implementations:
- `../introspector-collector/src/working_usage_collector_refactored.rs`
- Look for `impl Callbacks` patterns
- Identify correct import paths

#### Step 3: Correct Import Pattern
```rust
use rustc_interface::interface;

// Correct usage:
queries: &'tcx interface::Queries<'tcx>

// NOT:
queries: &'tcx rustc_interface::Queries<'tcx>
```

#### Step 4: Callback Implementation Template
```rust
impl Callbacks for ZombieCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            // Zombie infection logic here
        });
        rustc_driver::Compilation::Continue
    }
}
```

### Containment Measures
1. **Version Lock**: Pin rustc dependencies to known working versions
2. **API Monitoring**: Track rustc nightly changes that affect driver APIs
3. **Fallback Strategy**: Maintain simplified callback implementations for compatibility

### Prevention
- Always reference working examples in the same codebase
- Use `interface::` prefix for rustc_interface types
- Test builds after any rustc version updates

### Status: ACTIVE CONTAINMENT
The zombie driver requires immediate API fixes to restore P2P compilation network functionality.
