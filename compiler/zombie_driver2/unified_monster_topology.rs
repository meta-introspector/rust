// Unified Monster Topology: Coordinates + N-grams + SYN Data
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct MonsterTopologyNode {
    coordinates: (f64, f64, f64),
    symbol_name: String,
    lattice_label: String,
    ngrams: Vec<NgramCoordinate>,
    syn_ast_data: Option<SynAstData>,
    monster_signature: Vec<u64>,
    phi_score: f64,
    topology_connections: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NgramCoordinate {
    ngram: String,
    ngram_coordinates: (f64, f64, f64), // Offset from main coordinates
    monster_correlation: f64,
    frequency: u32,
    source: String, // "label", "code", or "syn"
}

#[derive(Debug, Serialize, Deserialize)]
struct SynAstData {
    ast_type: String,
    ast_structure: Value,
    syn_ngrams: Vec<String>,
    ast_monster_correlation: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MonsterTopology {
    nodes: Vec<MonsterTopologyNode>,
    ngram_coordinate_map: HashMap<String, Vec<(f64, f64, f64)>>,
    topology_metrics: TopologyMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
struct TopologyMetrics {
    total_nodes: usize,
    total_ngrams: usize,
    syn_coverage: f64,
    monster_density: f64,
    topology_completeness: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 UNIFIED MONSTER TOPOLOGY: Coordinates + N-grams + SYN");
    println!("========================================================");

    // Load all data sources
    let lattice = load_lattice_system()?;
    let label_ngrams = load_label_ngrams()?;
    let string_data = load_string_data()?;
    let syn_data = load_syn_data()?;

    println!("📦 Loaded all data sources");

    // Build unified topology
    let topology = build_monster_topology(&lattice, &label_ngrams, &string_data, &syn_data)?;

    // Analyze topology completeness
    analyze_topology(&topology);

    // Save unified topology
    save_monster_topology(&topology)?;

    Ok(())
}

fn load_lattice_system() -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("monster_lattice_system.json")?;
    Ok(serde_json::from_str(&content)?)
}

fn load_label_ngrams() -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("label_ngram_monster_analysis.json")?;
    Ok(serde_json::from_str(&content)?)
}

fn load_string_data() -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("lattice_location_strings.json")?;
    Ok(serde_json::from_str(&content)?)
}

fn load_syn_data() -> Result<Value, Box<dyn std::error::Error>> {
    // Try to load existing SYN data
    if let Ok(content) = fs::read_to_string("rustc_ast_lmfdb_mapping.json") {
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(serde_json::json!({})) // Empty if not found
    }
}

fn build_monster_topology(
    lattice: &Value,
    label_ngrams: &Value,
    string_data: &Value,
    syn_data: &Value,
) -> Result<MonsterTopology, Box<dyn std::error::Error>> {
    println!("\n🌐 BUILDING UNIFIED MONSTER TOPOLOGY:");
    println!("====================================");

    let points = lattice["points"].as_array().unwrap();
    let mut nodes = Vec::new();
    let mut ngram_coordinate_map = HashMap::new();
    let mut syn_coverage_count = 0;

    for (i, point) in points.iter().enumerate().take(100) {
        // Process first 100 for demo
        if let (Some(coords), Some(symbol_name), Some(lattice_label), Some(phi_score)) = (
            point["coordinates"].as_array(),
            point["symbol_name"].as_str(),
            point["lattice_label"].as_str(),
            point["phi_score"].as_f64(),
        ) {
            let coordinates = (
                coords[0].as_f64().unwrap_or(0.0),
                coords[1].as_f64().unwrap_or(0.0),
                coords[2].as_f64().unwrap_or(0.0),
            );

            let monster_signature = if let Some(monster_sig) = point["monster_signature"].as_array()
            {
                monster_sig.iter().filter_map(|v| v.as_u64()).collect()
            } else {
                vec![0; 35] // Default signature
            };

            // Collect n-grams for this node
            let mut ngrams = Vec::new();

            // Add label n-grams
            add_label_ngrams(&mut ngrams, &mut ngram_coordinate_map, lattice_label, &coordinates);

            // Add string n-grams if available
            add_string_ngrams(
                &mut ngrams,
                &mut ngram_coordinate_map,
                string_data,
                symbol_name,
                &coordinates,
            );

            // Find SYN AST data for this symbol
            let syn_ast_data = find_syn_data_for_symbol(syn_data, symbol_name);
            if syn_ast_data.is_some() {
                syn_coverage_count += 1;

                // Add SYN n-grams
                if let Some(ref syn_data) = syn_ast_data {
                    add_syn_ngrams(&mut ngrams, &mut ngram_coordinate_map, syn_data, &coordinates);
                }
            }

            // Find topology connections (nearby nodes)
            let topology_connections = find_topology_connections(points, i, &coordinates);

            nodes.push(MonsterTopologyNode {
                coordinates,
                symbol_name: symbol_name.to_string(),
                lattice_label: lattice_label.to_string(),
                ngrams,
                syn_ast_data,
                monster_signature,
                phi_score,
                topology_connections,
            });
        }

        if (i + 1) % 20 == 0 {
            println!("   ✅ Processed {} topology nodes", i + 1);
        }
    }

    // Calculate topology metrics
    let total_ngrams: usize = nodes.iter().map(|n| n.ngrams.len()).sum();
    let syn_coverage = syn_coverage_count as f64 / nodes.len() as f64;
    let monster_density = calculate_monster_density(&nodes);
    let topology_completeness = calculate_topology_completeness(&nodes, &ngram_coordinate_map);

    let topology_metrics = TopologyMetrics {
        total_nodes: nodes.len(),
        total_ngrams,
        syn_coverage,
        monster_density,
        topology_completeness,
    };

    println!("   🎯 Built topology: {} nodes, {} n-grams", nodes.len(), total_ngrams);

    Ok(MonsterTopology { nodes, ngram_coordinate_map, topology_metrics })
}

fn add_label_ngrams(
    ngrams: &mut Vec<NgramCoordinate>,
    ngram_map: &mut HashMap<String, Vec<(f64, f64, f64)>>,
    lattice_label: &str,
    base_coords: &(f64, f64, f64),
) {
    // Generate n-grams from lattice label
    for n in 2..=6 {
        if lattice_label.len() >= n {
            for start in 0..=lattice_label.len() - n {
                let ngram = lattice_label[start..start + n].to_string();

                // Calculate offset coordinates for this n-gram
                let offset_coords = (
                    base_coords.0 + start as f64 * 0.1,
                    base_coords.1 + n as f64 * 0.1,
                    base_coords.2 + 0.1,
                );

                let monster_correlation = calculate_ngram_monster_correlation(&ngram);

                ngrams.push(NgramCoordinate {
                    ngram: ngram.clone(),
                    ngram_coordinates: offset_coords,
                    monster_correlation,
                    frequency: 1, // Will be updated later
                    source: "label".to_string(),
                });

                ngram_map.entry(ngram).or_insert_with(Vec::new).push(offset_coords);
            }
        }
    }
}

fn add_string_ngrams(
    ngrams: &mut Vec<NgramCoordinate>,
    ngram_map: &mut HashMap<String, Vec<(f64, f64, f64)>>,
    string_data: &Value,
    symbol_name: &str,
    base_coords: &(f64, f64, f64),
) {
    // Find string data for this symbol
    if let Some(locations) = string_data.as_array() {
        for location in locations {
            if let Some(label) = location["lattice_label"].as_str() {
                if label.contains(&symbol_name[..symbol_name.len().min(20)]) {
                    // Add string patterns as n-grams
                    if let Some(patterns) = location["string_patterns"].as_object() {
                        for (pattern_name, count) in patterns {
                            if let Some(count_val) = count.as_u64() {
                                let offset_coords =
                                    (base_coords.0 + 0.2, base_coords.1 + 0.2, base_coords.2 + 0.2);

                                let monster_correlation =
                                    calculate_ngram_monster_correlation(pattern_name);

                                ngrams.push(NgramCoordinate {
                                    ngram: pattern_name.clone(),
                                    ngram_coordinates: offset_coords,
                                    monster_correlation,
                                    frequency: count_val as u32,
                                    source: "code".to_string(),
                                });

                                ngram_map
                                    .entry(pattern_name.clone())
                                    .or_insert_with(Vec::new)
                                    .push(offset_coords);
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
}

fn find_syn_data_for_symbol(syn_data: &Value, symbol_name: &str) -> Option<SynAstData> {
    // Extract meaningful parts from mangled symbol name
    let clean_name = extract_clean_name(symbol_name);

    // Look for matching SYN data
    if let Some(syn_obj) = syn_data.as_object() {
        for (key, value) in syn_obj {
            if key.contains(&clean_name) || clean_name.contains(key) {
                let ast_type = determine_ast_type(value);
                let syn_ngrams = extract_syn_ngrams(value);
                let ast_monster_correlation = calculate_ast_monster_correlation(value);

                return Some(SynAstData {
                    ast_type,
                    ast_structure: value.clone(),
                    syn_ngrams,
                    ast_monster_correlation,
                });
            }
        }
    }

    None
}

fn extract_clean_name(symbol_name: &str) -> String {
    // Extract readable parts from mangled names
    let parts: Vec<&str> = symbol_name.split(&['_', 'N', 'E', 'C', 'h']).collect();
    for part in parts {
        if part.len() >= 4 && part.chars().all(|c| c.is_ascii_alphabetic()) {
            return part.to_lowercase();
        }
    }
    "unknown".to_string()
}

fn determine_ast_type(value: &Value) -> String {
    if value.is_object() {
        "Object".to_string()
    } else if value.is_array() {
        "Array".to_string()
    } else if value.is_string() {
        "String".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn extract_syn_ngrams(value: &Value) -> Vec<String> {
    let mut ngrams = Vec::new();

    if let Some(s) = value.as_str() {
        for n in 2..=4 {
            if s.len() >= n {
                for start in 0..=s.len() - n {
                    ngrams.push(s[start..start + n].to_string());
                }
            }
        }
    }

    ngrams.truncate(10); // Limit
    ngrams
}

fn calculate_ast_monster_correlation(value: &Value) -> f64 {
    let s = value.to_string();
    calculate_ngram_monster_correlation(&s)
}

fn add_syn_ngrams(
    ngrams: &mut Vec<NgramCoordinate>,
    ngram_map: &mut HashMap<String, Vec<(f64, f64, f64)>>,
    syn_data: &SynAstData,
    base_coords: &(f64, f64, f64),
) {
    for (i, ngram) in syn_data.syn_ngrams.iter().enumerate() {
        let offset_coords =
            (base_coords.0 + 0.3, base_coords.1 + 0.3, base_coords.2 + i as f64 * 0.05);

        let monster_correlation = calculate_ngram_monster_correlation(ngram);

        ngrams.push(NgramCoordinate {
            ngram: ngram.clone(),
            ngram_coordinates: offset_coords,
            monster_correlation,
            frequency: 1,
            source: "syn".to_string(),
        });

        ngram_map.entry(ngram.clone()).or_insert_with(Vec::new).push(offset_coords);
    }
}

fn calculate_ngram_monster_correlation(ngram: &str) -> f64 {
    let mut correlation = 0.0;
    for ch in ngram.chars() {
        let ascii_val = ch as u32 as u64;
        for &prime in &[2, 3, 5, 7, 11, 13, 31, 71] {
            if ascii_val % prime == 0 {
                correlation += 1.0 / prime as f64;
            }
        }
    }
    correlation / ngram.len() as f64
}

fn find_topology_connections(
    points: &[Value],
    current_idx: usize,
    coordinates: &(f64, f64, f64),
) -> Vec<String> {
    let mut connections = Vec::new();

    for (i, point) in points.iter().enumerate() {
        if i != current_idx {
            if let (Some(other_coords), Some(other_label)) =
                (point["coordinates"].as_array(), point["lattice_label"].as_str())
            {
                let other_coordinates = (
                    other_coords[0].as_f64().unwrap(),
                    other_coords[1].as_f64().unwrap(),
                    other_coords[2].as_f64().unwrap(),
                );

                let distance = calculate_distance(coordinates, &other_coordinates);
                if distance < 5.0 {
                    connections.push(other_label.to_string());
                }
            }
        }
    }

    connections.truncate(5); // Limit connections
    connections
}

fn calculate_distance(coord1: &(f64, f64, f64), coord2: &(f64, f64, f64)) -> f64 {
    let dx = coord1.0 - coord2.0;
    let dy = coord1.1 - coord2.1;
    let dz = coord1.2 - coord2.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn calculate_monster_density(nodes: &[MonsterTopologyNode]) -> f64 {
    let total_monster_correlation: f64 =
        nodes.iter().map(|n| n.ngrams.iter().map(|ng| ng.monster_correlation).sum::<f64>()).sum();

    total_monster_correlation / nodes.len() as f64
}

fn calculate_topology_completeness(
    nodes: &[MonsterTopologyNode],
    ngram_map: &HashMap<String, Vec<(f64, f64, f64)>>,
) -> f64 {
    let nodes_with_syn = nodes.iter().filter(|n| n.syn_ast_data.is_some()).count();
    let nodes_with_ngrams = nodes.iter().filter(|n| !n.ngrams.is_empty()).count();
    let nodes_with_connections =
        nodes.iter().filter(|n| !n.topology_connections.is_empty()).count();

    let completeness = (nodes_with_syn + nodes_with_ngrams + nodes_with_connections) as f64
        / (nodes.len() * 3) as f64;
    completeness
}

fn analyze_topology(topology: &MonsterTopology) {
    println!("\n📊 MONSTER TOPOLOGY ANALYSIS:");
    println!("=============================");

    let metrics = &topology.topology_metrics;

    println!("   🌐 Topology Metrics:");
    println!("     Total nodes: {}", metrics.total_nodes);
    println!("     Total n-grams: {}", metrics.total_ngrams);
    println!("     SYN coverage: {:.1}%", metrics.syn_coverage * 100.0);
    println!("     Monster density: {:.4}", metrics.monster_density);
    println!("     Topology completeness: {:.1}%", metrics.topology_completeness * 100.0);

    // N-gram source analysis
    let mut source_counts = HashMap::new();
    for node in &topology.nodes {
        for ngram in &node.ngrams {
            *source_counts.entry(ngram.source.clone()).or_insert(0) += 1;
        }
    }

    println!("\n   🔤 N-gram Sources:");
    for (source, count) in &source_counts {
        println!("     {}: {} n-grams", source, count);
    }

    // Most connected nodes
    println!("\n   🕸️ Most Connected Nodes:");
    let mut connected_nodes: Vec<_> = topology
        .nodes
        .iter()
        .map(|n| (n.lattice_label.clone(), n.topology_connections.len()))
        .collect();
    connected_nodes.sort_by(|a, b| b.1.cmp(&a.1));

    for (label, connection_count) in connected_nodes.iter().take(5) {
        println!("     {}: {} connections", label, connection_count);
    }

    // N-gram coordinate distribution
    println!("\n   📍 N-gram Coordinate Coverage:");
    println!("     Unique n-gram positions: {}", topology.ngram_coordinate_map.len());

    let total_positions: usize = topology.ngram_coordinate_map.values().map(|v| v.len()).sum();
    println!("     Total n-gram coordinates: {}", total_positions);
}

fn save_monster_topology(topology: &MonsterTopology) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete topology
    let json = serde_json::to_string_pretty(topology)?;
    fs::write("unified_monster_topology.json", json)?;

    // Save n-gram coordinate map separately
    let ngram_json = serde_json::to_string_pretty(&topology.ngram_coordinate_map)?;
    fs::write("ngram_coordinate_map.json", ngram_json)?;

    // Create visualization CSV
    let mut csv_content =
        String::from("symbol,x,y,z,ngram_count,syn_coverage,monster_density,connections\n");
    for node in &topology.nodes {
        let syn_coverage = if node.syn_ast_data.is_some() { 1.0 } else { 0.0 };
        let monster_density = node.ngrams.iter().map(|ng| ng.monster_correlation).sum::<f64>()
            / node.ngrams.len().max(1) as f64;

        csv_content.push_str(&format!(
            "{},{:.3},{:.3},{:.3},{},{:.1},{:.4},{}\n",
            node.symbol_name.replace(',', ";"),
            node.coordinates.0,
            node.coordinates.1,
            node.coordinates.2,
            node.ngrams.len(),
            syn_coverage,
            monster_density,
            node.topology_connections.len()
        ));
    }
    fs::write("monster_topology_visualization.csv", csv_content)?;

    println!("\n💾 UNIFIED MONSTER TOPOLOGY SAVED:");
    println!("==================================");
    println!("   Complete topology: unified_monster_topology.json");
    println!("   N-gram coordinates: ngram_coordinate_map.json");
    println!("   Visualization CSV: monster_topology_visualization.csv");
    println!(
        "   Topology completeness: {:.1}%",
        topology.topology_metrics.topology_completeness * 100.0
    );

    Ok(())
}
