Empty Set vs Step 1 Comparison

Compilation Metrics:
- Empty: 0.085s, 5,596 bytes, 0 warnings
- Constants: 0.096s, 6,608 bytes, 6 warnings
- Delta: +12.9% time, +18.1% size

MIR Output:
- Empty: 3 lines (header only)
- Constants: 64 lines (6 const definitions with basic blocks)

Archive Contents:
- Both: lib.rmeta + single .rcgu.o file
- Different CGU hashes due to content

Disassembly:
- Identical structure, different filenames only
- No executable code generated (library crate)

Key Insight: Constants exist purely in metadata and MIR - no runtime code generated, but significant compile-time processing overhead for type checking and dead code analysis.
