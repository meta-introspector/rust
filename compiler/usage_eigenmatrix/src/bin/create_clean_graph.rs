use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔧 Creating clean graph from existing usage data\n");
    
    let mut clean_graph = HashMap::new();
    let mut symbol_table = HashMap::new();
    let mut def_paths = HashMap::new();
    
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage in usages {
                    let caller = usage["user_def_id"].as_str().unwrap_or("unknown");
                    let callee = usage["used_def_id"].as_str().unwrap_or("unknown");
                    
                    // Only include DefId -> DefId relationships (clean function calls)
                    if caller.contains("DefId") && callee.contains("DefId") {
                        clean_graph.entry(caller.to_string())
                            .or_insert_with(Vec::new)
                            .push(callee.to_string());
                        
                        // Extract symbol names from DefId strings
                        extract_symbol_info(caller, &mut symbol_table, &mut def_paths);
                        extract_symbol_info(callee, &mut symbol_table, &mut def_paths);
                    }
                }
            }
        }
    }
    
    // Create clean data structure
    let clean_data = serde_json::json!({
        "metadata": {
            "description": "Clean call graph with DefId->DefId relationships only",
            "total_nodes": clean_graph.len(),
            "total_symbols": symbol_table.len(),
            "generated_from": "existing usage_data"
        },
        "call_graph": clean_graph,
        "symbol_table": symbol_table,
        "def_paths": def_paths
    });
    
    // Save clean data
    fs::write("clean_graph_data.json", serde_json::to_string_pretty(&clean_data)?)?;
    
    println!("✅ Clean graph data created:");
    println!("  Call graph nodes: {}", clean_graph.len());
    println!("  Symbol table entries: {}", symbol_table.len());
    println!("  Def paths: {}", def_paths.len());
    println!("  Saved to: clean_graph_data.json");
    
    // Show sample entries
    println!("\n📋 Sample call graph entries:");
    for (caller, callees) in clean_graph.iter().take(5) {
        println!("  {} calls {} functions", clean_name(caller), callees.len());
        for callee in callees.iter().take(2) {
            println!("    → {}", clean_name(callee));
        }
    }
    
    println!("\n📋 Sample symbol table entries:");
    for (def_id, symbol) in symbol_table.iter().take(5) {
        println!("  {} → {}", clean_name(def_id), symbol);
    }
    
    Ok(())
}

fn extract_symbol_info(def_id_str: &str, symbol_table: &mut HashMap<String, String>, def_paths: &mut HashMap<String, String>) {
    // Extract symbol name from DefId string like:
    // "DefId(95:809 ~ rustc_span[771e]::symbol::{impl#14}::intern)"
    if let Some(start) = def_id_str.find("~ ") {
        if let Some(end) = def_id_str[start+2..].find(")") {
            let full_path = &def_id_str[start+2..start+2+end];
            
            // Extract module path and symbol name
            if let Some(last_colon) = full_path.rfind("::") {
                let module_path = &full_path[..last_colon];
                let symbol_name = &full_path[last_colon+2..];
                
                symbol_table.insert(def_id_str.to_string(), symbol_name.to_string());
                def_paths.insert(def_id_str.to_string(), module_path.to_string());
            } else {
                symbol_table.insert(def_id_str.to_string(), full_path.to_string());
                def_paths.insert(def_id_str.to_string(), full_path.to_string());
            }
        }
    }
}

fn clean_name(def_id: &str) -> String {
    if let Some(start) = def_id.find("~ ") {
        if let Some(end) = def_id[start+2..].find(")") {
            return def_id[start+2..start+2+end].chars().take(30).collect();
        }
    }
    def_id.chars().take(20).collect()
}
