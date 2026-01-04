use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct PureEmojiectTracer {
    function_emojis: HashMap<String, String>,
    call_graph: HashMap<String, Vec<String>>,
}

impl PureEmojiectTracer {
    fn new() -> Self {
        Self {
            function_emojis: HashMap::new(),
            call_graph: HashMap::new(),
        }
    }

    fn load_and_trace(&mut self) -> Result<()> {
        self.load_usage_data()?;
        self.assign_emojis();
        self.trace_compilation();
        Ok(())
    }

    fn load_usage_data(&mut self) -> Result<()> {
        let usage_dir = "../../usage_data";
        
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage_obj in usages {
                        let caller = usage_obj["user_def_id"].as_str().unwrap_or("unknown").to_string();
                        let callee = usage_obj["used_def_id"].as_str().unwrap_or("unknown").to_string();
                        
                        self.call_graph.entry(caller).or_insert_with(Vec::new).push(callee);
                    }
                }
            }
        }
        Ok(())
    }

    fn assign_emojis(&mut self) {
        let emojis = vec![
            "👑", "⚡", "🔍", "🔘", "🔢", "📝", "🔄", "💾", "⚖️", "🎨", "⚠️", "🔧", "🪄", "📦", "🎮", "❓",
            "🌟", "🔥", "💎", "🎯", "🚀", "⭐", "🌈", "🎪", "🎭", "🎨", "🎵", "🎲", "🎸", "🎺", "🎻", "🎹"
        ];
        
        let mut all_functions: Vec<_> = self.call_graph.keys().collect();
        all_functions.sort();
        
        for (i, func) in all_functions.iter().enumerate() {
            let emoji = emojis.get(i % emojis.len()).unwrap_or(&"❓");
            self.function_emojis.insert((*func).clone(), emoji.to_string());
        }
    }

    fn trace_compilation(&self) {
        println!("🎭 Pure Emojiect DFS Trace - Real Function Calls\n");
        
        // Show the actual call relationships
        println!("📋 Function → Emoji Mapping:");
        for (func, emoji) in self.function_emojis.iter().take(20) {
            println!("  {} = {}", emoji, self.clean_name(func));
        }
        
        println!("\n🔄 Call Graph (Who Calls Who):");
        let default_emoji = "❓".to_string();
        for (caller, callees) in self.call_graph.iter().take(10) {
            let caller_emoji = self.function_emojis.get(caller).unwrap_or(&default_emoji);
            println!("  {} calls:", caller_emoji);
            
            for callee in callees.iter().take(5) {
                let callee_emoji = self.function_emojis.get(callee).unwrap_or(&default_emoji);
                println!("    {} → {}", caller_emoji, callee_emoji);
            }
        }
        
        println!("\n🎯 DFS Compilation Trace:");
        if let Some(start_func) = self.call_graph.keys().next() {
            let mut visited = std::collections::HashSet::new();
            self.dfs_trace(start_func, &mut visited, 0);
        }
        
        println!("\n🔄 Self-Consumption Pattern:");
        self.show_self_consumption();
    }

    fn dfs_trace(&self, func: &str, visited: &mut std::collections::HashSet<String>, depth: usize) {
        if depth > 5 || visited.contains(func) {
            return;
        }
        
        visited.insert(func.to_string());
        let indent = "  ".repeat(depth);
        let emoji = self.function_emojis.get(func).unwrap_or(&"❓".to_string());
        
        println!("{}{}📍 compiling {}", indent, emoji, self.clean_name(func));
        
        if let Some(callees) = self.call_graph.get(func) {
            for callee in callees.iter().take(3) {
                self.dfs_trace(callee, visited, depth + 1);
            }
        }
        
        println!("{}{}✅ compiled", indent, emoji);
    }

    fn show_self_consumption(&self) {
        println!("Looking for functions that call themselves or create cycles...");
        
        for (func, callees) in &self.call_graph {
            let func_emoji = self.function_emojis.get(func).unwrap_or(&"❓".to_string());
            
            // Direct self-reference
            if callees.contains(func) {
                println!("  {} consumes itself: {} → {}", func_emoji, func_emoji, func_emoji);
            }
            
            // Indirect cycles (A calls B, B calls A)
            for callee in callees {
                if let Some(callee_callees) = self.call_graph.get(callee) {
                    if callee_callees.contains(func) {
                        let callee_emoji = self.function_emojis.get(callee).unwrap_or(&"❓".to_string());
                        println!("  {} ↔ {} (mutual consumption)", func_emoji, callee_emoji);
                    }
                }
            }
        }
    }

    fn clean_name(&self, name: &str) -> String {
        if name.contains("DefId") {
            if let Some(start) = name.find("~ ") {
                if let Some(end) = name[start+2..].find(")") {
                    return name[start+2..start+2+end].chars().take(30).collect();
                }
            }
        }
        name.chars().take(25).collect()
    }
}

fn main() -> Result<()> {
    let mut tracer = PureEmojiectTracer::new();
    tracer.load_and_trace()?;
    Ok(())
}
