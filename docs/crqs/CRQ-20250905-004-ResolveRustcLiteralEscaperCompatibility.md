# CRQ-20250905-004: Resolve `rustc-literal-escaper` Compatibility Issues

## Description
Investigate and resolve the compilation errors related to `rustc-literal-escaper` (e.g., `unescape_str` not found, `EscapeError` and `MixedUnit` unresolved, `is_fatal` method missing) when building the `rustc` workspace.

## Justification
`rustc-literal-escaper` is a critical dependency for core compiler components (`rustc_proc_macro`, `rustc_ast`), and its incompatibility is blocking the successful compilation of the `rustc` workspace. Resolution of this issue is a prerequisite for further crate integration.

## Assigned Agent
Specialized "Dependency Resolution Engineer" Agent (or a tool capable of analyzing Rust code and dependencies).

## Due Date
End of Day, September 6, 2025.

## Proposed Solution
Vendorize the `rustc-literal-escaper` crate using the source from `https://github.com/rust-lang/literal-escaper`.

## Deliverables
*   Analysis report detailing the root cause of the incompatibility (e.g., API changes, version mismatch, missing conditional compilation flags).
*   Proposed solution (e.g., updating the vendored `rustc-literal-escaper` to a compatible version, applying necessary patches to the vendored code, adjusting build configurations).
*   Successful compilation of the `rustc` workspace after applying the fix.

## Status
Open.
