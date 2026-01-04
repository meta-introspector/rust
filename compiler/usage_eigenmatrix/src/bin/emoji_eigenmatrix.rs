use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🎭 Rustc Emoji Eigenmatrix Generator");
    
    let nodes = load_and_rank_nodes()?;
    let emoji_matrix = assign_emojis(&nodes);
    
    println!("\n🏆 Top 30 Rustc Emoji Eigenmatrix:");
    for (i, (node, weight, emoji)) in emoji_matrix.iter().take(30).enumerate() {
        println!("{}. {} {} - {} uses", i+1, emoji, simplify_name(node), weight);
    }
    
    generate_emoji_categories(&emoji_matrix);
    save_emoji_matrix(&emoji_matrix)?;
    
    Ok(())
}

fn load_and_rank_nodes() -> Result<Vec<(String, usize)>> {
    let mut node_weights = HashMap::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                    let count = usage_obj["usage_count"].as_u64().unwrap_or(1) as usize;
                    
                    *node_weights.entry(used_id.to_string()).or_insert(0) += count;
                }
            }
        }
    }
    
    let mut sorted: Vec<_> = node_weights.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(sorted)
}

fn assign_emojis(nodes: &[(String, usize)]) -> Vec<(String, usize, String)> {
    nodes.iter().map(|(node, weight)| {
        let emoji = match classify_node(node, *weight) {
            NodeClass::Royalty => "👑",      // Top eigenvalues
            NodeClass::Core => "⚡",          // Core Rust functions
            NodeClass::Tracing => "🔍",      // Tracing/logging
            NodeClass::Boolean => "🔘",      // true/false
            NodeClass::Numbers => "🔢",      // Numeric literals
            NodeClass::Strings => "📝",      // String literals
            NodeClass::Iterator => "🔄",     // Iterator traits
            NodeClass::Memory => "💾",       // Memory operations
            NodeClass::Comparison => "⚖️",   // Comparison operations
            NodeClass::Format => "🎨",       // Formatting
            NodeClass::Error => "⚠️",        // Error handling
            NodeClass::Compiler => "🔧",     // Rustc internals
            NodeClass::Macro => "🪄",        // Macros
            NodeClass::Type => "📦",         // Type operations
            NodeClass::Control => "🎮",      // Control flow
            NodeClass::Unknown => "❓",      // Everything else
        };
        (node.clone(), *weight, emoji.to_string())
    }).collect()
}

#[derive(Debug)]
enum NodeClass {
    Royalty, Core, Tracing, Boolean, Numbers, Strings, Iterator, 
    Memory, Comparison, Format, Error, Compiler, Macro, Type, Control, Unknown
}

fn classify_node(node: &str, weight: usize) -> NodeClass {
    // Royalty: Top 1% by weight
    if weight > 4000 {
        return NodeClass::Royalty;
    }
    
    // Core boolean constants
    if node == "true" || node == "false" {
        return NodeClass::Boolean;
    }
    
    // Tracing system
    if node.contains("META") || node.contains("CALLSITE") || node.contains("tracing") {
        return NodeClass::Tracing;
    }
    
    // Core Rust functions
    if node.contains("core[") && (node.contains("::fmt::") || node.contains("::cmp::") || node.contains("::iter::")) {
        return NodeClass::Core;
    }
    
    // Numbers
    if node.chars().all(|c| c.is_ascii_digit() || c == '.') && !node.is_empty() {
        return NodeClass::Numbers;
    }
    
    // Strings
    if node.starts_with("\"") && node.ends_with("\"") {
        return NodeClass::Strings;
    }
    
    // Iterator operations
    if node.contains("::iter") || node.contains("Iterator::") || node.contains("IntoIterator") {
        return NodeClass::Iterator;
    }
    
    // Memory operations
    if node.contains("::alloc") || node.contains("::mem::") || node.contains("::ptr::") {
        return NodeClass::Memory;
    }
    
    // Comparison operations
    if node.contains("::cmp::") || node.contains("PartialEq") || node.contains("PartialOrd") {
        return NodeClass::Comparison;
    }
    
    // Formatting
    if node.contains("::fmt::") || node.contains("Display") || node.contains("Debug") {
        return NodeClass::Format;
    }
    
    // Error handling
    if node.contains("Result::") || node.contains("Option::") || node.contains("::unwrap") {
        return NodeClass::Error;
    }
    
    // Compiler internals
    if node.contains("rustc_") {
        return NodeClass::Compiler;
    }
    
    // Macros
    if node.contains("macro") || node.contains("!") {
        return NodeClass::Macro;
    }
    
    // Type operations
    if node.contains("::ty::") || node.contains("Type") {
        return NodeClass::Type;
    }
    
    // Control flow
    if node.contains("ControlFlow") || node.contains("::ops::") {
        return NodeClass::Control;
    }
    
    NodeClass::Unknown
}

fn generate_emoji_categories(emoji_matrix: &[(String, usize, String)]) {
    let mut categories = HashMap::new();
    
    for (_, weight, emoji) in emoji_matrix {
        *categories.entry(emoji.clone()).or_insert(0) += weight;
    }
    
    let mut sorted_categories: Vec<_> = categories.into_iter().collect();
    sorted_categories.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n🎭 Emoji Categories by Total Weight:");
    for (emoji, total_weight) in sorted_categories.iter().take(16) {
        let category_name = match emoji.as_str() {
            "👑" => "Royalty (Top Eigenvalues)",
            "⚡" => "Core Rust Functions", 
            "🔍" => "Tracing & Observability",
            "🔘" => "Boolean Constants",
            "🔢" => "Numeric Literals",
            "📝" => "String Literals",
            "🔄" => "Iterator Operations",
            "💾" => "Memory Management",
            "⚖️" => "Comparison Operations",
            "🎨" => "Formatting & Display",
            "⚠️" => "Error Handling",
            "🔧" => "Compiler Internals",
            "🪄" => "Macros & Meta",
            "📦" => "Type Operations",
            "🎮" => "Control Flow",
            _ => "Unknown"
        };
        println!("  {} {} - {} total uses", emoji, category_name, total_weight);
    }
}

fn save_emoji_matrix(emoji_matrix: &[(String, usize, String)]) -> Result<()> {
    let mut output = String::from("# 🎭 Rustc Emoji Eigenmatrix\n\n");
    output.push_str("## Top 50 Nodes by Weight\n\n");
    
    for (i, (node, weight, emoji)) in emoji_matrix.iter().take(50).enumerate() {
        output.push_str(&format!("{}. {} **{}** - {} uses\n", 
            i+1, emoji, simplify_name(node), weight));
    }
    
    output.push_str("\n## Emoji Legend\n\n");
    output.push_str("- 👑 Royalty (Top Eigenvalues)\n");
    output.push_str("- ⚡ Core Rust Functions\n");
    output.push_str("- 🔍 Tracing & Observability\n");
    output.push_str("- 🔘 Boolean Constants\n");
    output.push_str("- 🔢 Numeric Literals\n");
    output.push_str("- 📝 String Literals\n");
    output.push_str("- 🔄 Iterator Operations\n");
    output.push_str("- 💾 Memory Management\n");
    output.push_str("- ⚖️ Comparison Operations\n");
    output.push_str("- 🎨 Formatting & Display\n");
    output.push_str("- ⚠️ Error Handling\n");
    output.push_str("- 🔧 Compiler Internals\n");
    output.push_str("- 🪄 Macros & Meta\n");
    output.push_str("- 📦 Type Operations\n");
    output.push_str("- 🎮 Control Flow\n");
    output.push_str("- ❓ Unknown\n");
    
    fs::write("rustc_emoji_eigenmatrix.md", output)?;
    println!("\n💾 Emoji eigenmatrix saved to rustc_emoji_eigenmatrix.md");
    
    Ok(())
}

fn simplify_name(name: &str) -> String {
    if name.contains("DefId") {
        if let Some(start) = name.find("~ ") {
            if let Some(end) = name[start+2..].find(")") {
                return name[start+2..start+2+end].to_string();
            }
        }
    }
    
    if name.len() > 40 {
        format!("{}...", &name[..37])
    } else {
        name.to_string()
    }
}
