use anyhow::Result;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<()> {
    println!("🎯 RUSTC MAIN() COMPLETE CALL GRAPH");
    println!("═══════════════════════════════════");
    
    let usage_data = load_usage_data()?;
    let main_graph = build_main_call_graph(&usage_data);
    
    generate_main_dot_graph(&main_graph)?;
    analyze_main_path(&main_graph);
    
    Ok(())
}

fn load_usage_data() -> Result<Vec<(String, String)>> {
    let mut edges = Vec::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let user_id = usage_obj["user_def_id"].as_str().unwrap_or("unknown");
                    let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                    
                    edges.push((user_id.to_string(), used_id.to_string()));
                }
            }
        }
    }
    
    Ok(edges)
}

fn build_main_call_graph(edges: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    
    // Find all main-related functions
    for (user, used) in edges {
        if is_main_related(user) || is_main_related(used) {
            graph.entry(user.clone()).or_insert_with(Vec::new).push(used.clone());
        }
    }
    
    println!("📊 Found {} main-related nodes", graph.len());
    graph
}

fn is_main_related(node: &str) -> bool {
    node.contains("main") || 
    node.contains("rustc_driver") ||
    node.contains("run_compiler") ||
    node.contains("parse_crate") ||
    node.contains("analysis") ||
    node.contains("codegen") ||
    node.contains("link")
}

fn generate_main_dot_graph(graph: &HashMap<String, Vec<String>>) -> Result<()> {
    let mut dot_content = String::from("digraph rustc_main_graph {\n");
    dot_content.push_str("  rankdir=TB;\n");
    dot_content.push_str("  node [shape=box, style=filled];\n");
    
    // Highlight main entry point
    dot_content.push_str("  \"main\" [fillcolor=red, fontcolor=white];\n");
    dot_content.push_str("  \"rustc_driver::main\" [fillcolor=orange];\n");
    
    // Add all edges
    for (user, used_list) in graph {
        let simplified_user = simplify_node_name(user);
        for used in used_list {
            let simplified_used = simplify_node_name(used);
            dot_content.push_str(&format!("  \"{}\" -> \"{}\";\n", simplified_user, simplified_used));
        }
    }
    
    dot_content.push_str("}\n");
    
    fs::write("rustc_main_graph.dot", dot_content)?;
    println!("📊 Generated rustc main() graph: rustc_main_graph.dot");
    println!("   Use: dot -Tpng rustc_main_graph.dot -o rustc_main.png");
    
    Ok(())
}

fn simplify_node_name(name: &str) -> String {
    if name.contains("DefId") {
        // Extract function name from DefId
        if let Some(start) = name.find("::") {
            if let Some(end) = name[start+2..].find(")") {
                return name[start+2..start+2+end].to_string();
            }
        }
        "core_fn".to_string()
    } else if name.len() > 30 {
        format!("{}...", &name[..27])
    } else {
        name.to_string()
    }
}

fn analyze_main_path(graph: &HashMap<String, Vec<String>>) {
    println!("\n🛤️ RUSTC MAIN() EXECUTION PATH:");
    
    // Find main entry points
    let main_nodes: Vec<_> = graph.keys()
        .filter(|k| k.contains("main") && !k.contains("DefId"))
        .collect();
    
    println!("📍 Main entry points found:");
    for main_node in &main_nodes {
        println!("  • {}", main_node);
        
        if let Some(callees) = graph.get(*main_node) {
            println!("    Calls {} functions:", callees.len());
            for callee in callees.iter().take(5) {
                println!("      → {}", simplify_node_name(callee));
            }
            if callees.len() > 5 {
                println!("      ... and {} more", callees.len() - 5);
            }
        }
        println!();
    }
}
