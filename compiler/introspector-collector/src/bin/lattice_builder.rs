use introspector_collector::rust_lattice::*;
use introspector_collector::reconstruction_engine::*;

fn main() {
    println!("Building complete Rust code lattice...");
    
    let mut lattice = RustLattice::new();
    let reconstruction_engine = ReconstructionEngine::new();
    
    // Sample Rust snippets to ingest into lattice
    let snippets = vec![
        "enum Color { Red, Green, Blue }",
        "fn paint(color: Color) -> String { \"painted\" }",
        "struct Point { x: i32, y: i32 }",
        "enum Option<T> { None, Some(T) }",
        "fn unwrap<T>(opt: Option<T>) -> T { match opt { Some(v) => v, None => panic!() } }",
        "macro_rules! vec { ($($x:expr),*) => { Vec::from([$($x),*]) }; }",
        "mod graphics { pub use super::Color; }",
        "let x = Color::Red;",
        "match color { Red => 1, Green => 2, Blue => 3 }",
    ];
    
    // Ingest all snippets into the lattice
    lattice.ingest_all_snippets(snippets);
    
    println!("Initial lattice state:");
    println!("{}", lattice.generate_summary());
    
    // Show identified holes
    println!("\nIdentified holes to fill:");
    for hole in &lattice.holes {
        println!("  - {}", hole);
    }
    
    // Fill the holes
    println!("\nFilling holes...");
    lattice.fill_holes(&reconstruction_engine);
    
    println!("Final lattice state:");
    println!("{}", lattice.generate_summary());
    
    // Query for specific patterns
    println!("\nQuerying lattice:");
    let enum_nodes = lattice.query_pattern("enum");
    println!("Enum constructs: {:?}", enum_nodes);
    
    let function_nodes = lattice.query_pattern("fn ");
    println!("Function constructs: {:?}", function_nodes);
    
    // Show connections
    println!("\nLattice connections:");
    for (from, connections) in &lattice.connections {
        if !connections.is_empty() {
            println!("  {} -> {:?}", from, connections);
        }
    }
    
    println!("\nLattice construction complete!");
    println!("The lattice now contains a complete mapping of Rust constructs with filled holes.");
}
