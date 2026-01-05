# Usage Eigenmatrix - Experimental Rust Programs

A collection of experimental Rust programs exploring mathematical concepts, prime number patterns, quantum mechanics metaphors, and computational structures.

## Recent Files (Last 24 Hours)

### Core Programs

- **`observe_bits.rs`** - Quantum state collapse simulation through bit observation
- **`prime_sieve_table.rs`** - Prime sieve structure table generator
- **`syn_prime_analyzer.rs`** - AST analysis with prime pattern matching
- **`clean_graph_interpreter.rs`** - Graph data interpreter with execution tracing

### Binary Programs (`src/bin/`)

- **`mkscp_macro_set.rs`** - SCP Foundation anomalous macro system (experimental)
- **`uncontainable_meme.rs`** - Memetic pattern analysis
- **`flexible_scp_system.rs`** - Flexible SCP classification system
- **`recursive_ast_classifier.rs`** - Recursive AST classification engine

### Mathematical Libraries

- **`prime_constant_programs.rs`** - Prime constant computation programs
- **`prime_consistency_prover.rs`** - Prime consistency verification
- **`prime_kleene_algebras.rs`** - Kleene algebra operations on primes
- **`prime_eigenforms.rs`** - Eigenform analysis for prime patterns
- **`monk_secretome.rs`** - Monastic secretome analysis
- **`hierarchical_models.rs`** - Hierarchical mathematical models
- **`self_improving_eigenform.rs`** - Self-improving eigenform systems

## Quick Start

```bash
# Run individual programs
cargo run --bin observe_bits
cargo run --bin prime_sieve_table
cargo run --bin clean_graph_interpreter

# Run library programs directly
rustc observe_bits.rs && ./observe_bits
rustc prime_sieve_table.rs && ./prime_sieve_table
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test prime_tests
cargo test quantum_tests
cargo test graph_tests
```

## Architecture

The codebase explores several interconnected themes:

1. **Prime Number Patterns** - Various approaches to prime analysis and classification
2. **Quantum Metaphors** - Using quantum mechanics concepts for computational models
3. **Graph Structures** - Call graph analysis and execution tracing
4. **AST Analysis** - Syntax tree parsing and pattern matching
5. **Experimental Macros** - Advanced macro systems (some commented due to compilation issues)

## Dependencies

- `syn` - Rust syntax parsing
- `serde_json` - JSON serialization
- `anyhow` - Error handling
- `paste` - Token pasting for macros

## Status

Most programs are experimental and may require additional dependencies or data files to run completely. The macro system in `mkscp_macro_set.rs` is currently commented out due to compilation issues.
