use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🎯 Graph Interpreter with Clean Data\n");
    
    // Load clean graph data
    let clean_data = fs::read_to_string("clean_graph_data.json")?;
    let data: Value = serde_json::from_str(&clean_data)?;
    
    let call_graph = data["call_graph"].as_object().unwrap();
    let symbol_table = data["symbol_table"].as_object().unwrap();
    
    println!("📊 Loaded clean data:");
    println!("  Call graph: {} nodes", call_graph.len());
    println!("  Symbol table: {} entries", symbol_table.len());
    
    // Find entry point (most connected node)
    let mut node_connections = HashMap::new();
    for (caller, callees) in call_graph {
        let count = callees.as_array().unwrap().len();
        node_connections.insert(caller.clone(), count);
    }
    
    let default_entry = "unknown".to_string();
    let entry_point = node_connections.iter()
        .max_by_key(|(_, &count)| count)
        .map(|(node, _)| node)
        .unwrap_or(&default_entry);
    
    println!("\n🚀 Entry point: {}", get_symbol_name(entry_point, symbol_table));
    println!("  Calls {} functions", node_connections.get(entry_point).unwrap_or(&0));
    
    // Simple execution trace
    println!("\n🔄 Execution trace:");
    let mut visited = std::collections::HashSet::new();
    execute_node(entry_point, call_graph, symbol_table, &mut visited, 0, 5);
    
    println!("\n✅ Clean graph interpreter working!");
    
    Ok(())
}

fn execute_node(
    node: &str, 
    call_graph: &serde_json::Map<String, Value>,
    symbol_table: &serde_json::Map<String, Value>,
    visited: &mut std::collections::HashSet<String>,
    depth: usize,
    max_depth: usize
) {
    if depth >= max_depth || visited.contains(node) {
        return;
    }
    
    visited.insert(node.to_string());
    let indent = "  ".repeat(depth);
    let symbol = get_symbol_name(node, symbol_table);
    
    println!("{}📍 Executing: {}", indent, symbol);
    
    if let Some(callees) = call_graph.get(node) {
        if let Some(callees_array) = callees.as_array() {
            for callee in callees_array.iter().take(2) {
                if let Some(callee_str) = callee.as_str() {
                    let callee_symbol = get_symbol_name(callee_str, symbol_table);
                    println!("{}  → calls: {}", indent, callee_symbol);
                    execute_node(callee_str, call_graph, symbol_table, visited, depth + 1, max_depth);
                }
            }
        }
    }
}

fn get_symbol_name(def_id: &str, symbol_table: &serde_json::Map<String, Value>) -> String {
    symbol_table.get(def_id)
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string()
}
