Full Stack Trace Analysis: Constants Compilation

Recording: 54 samples, 113.4M instructions, 0.578 MB trace data

Top Call Stack (44.63% of instructions):
0xffffffffffffffff (kernel)
├─ emit_diagnostic (13.59%)
│  ├─ DiagCtxtInner::emit_diagnostic
│  ├─ track_diagnostic callback
│  ├─ HumanEmitter::emit_diagnostic  
│  ├─ translate_message
│  ├─ Once::call (lazy initialization)
│  └─ FluentBundle parsing (10.87%)
│     └─ fluent_syntax parser
│        └─ get_attributes/get_pattern

Key Insights:
- 44.63% of instructions spent in diagnostic/error handling system
- 13.59% in error message emission pipeline
- 10.87% parsing Fluent localization bundles
- Heavy lazy initialization overhead for i18n system
- Constants trigger full diagnostic infrastructure even for warnings

This shows constants activate the complete error reporting system, including internationalization, even for simple dead code warnings.
