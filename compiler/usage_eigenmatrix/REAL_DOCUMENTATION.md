# Graph Interpreter Documentation

## What Actually Exists

### Files Created
- `graph_interpreter_callbacks.rs` - Main interpreter
- `simple_bfs.rs` - BFS traversal 
- `find_loops.rs` - Loop detection
- `call_strength.rs` - Call analysis
- `random_walk.rs` - Random execution
- `trace_static_meta.rs` - Debug tracer

### Real Data
- Usage data from rustc compilation in `usage_data/*.json`
- Call graph: 40,068 nodes loaded from actual usage files
- Symbol table: 12,808 entries built from DefId parsing
- Entry point: "static META" (highest eigenvalue from usage_eigenmatrix.json)

### Interpreter State
```rust
struct GraphInterpreterWithCallbacks {
    instruction_pointer: String,           // Current node
    registers: [String; 8],               // R0-R7
    stack: Vec<String>,                   // Call stack  
    heap: HashMap<String, String>,        // Memory
    graph: HashMap<String, Vec<String>>,  // Call relationships
    symbol_table: HashMap<String, String>, // DefId -> file path
    ast_cache: HashMap<String, String>,   // Loaded ASTs
}
```

### Instruction Types
- `CALL` - DefId nodes (function calls)
- `LOAD` - static nodes (constants)
- `LOAD_STR` - string literals (starts with ")
- `LOAD_NUM` - numeric literals (all digits)
- `LOAD_DATA` - everything else

### Actual Execution
1. Loads 40,068 nodes from usage JSON files
2. Builds symbol table from DefId patterns
3. Starts at "static META" 
4. Executes call graph as instructions
5. Panics on unhandled node types (no NOPs)

### Real Results
- BFS found 7,980 reachable nodes from main
- Loop detection found 9 self-loops, 0 mutual cycles
- Call strength shows "static META" calls 30,490 functions
- First call from "static META" is string literal "event compiler/rustc_infer/..."

### What Works
- Graph loading from usage data
- Basic instruction execution (CALL, LOAD variants)
- AST callback system (loads when DefId encountered)
- Symbol table mapping DefId to file paths

### What Doesn't Work Yet
- String literals cause execution to loop (no callees)
- AST loading is simulated (generates fake AST content)
- No actual file reading from symbol table paths
- Limited instruction set

### Usage
```bash
cargo run --bin graph_interpreter_callbacks  # Main interpreter
cargo run --bin simple_bfs                   # BFS traversal
cargo run --bin find_loops                   # Loop detection
```

This is a working prototype that loads real rustc usage data and executes it as a program.
