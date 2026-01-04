use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔍 Real Input/Processing/Output Classification\n");
    
    let mut node_usage = HashMap::new();
    let usage_dir = "../../usage_data";
    
    // Analyze actual usage patterns
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage in usages {
                    let used_node = usage["used_def_id"].as_str().unwrap_or("unknown");
                    let count = usage["usage_count"].as_u64().unwrap_or(1);
                    
                    *node_usage.entry(used_node.to_string()).or_insert(0) += count;
                }
            }
        }
    }
    
    // Classify by actual behavior
    let mut input_nodes = Vec::new();
    let mut processing_nodes = Vec::new();
    let mut output_nodes = Vec::new();
    
    for (node, usage_count) in &node_usage {
        let classification = classify_by_behavior(node);
        
        match classification {
            "Input" => input_nodes.push((node.clone(), *usage_count)),
            "Processing" => processing_nodes.push((node.clone(), *usage_count)),
            "Output" => output_nodes.push((node.clone(), *usage_count)),
            _ => {}
        }
    }
    
    // Sort by usage count
    input_nodes.sort_by(|a, b| b.1.cmp(&a.1));
    processing_nodes.sort_by(|a, b| b.1.cmp(&a.1));
    output_nodes.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("📥 INPUT LAYER (Disk/Strings/Raw Data):");
    for (node, count) in input_nodes.iter().take(10) {
        println!("  {} ({}x) - {}", get_emoji(node), count, clean_name(node));
    }
    
    println!("\n⚙️ PROCESSING LAYER (HIR/AST/Analysis):");
    for (node, count) in processing_nodes.iter().take(10) {
        println!("  {} ({}x) - {}", get_emoji(node), count, clean_name(node));
    }
    
    println!("\n📤 OUTPUT LAYER (CodeGen/Emit):");
    for (node, count) in output_nodes.iter().take(10) {
        println!("  {} ({}x) - {}", get_emoji(node), count, clean_name(node));
    }
    
    println!("\n📊 Layer Statistics:");
    println!("  Input nodes: {}", input_nodes.len());
    println!("  Processing nodes: {}", processing_nodes.len());
    println!("  Output nodes: {}", output_nodes.len());
    
    let input_usage: u64 = input_nodes.iter().map(|(_, count)| count).sum();
    let processing_usage: u64 = processing_nodes.iter().map(|(_, count)| count).sum();
    let output_usage: u64 = output_nodes.iter().map(|(_, count)| count).sum();
    
    println!("\n💥 Usage Distribution:");
    println!("  Input layer: {} total usages", input_usage);
    println!("  Processing layer: {} total usages", processing_usage);
    println!("  Output layer: {} total usages", output_usage);
    
    Ok(())
}

fn classify_by_behavior(node: &str) -> &str {
    // INPUT: Disk, strings, literals, raw data
    if node.contains("\"") || 
       node.chars().all(|c| c.is_ascii_digit()) ||
       node.contains("static") ||
       node.contains("read") ||
       node.contains("parse") ||
       node.contains("lex") {
        return "Input";
    }
    
    // OUTPUT: CodeGen, emit, write, link
    if node.contains("codegen") ||
       node.contains("emit") ||
       node.contains("write") ||
       node.contains("link") ||
       node.contains("output") {
        return "Output";
    }
    
    // PROCESSING: HIR, AST, analysis, checking
    if node.contains("hir") ||
       node.contains("ast") ||
       node.contains("check") ||
       node.contains("infer") ||
       node.contains("resolve") ||
       node.contains("analysis") ||
       node.contains("DefId") {
        return "Processing";
    }
    
    "Other"
}

fn get_emoji(node: &str) -> &str {
    if classify_by_behavior(node) == "Input" {
        if node.contains("static") { "👑" }
        else if node.contains("\"") { "📝" }
        else { "📥" }
    } else if classify_by_behavior(node) == "Processing" {
        if node.contains("hir") { "🌳" }
        else if node.contains("check") { "🔍" }
        else if node.contains("infer") { "🧠" }
        else { "⚙️" }
    } else {
        if node.contains("emit") { "📤" }
        else { "🔧" }
    }
}

fn clean_name(name: &str) -> String {
    if name.contains("DefId") {
        if let Some(start) = name.find("~ ") {
            if let Some(end) = name[start+2..].find(")") {
                return name[start+2..start+2+end].chars().take(30).collect();
            }
        }
    }
    name.chars().take(25).collect()
}
