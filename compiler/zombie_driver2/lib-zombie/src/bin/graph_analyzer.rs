use nalgebra::{DMatrix, DVector};
use petgraph::Graph;
use petgraph::graph::{NodeIndex, UnGraph};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EssentialArrow {
    pair: (char, char),
    strength: f64,
    must_preserve: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analysis_file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/debug/merged_rustc_analysis.json".to_string());

    println!("🔗 Character Graph Analysis - Eigenmatrix Calculator");
    println!("====================================================");
    println!("📁 Loading: {}", analysis_file);

    // Load merged analysis
    let content = fs::read_to_string(&analysis_file)?;
    let analysis: serde_json::Value = serde_json::from_str(&content)?;

    // Extract essential arrows
    let mut arrows = Vec::new();
    if let Some(arrow_array) = analysis["combined_essential_arrows"].as_array() {
        for arrow in arrow_array {
            if let (Some(pair), Some(strength), Some(must_preserve)) = (
                arrow["pair"].as_array(),
                arrow["strength"].as_f64(),
                arrow["must_preserve"].as_bool(),
            ) {
                if pair.len() == 2 {
                    if let (Some(from_str), Some(to_str)) = (pair[0].as_str(), pair[1].as_str()) {
                        if let (Some(from_ch), Some(to_ch)) =
                            (from_str.chars().next(), to_str.chars().next())
                        {
                            arrows.push(EssentialArrow {
                                pair: (from_ch, to_ch),
                                strength,
                                must_preserve,
                            });
                        }
                    }
                }
            }
        }
    }

    println!("🎯 Loaded {} essential arrows", arrows.len());

    // Build character graph
    let (graph, char_map, adjacency_matrix) = build_character_graph(&arrows)?;

    // Generate DOT graph
    generate_dot_graph(&graph, &char_map, &arrows)?;

    // Calculate eigenmatrix
    calculate_eigenmatrix(&adjacency_matrix, &char_map)?;

    Ok(())
}

fn build_character_graph(
    arrows: &[EssentialArrow],
) -> Result<(UnGraph<char, f64>, HashMap<char, NodeIndex>, DMatrix<f64>), Box<dyn std::error::Error>>
{
    let mut graph = Graph::new_undirected();
    let mut char_map = HashMap::new();

    // Add all unique characters as nodes
    let mut chars = std::collections::HashSet::new();
    for arrow in arrows {
        chars.insert(arrow.pair.0);
        chars.insert(arrow.pair.1);
    }

    for &ch in &chars {
        let node_idx = graph.add_node(ch);
        char_map.insert(ch, node_idx);
    }

    // Add edges with weights
    for arrow in arrows {
        if let (Some(&from_idx), Some(&to_idx)) =
            (char_map.get(&arrow.pair.0), char_map.get(&arrow.pair.1))
        {
            graph.add_edge(from_idx, to_idx, arrow.strength);
        }
    }

    // Build adjacency matrix
    let n = chars.len();
    let mut adjacency_matrix = DMatrix::zeros(n, n);

    let char_to_index: HashMap<char, usize> =
        chars.iter().enumerate().map(|(i, &ch)| (ch, i)).collect();

    for arrow in arrows {
        if let (Some(&i), Some(&j)) =
            (char_to_index.get(&arrow.pair.0), char_to_index.get(&arrow.pair.1))
        {
            adjacency_matrix[(i, j)] = arrow.strength;
            adjacency_matrix[(j, i)] = arrow.strength; // Symmetric for undirected
        }
    }

    println!("📊 Built graph: {} nodes, {} edges", graph.node_count(), graph.edge_count());

    Ok((graph, char_map, adjacency_matrix))
}

fn generate_dot_graph(
    graph: &UnGraph<char, f64>,
    char_map: &HashMap<char, NodeIndex>,
    arrows: &[EssentialArrow],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut dot = String::new();
    dot.push_str("graph RustcCharacters {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=circle, style=filled];\n");

    // Add nodes with character labels
    for (&ch, &node_idx) in char_map {
        let label = match ch {
            ' ' => "SPC".to_string(),
            '\n' => "NL".to_string(),
            '\t' => "TAB".to_string(),
            c => c.to_string(),
        };

        // Color nodes by frequency/importance
        let color = if arrows.iter().any(|a| a.pair.0 == ch || a.pair.1 == ch) {
            "lightblue"
        } else {
            "lightgray"
        };

        dot.push_str(&format!(
            "  {} [label=\"{}\", fillcolor={}];\n",
            node_idx.index(),
            label,
            color
        ));
    }

    // Add edges with weights
    for arrow in arrows.iter().take(20) {
        // Top 20 arrows only
        if let (Some(&from_idx), Some(&to_idx)) =
            (char_map.get(&arrow.pair.0), char_map.get(&arrow.pair.1))
        {
            let thickness = (arrow.strength * 3.0).max(0.5).min(5.0);
            dot.push_str(&format!(
                "  {} -- {} [label=\"{:.2}\", penwidth={}];\n",
                from_idx.index(),
                to_idx.index(),
                arrow.strength,
                thickness
            ));
        }
    }

    dot.push_str("}\n");

    fs::write("rustc_character_graph.dot", &dot)?;
    println!("📈 DOT graph saved to: rustc_character_graph.dot");

    Ok(())
}

fn calculate_eigenmatrix(
    matrix: &DMatrix<f64>,
    char_map: &HashMap<char, NodeIndex>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 Calculating eigenmatrix...");

    // Calculate eigenvalues and eigenvectors
    let eigen = matrix.clone().symmetric_eigen();
    let eigenvalues = eigen.eigenvalues;
    let eigenvectors = eigen.eigenvectors;

    println!("📊 Eigenanalysis Results:");
    println!("   Matrix size: {}x{}", matrix.nrows(), matrix.ncols());
    println!("   Eigenvalues: {}", eigenvalues.len());

    // Show top eigenvalues
    let mut eigenvalue_pairs: Vec<(usize, f64)> =
        eigenvalues.iter().enumerate().map(|(i, &val)| (i, val)).collect();
    eigenvalue_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n🔝 Top 5 Eigenvalues:");
    for (i, (idx, val)) in eigenvalue_pairs.iter().take(5).enumerate() {
        println!("   {}. λ_{} = {:.6}", i + 1, idx, val);
    }

    // Dominant eigenvector analysis
    if let Some((dominant_idx, dominant_val)) = eigenvalue_pairs.first() {
        println!("\n🎯 Dominant Eigenvector Analysis:");
        println!("   Eigenvalue: {:.6}", dominant_val);

        let dominant_eigenvector = eigenvectors.column(*dominant_idx);

        // Map eigenvector components back to characters
        let char_to_index: HashMap<char, usize> =
            char_map.iter().enumerate().map(|(i, (&ch, _))| (ch, i)).collect();

        let mut char_components: Vec<(char, f64)> =
            char_to_index.iter().map(|(&ch, &idx)| (ch, dominant_eigenvector[idx])).collect();
        char_components.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());

        println!("   Top character components:");
        for (i, (ch, component)) in char_components.iter().take(10).enumerate() {
            let label = match *ch {
                ' ' => "SPC".to_string(),
                '\n' => "NL".to_string(),
                c => c.to_string(),
            };
            println!("     {}. '{}' → {:.6}", i + 1, label, component);
        }
    }

    // Save eigenmatrix results
    let results = serde_json::json!({
        "matrix_size": [matrix.nrows(), matrix.ncols()],
        "eigenvalues": eigenvalues.as_slice(),
        "dominant_eigenvalue": eigenvalue_pairs.first().map(|(_, val)| val),
        "spectral_radius": eigenvalues.iter().map(|v| v.abs()).fold(0.0, f64::max)
    });

    fs::write("rustc_eigenmatrix.json", serde_json::to_string_pretty(&results)?)?;
    println!("📁 Eigenmatrix results saved to: rustc_eigenmatrix.json");

    Ok(())
}
