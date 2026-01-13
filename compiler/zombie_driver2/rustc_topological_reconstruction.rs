// Rustc Reconstruction from Topological Lattice
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct RustcReconstruction {
    bfs_traversal: Vec<FunctionNode>,
    dfs_traversal: Vec<FunctionNode>,
    topological_lattice: TopologicalLattice,
    reconstruction_graph: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionNode {
    name: String,
    coordinates: (f64, f64, f64),
    monster_signature: Vec<u64>,
    connections: Vec<String>,
    lattice_position: String,
    reconstruction_order: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TopologicalLattice {
    layers: Vec<LatticeLayer>,
    total_functions: usize,
    monster_flow: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct LatticeLayer {
    layer_id: u32,
    functions: Vec<String>,
    monster_correlation: f64,
    semantic_meaning: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 RUSTC RECONSTRUCTION FROM TOPOLOGICAL LATTICE");
    println!("=================================================");

    // Load topology data
    let topology = load_monster_topology()?;
    println!("📦 Loaded topology: {} nodes", topology["nodes"].as_array().unwrap().len());

    // Build function graph
    let function_graph = build_function_graph(&topology);
    println!("🕸️ Built function graph: {} functions", function_graph.len());

    // Find entry points
    let entry_points = find_entry_points(&function_graph);
    println!("🎯 Found {} entry points", entry_points.len());

    // BFS reconstruction
    let bfs_result = bfs_reconstruction(&function_graph, &entry_points);
    println!("🔍 BFS traversal: {} nodes", bfs_result.len());

    // DFS reconstruction
    let dfs_result = dfs_reconstruction(&function_graph, &entry_points);
    println!("🌊 DFS traversal: {} nodes", dfs_result.len());

    // Build topological lattice
    let lattice = build_topological_lattice(&topology, &bfs_result, &dfs_result);

    // Create reconstruction
    let reconstruction = RustcReconstruction {
        bfs_traversal: bfs_result,
        dfs_traversal: dfs_result,
        topological_lattice: lattice,
        reconstruction_graph: function_graph,
    };

    // Display lattice
    display_topological_lattice(&reconstruction);

    // Save reconstruction
    save_reconstruction(&reconstruction)?;

    Ok(())
}

fn load_monster_topology() -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("unified_monster_topology.json")?;
    Ok(serde_json::from_str(&content)?)
}

fn build_function_graph(topology: &Value) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    if let Some(nodes) = topology["nodes"].as_array() {
        for node in nodes {
            if let (Some(name), Some(connections)) =
                (node["symbol_name"].as_str(), node["topology_connections"].as_array())
            {
                let connections_vec: Vec<String> =
                    connections.iter().filter_map(|c| c.as_str()).map(|s| s.to_string()).collect();

                graph.insert(name.to_string(), connections_vec);
            }
        }
    }

    graph
}

fn find_entry_points(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut entry_points = Vec::new();

    // Find functions with "main", "driver", "start" in name
    for name in graph.keys() {
        if name.contains("main") || name.contains("driver") || name.contains("start") {
            entry_points.push(name.clone());
        }
    }

    // If no obvious entry points, use most connected functions
    if entry_points.is_empty() {
        let mut by_connections: Vec<_> =
            graph.iter().map(|(name, connections)| (name.clone(), connections.len())).collect();
        by_connections.sort_by(|a, b| b.1.cmp(&a.1));

        entry_points = by_connections.iter().take(3).map(|(name, _)| name.clone()).collect();
    }

    entry_points
}

fn bfs_reconstruction(
    graph: &HashMap<String, Vec<String>>,
    entry_points: &[String],
) -> Vec<FunctionNode> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    let mut order = 0;

    // Start BFS from all entry points
    for entry in entry_points {
        if graph.contains_key(entry) {
            queue.push_back((entry.clone(), 0));
        }
    }

    while let Some((current, depth)) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());

        // Create function node
        let node = FunctionNode {
            name: current.clone(),
            coordinates: (0.0, 0.0, depth as f64), // BFS assigns depth as Z coordinate
            monster_signature: vec![0; 35],        // Will be filled from topology data
            connections: graph.get(&current).cloned().unwrap_or_default(),
            lattice_position: format!("BFS_L{}", depth),
            reconstruction_order: order,
        };

        result.push(node);
        order += 1;

        // Add neighbors to queue
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) && graph.contains_key(neighbor) {
                    queue.push_back((neighbor.clone(), depth + 1));
                }
            }
        }

        if result.len() >= 100 {
            // Limit for demo
            break;
        }
    }

    result
}

fn dfs_reconstruction(
    graph: &HashMap<String, Vec<String>>,
    entry_points: &[String],
) -> Vec<FunctionNode> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    let mut order = 0;

    for entry in entry_points {
        if graph.contains_key(entry) {
            dfs_recursive(graph, entry, &mut visited, &mut result, &mut order, 0);
        }
    }

    result
}

fn dfs_recursive(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    visited: &mut HashSet<String>,
    result: &mut Vec<FunctionNode>,
    order: &mut u32,
    depth: u32,
) {
    if visited.contains(current) || result.len() >= 100 || depth > 20 {
        return;
    }

    visited.insert(current.to_string());

    // Create function node
    let node = FunctionNode {
        name: current.to_string(),
        coordinates: (depth as f64, 0.0, 0.0), // DFS assigns depth as X coordinate
        monster_signature: vec![0; 35],
        connections: graph.get(current).cloned().unwrap_or_default(),
        lattice_position: format!("DFS_D{}", depth),
        reconstruction_order: *order,
    };

    result.push(node);
    *order += 1;

    // Recurse into neighbors
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            if graph.contains_key(neighbor) {
                dfs_recursive(graph, neighbor, visited, result, order, depth + 1);
            }
        }
    }
}

fn build_topological_lattice(
    topology: &Value,
    bfs_result: &[FunctionNode],
    dfs_result: &[FunctionNode],
) -> TopologicalLattice {
    let mut layers = Vec::new();

    // Group BFS nodes by depth (lattice layers)
    let mut depth_groups: HashMap<u32, Vec<String>> = HashMap::new();
    for node in bfs_result {
        let depth = node.coordinates.2 as u32;
        depth_groups.entry(depth).or_insert_with(Vec::new).push(node.name.clone());
    }

    // Create lattice layers
    for (depth, functions) in depth_groups {
        let monster_correlation = calculate_layer_monster_correlation(topology, &functions);
        let semantic_meaning = determine_layer_semantics(&functions, depth);

        layers.push(LatticeLayer {
            layer_id: depth,
            functions,
            monster_correlation,
            semantic_meaning,
        });
    }

    layers.sort_by_key(|l| l.layer_id);

    let total_functions = bfs_result.len() + dfs_result.len();
    let monster_flow = layers.iter().map(|l| l.monster_correlation).sum();

    TopologicalLattice { layers, total_functions, monster_flow }
}

fn calculate_layer_monster_correlation(topology: &Value, functions: &[String]) -> f64 {
    let mut total_correlation = 0.0;

    if let Some(nodes) = topology["nodes"].as_array() {
        for node in nodes {
            if let Some(name) = node["symbol_name"].as_str() {
                if functions.contains(&name.to_string()) {
                    if let Some(phi_score) = node["phi_score"].as_f64() {
                        total_correlation += phi_score;
                    }
                }
            }
        }
    }

    total_correlation / functions.len().max(1) as f64
}

fn determine_layer_semantics(functions: &[String], depth: u32) -> String {
    let mut semantics = Vec::new();

    // Analyze function names for semantic patterns
    let mut has_main = false;
    let mut has_compile = false;
    let mut has_parse = false;
    let mut has_driver = false;

    for func in functions {
        if func.contains("main") {
            has_main = true;
        }
        if func.contains("compile") {
            has_compile = true;
        }
        if func.contains("parse") {
            has_parse = true;
        }
        if func.contains("driver") {
            has_driver = true;
        }
    }

    match depth {
        0 => {
            if has_main {
                semantics.push("entry_layer");
            }
            if has_driver {
                semantics.push("driver_initialization");
            }
        }
        1..=2 => {
            if has_compile {
                semantics.push("compilation_core");
            }
            if has_parse {
                semantics.push("parsing_layer");
            }
        }
        3..=5 => semantics.push("processing_layer"),
        _ => semantics.push("deep_execution"),
    }

    if semantics.is_empty() {
        semantics.push("unknown_layer");
    }

    semantics.join("_")
}

fn display_topological_lattice(reconstruction: &RustcReconstruction) {
    println!("\n🏗️ RUSTC TOPOLOGICAL LATTICE:");
    println!("=============================");

    println!("   📊 Reconstruction Summary:");
    println!("     BFS nodes: {}", reconstruction.bfs_traversal.len());
    println!("     DFS nodes: {}", reconstruction.dfs_traversal.len());
    println!("     Lattice layers: {}", reconstruction.topological_lattice.layers.len());
    println!("     Total monster flow: {:.2}", reconstruction.topological_lattice.monster_flow);

    println!("\n   🔄 BFS Traversal Order:");
    for (i, node) in reconstruction.bfs_traversal.iter().take(10).enumerate() {
        let short_name = if node.name.len() > 40 { &node.name[..40] } else { &node.name };
        println!("     {}: {} [{}]", i + 1, short_name, node.lattice_position);
    }

    println!("\n   🌊 DFS Traversal Order:");
    for (i, node) in reconstruction.dfs_traversal.iter().take(10).enumerate() {
        let short_name = if node.name.len() > 40 { &node.name[..40] } else { &node.name };
        println!("     {}: {} [{}]", i + 1, short_name, node.lattice_position);
    }

    println!("\n   🏛️ Lattice Layers:");
    for layer in &reconstruction.topological_lattice.layers {
        println!(
            "     Layer {}: {} functions - {} (Monster: {:.3})",
            layer.layer_id,
            layer.functions.len(),
            layer.semantic_meaning,
            layer.monster_correlation
        );

        // Show first few functions in layer
        for func in layer.functions.iter().take(3) {
            let short_name = if func.len() > 30 { &func[..30] } else { func };
            println!("       - {}", short_name);
        }
    }

    println!("\n   🎯 Reconstruction Graph Connectivity:");
    let total_connections: usize =
        reconstruction.reconstruction_graph.values().map(|connections| connections.len()).sum();
    println!("     Total connections: {}", total_connections);
    println!(
        "     Average connections per function: {:.1}",
        total_connections as f64 / reconstruction.reconstruction_graph.len() as f64
    );
}

fn save_reconstruction(
    reconstruction: &RustcReconstruction,
) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete reconstruction
    let json = serde_json::to_string_pretty(reconstruction)?;
    fs::write("rustc_topological_reconstruction.json", json)?;

    // Save BFS/DFS comparison CSV
    let mut csv_content =
        String::from("traversal_type,order,function_name,lattice_position,connections\n");

    for node in &reconstruction.bfs_traversal {
        let short_name = node.name.replace(',', ";");
        csv_content.push_str(&format!(
            "BFS,{},{},{},{}\n",
            node.reconstruction_order,
            short_name,
            node.lattice_position,
            node.connections.len()
        ));
    }

    for node in &reconstruction.dfs_traversal {
        let short_name = node.name.replace(',', ";");
        csv_content.push_str(&format!(
            "DFS,{},{},{},{}\n",
            node.reconstruction_order,
            short_name,
            node.lattice_position,
            node.connections.len()
        ));
    }

    fs::write("rustc_traversal_comparison.csv", csv_content)?;

    // Save lattice structure
    let mut lattice_csv =
        String::from("layer_id,function_count,monster_correlation,semantic_meaning,functions\n");
    for layer in &reconstruction.topological_lattice.layers {
        let functions_str = layer.functions.iter().take(5).cloned().collect::<Vec<_>>().join(";");
        lattice_csv.push_str(&format!(
            "{},{},{:.4},{},{}\n",
            layer.layer_id,
            layer.functions.len(),
            layer.monster_correlation,
            layer.semantic_meaning,
            functions_str
        ));
    }
    fs::write("rustc_lattice_structure.csv", lattice_csv)?;

    println!("\n💾 RUSTC RECONSTRUCTION SAVED:");
    println!("==============================");
    println!("   Complete reconstruction: rustc_topological_reconstruction.json");
    println!("   BFS/DFS comparison: rustc_traversal_comparison.csv");
    println!("   Lattice structure: rustc_lattice_structure.csv");

    Ok(())
}
