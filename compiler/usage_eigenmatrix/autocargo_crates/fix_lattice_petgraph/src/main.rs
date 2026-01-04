use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use petgraph::algo::toposort;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

type DependencyGraph = Graph<String, (), Directed>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analysis = fs::read_to_string("rustc_complete_analysis.txt")?;
    
    let mut graph = Graph::new();
    let mut node_map = HashMap::new();
    let mut current_stack = Vec::new();
    
    // Parse dependencies from indentation structure
    for line in analysis.lines() {
        if line.contains("📍") {
            let indent_level = line.chars().take_while(|&c| c == ' ').count() / 2;
            
            if let Some(func_part) = line.split("📍").nth(1) {
                if let Some(func_name) = func_part.split(":").next() {
                    let func_name = func_name.trim().to_string();
                    
                    // Add node if not exists
                    let current_idx = *node_map.entry(func_name.clone()).or_insert_with(|| {
                        graph.add_node(func_name.clone())
                    });
                    
                    // Adjust stack to current level
                    current_stack.truncate(indent_level);
                    
                    // Add edges: current function depends on functions in stack
                    for parent_func in &current_stack {
                        if let Some(&parent_idx) = node_map.get(parent_func) {
                            // parent -> current (parent calls current)
                            graph.add_edge(parent_idx, current_idx, ());
                        }
                    }
                    
                    // Push current function to stack
                    current_stack.push(func_name);
                }
            }
        }
    }
    
    // Topological sort with cycle detection
    let sorted = match toposort(&graph, None) {
        Ok(sorted) => sorted,
        Err(cycle) => {
            eprintln!("❌ CYCLE DETECTED in dependency graph!");
            let cycle_node = cycle.node_id();
            eprintln!("🔄 Cycle involves node: {:?}", cycle_node);
            eprintln!("🔄 Problematic function: {}", graph[cycle_node]);
            
            // Find and report the cycle
            eprintln!("\n🔍 Analyzing dependency structure...");
            let mut cycle_functions = Vec::new();
            
            // Simple cycle detection - find strongly connected components
            for node_idx in graph.node_indices() {
                let func_name = &graph[node_idx];
                let mut visited = std::collections::HashSet::new();
                if has_cycle_from_node(&graph, node_idx, &mut visited, &mut Vec::new()) {
                    cycle_functions.push(func_name.clone());
                }
            }
            
            eprintln!("🔄 Functions involved in cycles: {}", cycle_functions.len());
            for (i, func) in cycle_functions.iter().take(10).enumerate() {
                eprintln!("   {}. {}", i + 1, func);
            }
            if cycle_functions.len() > 10 {
                eprintln!("   ... and {} more", cycle_functions.len() - 10);
            }
            
            return Err("Dependency cycle detected - cannot create proper lattice".into());
        }
    };
    
    // Group by levels (reverse topological order for dependency depth)
    let sorted_functions: Vec<String> = sorted.into_iter()
        .map(|idx| graph[idx].clone())
        .collect();
    
    // Calculate dependency levels
    let mut levels = Vec::new();
    let mut processed = std::collections::HashSet::new();
    
    // Start from functions with no dependencies (leaves)
    for func in &sorted_functions {
        if !processed.contains(func) {
            let level = calculate_level(func, &graph, &node_map, &mut processed);
            while levels.len() <= level {
                levels.push(Vec::new());
            }
            levels[level].push(func.clone());
        }
    }
    
    // Generate corrected lattice
    generate_corrected_lattice(&levels)?;
    
    Ok(())
}

fn has_cycle_from_node(
    graph: &DependencyGraph,
    start: NodeIndex,
    visited: &mut std::collections::HashSet<NodeIndex>,
    path: &mut Vec<NodeIndex>
) -> bool {
    if path.contains(&start) {
        return true; // Found cycle
    }
    
    if visited.contains(&start) {
        return false; // Already processed
    }
    
    visited.insert(start);
    path.push(start);
    
    for edge in graph.edges(start) {
        if has_cycle_from_node(graph, edge.target(), visited, path) {
            return true;
        }
    }
    
    path.pop();
    false
}

fn calculate_level(
    func: &str, 
    graph: &DependencyGraph, 
    node_map: &HashMap<String, NodeIndex>,
    processed: &mut std::collections::HashSet<String>
) -> usize {
    if processed.contains(func) {
        return 0;
    }
    
    processed.insert(func.to_string());
    
    if let Some(&node_idx) = node_map.get(func) {
        let mut max_dep_level = 0;
        
        // Check all dependencies (outgoing edges)
        for edge in graph.edges(node_idx) {
            let dep_func = &graph[edge.target()];
            if !processed.contains(dep_func) {
                let dep_level = calculate_level(dep_func, graph, node_map, processed);
                max_dep_level = max_dep_level.max(dep_level + 1);
            }
        }
        
        max_dep_level
    } else {
        0
    }
}

fn generate_corrected_lattice(levels: &[Vec<String>]) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = fs::File::create("rustc_fixed_lattice.txt")?;
    
    writeln!(output, "🔬 FIXED RUSTC DEPENDENCY LATTICE")?;
    writeln!(output, "=================================")?;
    writeln!(output, "Total Levels: {}", levels.len())?;
    writeln!(output, "Structure: Leaves (Level 0) → Main (Level {})", levels.len() - 1)?;
    writeln!(output)?;
    
    for (level_idx, level_functions) in levels.iter().enumerate() {
        if level_functions.is_empty() { continue; }
        
        let level_type = if level_idx == 0 {
            "🌱 LEAF LEVEL (No Dependencies)"
        } else if level_idx == levels.len() - 1 {
            "🚀 MAIN LEVEL (Entry Point)"
        } else {
            "📊 INTERMEDIATE LEVEL"
        };
        
        writeln!(output, "LEVEL {} - {} ({} functions)", level_idx, level_type, level_functions.len())?;
        writeln!(output, "├─ Dependency Depth: {}", level_idx)?;
        writeln!(output, "└─ Functions:")?;
        
        for (idx, func) in level_functions.iter().enumerate() {
            let prefix = if idx == level_functions.len() - 1 { "   └─" } else { "   ├─" };
            writeln!(output, "{} {}", prefix, func)?;
        }
        writeln!(output)?;
    }
    
    println!("✅ Generated rustc_fixed_lattice.txt");
    println!("🔬 Fixed lattice has {} levels", levels.len());
    
    // Show top level (should be rustc::main::main)
    if let Some(top_level) = levels.last() {
        println!("🚀 Top level (main): {} functions", top_level.len());
        for func in top_level {
            println!("   - {}", func);
        }
    }
    
    // Show bottom level (leaves)
    if let Some(bottom_level) = levels.first() {
        println!("🌱 Bottom level (leaves): {} functions", bottom_level.len());
        for func in bottom_level.iter().take(5) {
            println!("   - {}", func);
        }
        if bottom_level.len() > 5 {
            println!("   ... and {} more", bottom_level.len() - 5);
        }
    }
    
    Ok(())
}
