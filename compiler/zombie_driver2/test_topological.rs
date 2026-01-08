extern crate syn_analyzer;
extern crate serde_json;

fn main() {
    let content = std::fs::read_to_string("test_simple.rs").unwrap();
    match syn_analyzer::analyze_file("test_simple.rs", &content) {
        Ok(analysis) => {
            println!("=== TOPOLOGICAL JSON ANALYSIS ===");
            println!("Total nodes: {}", analysis.total_nodes);
            println!("Node counts: {:#?}", analysis.syn_node_counts);
            println!("\n=== JSON PATHS (first 10) ===");
            for (i, path) in analysis.json_paths.iter().take(10).enumerate() {
                println!("{}: {}", i, path);
            }
            println!("\n=== PATH MATRIX (first 10) ===");
            for (i, (path, count)) in analysis.path_matrix.iter().take(10).enumerate() {
                println!("{}: {} -> {}", i, path, count);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
