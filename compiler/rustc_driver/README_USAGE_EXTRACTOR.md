# Rust Symbol Usage Extractor

## Overview
This tool extracts real symbol usage relationships from Rust code using the compiler's type checker. It shows exactly which symbols use which other symbols across crates and modules.

## Format
The output follows the pattern: `crate::module::decl::ast USES crate::module::decl::ast`

## Example Output
```
REAL_USES: crate::complex_test::root::process_data USES crate::complex_test::utils::Helper::new (AssocFn)
REAL_USES: crate::complex_test::root::process_data USES crate::std::std::collections::HashMap::<K, V>::new (AssocFn)
```

## Files
- `usage_prover.rs` - Main extractor using rustc's TypeckResults
- `complex_test.rs` - Test file with cross-module usage
- `test_usage.rs` - Simple test file

## Key Features
- **Real data only** - Uses rustc's TypeckResults, no fake connections
- **Cross-crate tracking** - Shows usage from local code to external crates
- **Cross-module tracking** - Shows usage between modules within same crate
- **Full qualification** - Every symbol has complete crate::module::decl path
- **Type information** - Shows what kind of symbol is being used (AssocFn, etc.)

## Usage
```bash
rustc usage_prover.rs -o usage_prover
./usage_prover --crate-type lib your_file.rs
```

## Technical Details
- Uses `rustc_interface::Callbacks::after_analysis` hook
- Accesses `TyCtxt::typeck()` for type checker results  
- Iterates through `TypeckResults::type_dependent_defs()` for real usage data
- Filters items by `DefKind` to only process items with bodies
- Extracts `DefId` and resolves to full paths using `def_path_str()`

This provides the foundation for extracting Rust idioms by analyzing real usage patterns across the entire dependency graph.
