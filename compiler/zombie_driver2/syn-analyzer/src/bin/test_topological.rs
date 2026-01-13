use syn_analyzer::analyze_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <rust_file>", args[0]);
        return;
    }

    let file_path = &args[1];
    match std::fs::read_to_string(file_path) {
        Ok(content) => match analyze_file(file_path, &content) {
            Ok(analysis) => {
                println!("=== TOPOLOGICAL JSON ANALYSIS ===");
                println!("File: {}", analysis.file_path);
                println!("Total nodes: {}", analysis.total_nodes);
                println!("Node counts: {:#?}", analysis.syn_node_counts);
                println!("\n=== JSON PATHS (first 20) ===");
                for (i, path) in analysis.json_paths.iter().take(20).enumerate() {
                    println!("{}: {}", i, path);
                }
                println!("\n=== PATH MATRIX (first 10) ===");
                for (i, (path, count)) in analysis.path_matrix.iter().take(10).enumerate() {
                    println!("{}: {} -> {}", i, path, count);
                }
            }
            Err(e) => println!("Error: {}", e),
        },
        Err(e) => println!("Failed to read file: {}", e),
    }
}
