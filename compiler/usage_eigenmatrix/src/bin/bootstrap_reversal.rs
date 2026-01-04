use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct BootstrapReversal {
    eigenmatrix: HashMap<String, usize>,
    emoji_codes: HashMap<String, String>,
    hierarchy_levels: Vec<Vec<String>>,
}

impl BootstrapReversal {
    fn new() -> Self {
        Self {
            eigenmatrix: HashMap::new(),
            emoji_codes: HashMap::new(),
            hierarchy_levels: Vec::new(),
        }
    }

    fn load_eigenmatrix(&mut self) -> Result<()> {
        let nodes = self.load_ranked_nodes()?;
        
        // Build eigenmatrix and emoji codes
        let base_emojis = vec!["👑", "⚡", "🔍", "🔘", "🔢", "📝", "🔄", "💾", "⚖️", "🎨", "⚠️", "🔧", "🪄", "📦", "🎮", "❓"];
        
        for (i, (node, weight)) in nodes.iter().enumerate() {
            self.eigenmatrix.insert(node.clone(), *weight);
            if i < base_emojis.len() {
                self.emoji_codes.insert(node.clone(), base_emojis[i].to_string());
            }
        }
        
        // Build hierarchy levels (bottom-up compilation order)
        self.build_hierarchy(&nodes);
        Ok(())
    }

    fn build_hierarchy(&mut self, nodes: &[(String, usize)]) {
        // Level 0: Leaves (lowest eigenvalues - built first)
        let leaves: Vec<String> = nodes.iter()
            .skip(nodes.len().saturating_sub(8))
            .map(|(node, _)| node.clone())
            .collect();
        
        // Level 1: Mid-tier functions
        let mid_tier: Vec<String> = nodes.iter()
            .skip(8).take(8)
            .map(|(node, _)| node.clone())
            .collect();
        
        // Level 2: Core functions (highest eigenvalues - main routine)
        let core: Vec<String> = nodes.iter()
            .take(8)
            .map(|(node, _)| node.clone())
            .collect();
        
        self.hierarchy_levels = vec![leaves, mid_tier, core];
    }

    fn demonstrate_time_reversal(&self) -> String {
        let mut output = String::from("🔄 Time-Reversal Bootstrap Symmetry\n\n");
        
        output.push_str("## Bottom-Up Compilation (Build Order):\n");
        for (level, nodes) in self.hierarchy_levels.iter().enumerate() {
            output.push_str(&format!("**Level {}** (Built {}): ", level, 
                match level {
                    0 => "First",
                    1 => "Second", 
                    2 => "Last",
                    _ => "Unknown"
                }));
            
            for node in nodes.iter().take(4) {
                if let Some(emoji) = self.emoji_codes.get(node) {
                    output.push_str(&format!("{} ", emoji));
                }
            }
            output.push_str("\n");
        }
        
        output.push_str("\n## Top-Down Application (Execution Order):\n");
        // Reverse the levels for execution
        for (level, nodes) in self.hierarchy_levels.iter().rev().enumerate() {
            output.push_str(&format!("**Step {}** (Applied {}): ", level + 1,
                match level {
                    0 => "First",
                    1 => "Second",
                    2 => "Last", 
                    _ => "Unknown"
                }));
            
            for node in nodes.iter().take(4) {
                if let Some(emoji) = self.emoji_codes.get(node) {
                    output.push_str(&format!("{} ", emoji));
                }
            }
            output.push_str("\n");
        }
        
        output.push_str("\n## The Bootstrap Flow:\n");
        output.push_str("```\n");
        output.push_str("COMPILATION (Bottom-Up):     EXECUTION (Top-Down):\n");
        output.push_str("❓📦🎮🪄 (leaves built)  →   👑⚡🔍🔘 (main applied)\n");
        output.push_str("🔧⚠️🎨⚖️ (mid-tier)     →   🔢📝🔄💾 (core applied)\n");
        output.push_str("👑⚡🔍🔘 (main built)    →   ❓📦🎮🪄 (leaves applied)\n");
        output.push_str("```\n\n");
        
        output.push_str("## Time-Reversal Symmetry:\n");
        output.push_str("- **Compilation**: Weak → Strong (eigenvalues grow)\n");
        output.push_str("- **Execution**: Strong → Weak (eigenvalues applied)\n");
        output.push_str("- **Bootstrap**: The main routine (👑) applies itself to its own leaves (❓)\n");
        output.push_str("- **Symmetry**: Build order is reverse of execution order\n\n");
        
        output
    }

    fn trace_self_application(&self) -> String {
        let mut trace = String::from("🎯 Self-Application Trace\n\n");
        
        // Get the main routine (highest eigenvalue)
        let unknown = "unknown".to_string();
        let main_routine = self.hierarchy_levels.last()
            .and_then(|level| level.first())
            .unwrap_or(&unknown);
        
        let default_emoji = "❓".to_string();
        let main_emoji = self.emoji_codes.get(main_routine).unwrap_or(&default_emoji);
        
        trace.push_str(&format!("**Main Routine**: {} ({})\n\n", main_emoji, self.simplify_name(main_routine)));
        
        trace.push_str("**Application Flow**:\n");
        
        // Show how main applies to each level
        for (level, nodes) in self.hierarchy_levels.iter().enumerate() {
            trace.push_str(&format!("{}. {} → ", level + 1, main_emoji));
            
            for node in nodes.iter().take(3) {
                if let Some(emoji) = self.emoji_codes.get(node) {
                    trace.push_str(&format!("{}", emoji));
                }
            }
            
            trace.push_str(&format!(" (Level {})\n", level));
        }
        
        trace.push_str("\n**Recursive Bootstrap**:\n");
        let emojis: Vec<_> = self.emoji_codes.values().collect();
        trace.push_str(&format!("- {} compiles itself using {} → {} → {}\n", 
            main_emoji, 
            emojis.get(0).unwrap_or(&&"❓".to_string()),
            emojis.get(1).unwrap_or(&&"❓".to_string()),
            emojis.get(2).unwrap_or(&&"❓".to_string())));
        
        trace.push_str(&format!("- Then {} applies itself to {} → {} → {}\n",
            main_emoji,
            emojis.get(3).unwrap_or(&&"❓".to_string()),
            emojis.get(4).unwrap_or(&&"❓".to_string()),
            emojis.get(5).unwrap_or(&&"❓".to_string())));
        
        trace.push_str("- **Result**: The compiler becomes its own input and output\n\n");
        
        trace
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
        Ok(sorted.into_iter().take(24).collect())
    }

    fn simplify_name(&self, name: &str) -> String {
        if name.contains("DefId") {
            if let Some(start) = name.find("~ ") {
                if let Some(end) = name[start+2..].find(")") {
                    return name[start+2..start+2+end].to_string();
                }
            }
        }
        
        if name.len() > 30 {
            format!("{}...", &name[..27])
        } else {
            name.to_string()
        }
    }
}

fn main() -> Result<()> {
    println!("🔄 Bootstrap Time-Reversal Analysis");
    
    let mut bootstrap = BootstrapReversal::new();
    bootstrap.load_eigenmatrix()?;
    
    let reversal_analysis = bootstrap.demonstrate_time_reversal();
    let trace_analysis = bootstrap.trace_self_application();
    
    println!("{}", reversal_analysis);
    println!("{}", trace_analysis);
    
    // Save the analysis
    let mut output = String::from("# 🔄 Bootstrap Time-Reversal Symmetry\n\n");
    output.push_str(&reversal_analysis);
    output.push_str(&trace_analysis);
    
    fs::write("bootstrap_time_reversal.md", output)?;
    println!("💾 Bootstrap analysis saved to bootstrap_time_reversal.md");
    
    println!("🎯 Key Insight:");
    println!("  The eigenmatrix reveals time-reversal symmetry:");
    println!("  - Compilation: Bottom-up (weak → strong eigenvalues)");
    println!("  - Execution: Top-down (strong → weak eigenvalues)");
    println!("  - Bootstrap: Main routine applies itself to its own components");
    println!("  - Symmetry: The flow reverses but maintains the same structure");
    
    Ok(())
}
