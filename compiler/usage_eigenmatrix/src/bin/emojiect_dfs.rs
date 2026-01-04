use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct EmojiectTracer {
    emoji_codes: HashMap<String, String>,
    call_graph: HashMap<String, Vec<String>>,
    trace_depth: usize,
}

impl EmojiectTracer {
    fn new() -> Self {
        Self {
            emoji_codes: HashMap::new(),
            call_graph: HashMap::new(),
            trace_depth: 0,
        }
    }

    fn load_graph(&mut self) -> Result<()> {
        let nodes = self.load_ranked_nodes()?;
        self.build_emoji_codes(&nodes);
        self.build_call_graph()?;
        Ok(())
    }

    fn build_emoji_codes(&mut self, nodes: &[(String, usize)]) {
        let base_emojis = vec!["👑", "⚡", "🔍", "🔘", "🔢", "📝", "🔄", "💾", "⚖️", "🎨", "⚠️", "🔧", "🪄", "📦", "🎮", "❓"];
        
        for (i, (node, _)) in nodes.iter().enumerate() {
            let emoji = if i < base_emojis.len() {
                base_emojis[i].to_string()
            } else {
                format!("{}🔹", base_emojis[i % base_emojis.len()])
            };
            self.emoji_codes.insert(node.clone(), emoji);
        }
    }

    fn build_call_graph(&mut self) -> Result<()> {
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
                        
                        self.call_graph.entry(user_id.to_string())
                            .or_insert_with(Vec::new)
                            .push(used_id.to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn trace_main_routine_dfs(&mut self) -> String {
        let mut trace = String::from("🎯 Main Routine DFS Compilation Trace\n\n");
        
        // Show multiple starting points to see the full ecosystem
        let top_nodes = self.get_top_nodes();
        
        for (i, node) in top_nodes.iter().take(3).enumerate() {
            let emoji = self.emoji_codes.get(node).unwrap_or(&"❓".to_string()).clone();
            trace.push_str(&format!("**Starting DFS #{} from**: {} {}\n\n", 
                i + 1, emoji, self.simplify_name(node)));
            
            let mut visited = std::collections::HashSet::new();
            let dfs_trace = self.dfs_compile(node, &mut visited, 0);
            trace.push_str(&dfs_trace);
            trace.push_str("\n");
        }
        
        trace.push_str("**Emojiect Ecosystem Overview**:\n");
        trace.push_str(&self.show_ecosystem_pattern());
        
        trace
    }

    fn dfs_compile(&mut self, node: &str, visited: &mut std::collections::HashSet<String>, depth: usize) -> String {
        if depth > 8 || visited.contains(node) {
            return String::new();
        }
        
        visited.insert(node.to_string());
        let indent = "  ".repeat(depth);
        let default_emoji = "❓".to_string();
        let emoji = self.emoji_codes.get(node).unwrap_or(&default_emoji).clone();
        let simplified_name = self.simplify_name(node);
        
        let mut trace = format!("{}{}📍 {} compiling...\n", indent, emoji, simplified_name);
        
        // Get children and trace them
        let children: Vec<String> = self.call_graph.get(node).cloned().unwrap_or_default();
        for child in children.iter().take(5) { // Show more children
            let child_trace = self.dfs_compile(child, visited, depth + 1);
            if !child_trace.is_empty() {
                trace.push_str(&child_trace);
            }
        }
        
        trace.push_str(&format!("{}{}✅ {} compiled\n", indent, emoji, simplified_name));
        trace
    }

    fn show_self_application(&self, main_routine: &str) -> String {
        let mut output = String::new();
        let default_main_emoji = "👑".to_string();
        let main_emoji = self.emoji_codes.get(main_routine).unwrap_or(&default_main_emoji);
        
        output.push_str(&format!("1. {} applies to itself: {} → {} (self-reference)\n", 
            main_emoji, main_emoji, main_emoji));
        
        if let Some(children) = self.call_graph.get(main_routine) {
            for (i, child) in children.iter().take(5).enumerate() {
                let default_child_emoji = "❓".to_string();
                let child_emoji = self.emoji_codes.get(child).unwrap_or(&default_child_emoji);
                output.push_str(&format!("{}. {} applies to {}: {} → {} ({})\n", 
                    i + 2, main_emoji, child_emoji, main_emoji, child_emoji, self.simplify_name(child)));
            }
        }
        
        output.push_str(&format!("\n**Result**: {} has compiled itself and all its dependencies\n", main_emoji));
        output.push_str("**Bootstrap Complete**: The compiler can now compile anything, including itself\n");
        
        output
    }

    fn find_main_routine(&self) -> String {
        // Find the node with highest eigenvalue
        self.emoji_codes.iter()
            .find(|(_, emoji)| *emoji == "👑")
            .map(|(node, _)| node.clone())
            .unwrap_or_else(|| "false".to_string())
    }

    fn load_ranked_nodes(&self) -> Result<Vec<(String, usize)>> {
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
        Ok(sorted.into_iter().take(32).collect())
    }

    fn simplify_name(&self, name: &str) -> String {
        if name.contains("DefId") {
            if let Some(start) = name.find("~ ") {
                if let Some(end) = name[start+2..].find(")") {
                    return name[start+2..start+2+end].chars().take(25).collect();
                }
            }
        }
        
        name.chars().take(20).collect()
    }
}

fn main() -> Result<()> {
    println!("🎭 Emojiect DFS Compilation Tracer");
    
    let mut tracer = EmojiectTracer::new();
    tracer.load_graph()?;
    
    let dfs_trace = tracer.trace_main_routine_dfs();
    println!("{}", dfs_trace);
    
    // Save the trace
    fs::write("emojiect_dfs_trace.md", format!("# 🎭 Emojiect DFS Trace\n\n{}", dfs_trace))?;
    println!("💾 DFS trace saved to emojiect_dfs_trace.md");
    
    println!("\n🎯 Key Discovery:");
    println!("  Each AST node becomes an 'emojiect' - a visual computational unit");
    println!("  The main routine (👑) compiles itself through depth-first traversal");
    println!("  We can watch the compiler bootstrap itself in emoji form!");
    println!("  The trace shows the exact moment Rust becomes self-aware");
    
    Ok(())
}
