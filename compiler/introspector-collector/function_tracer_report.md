# Function Tracer Report

## Runtime Tracing → Usage Index Mapping

- **Total indexed functions**: 0
- **Tracing method**: strace (system calls)
- **Next steps**: Use rustc compiler tracing for actual function calls

## Top Functions by Crate


## Integration Strategy

1. **Compiler Tracing**: Use rustc's built-in tracing for function calls
2. **Symbol Matching**: Map traced functions to usage index
3. **Performance Analysis**: Correlate runtime performance with usage patterns
4. **Optimization**: Identify hot paths in compilation
