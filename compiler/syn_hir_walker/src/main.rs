use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SynHirNode {
    name: String,
    usage_count: u64,
    hir_symbols: Vec<String>,
    syn_patterns: Vec<String>,
}

#[derive(Debug)]
pub struct SynHirWalker {
    nodes: Vec<SynHirNode>,
    mycelial_path: String,
}

impl SynHirWalker {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            mycelial_path: "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data".to_string(),
        }
    }
    
    pub fn discover_syn_data(&mut self) {
        println!("🔍 Discovering syn-related HIR data...");
        
        // Load syn-heavy files
        let syn_files = vec![
            ("rustc_trait_selection_traits_project.json", "lifetime_syntax"),
            ("rustc_lint_literals.json", "literals"), 
            ("rustc_trait_selection_traits_project.json", "async_destructor"),
            ("rustc_trait_selection_traits_project.json", "trait_project"),
            ("rustc_mir_transform_coroutine_drop.json", "coroutine_drop"),
        ];
        
        for (file, pattern) in syn_files {
            if let Some(node) = self.load_syn_hir_node(file, pattern) {
                self.nodes.push(node);
            }
        }
        
        // Sort by usage count
        self.nodes.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        
        println!("✅ Discovered {} syn HIR nodes", self.nodes.len());
    }
    
    fn load_syn_hir_node(&self, filename: &str, pattern: &str) -> Option<SynHirNode> {
        let file_path = format!("{}/{}", self.mycelial_path, filename);
        
        if let Ok(content) = std::fs::read_to_string(&file_path) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(usages) = json["usages"].as_array() {
                    let mut hir_symbols = Vec::new();
                    let mut syn_patterns = Vec::new();
                    let mut total_usage = 0u64;
                    
                    for usage in usages.iter().take(50) {
                        if let Some(symbol) = usage["symbol"].as_str() {
                            if let Some(count) = usage["usage_count"].as_u64() {
                                total_usage += count;
                                
                                // Extract syn-related patterns
                                if symbol.contains("syn") || symbol.contains("Syn") {
                                    syn_patterns.push(symbol.to_string());
                                }
                                
                                hir_symbols.push(format!("{} ({})", symbol, count));
                            }
                        }
                    }
                    
                    println!("📁 Loaded {}: {} usages, {} syn patterns", 
                            filename, usages.len(), syn_patterns.len());
                    
                    return Some(SynHirNode {
                        name: pattern.to_string(),
                        usage_count: total_usage,
                        hir_symbols,
                        syn_patterns,
                    });
                }
            }
        }
        
        None
    }
    
    pub fn walk_syn_hir(&self) {
        println!("🚶 Walking syn HIR data with detailed analysis...");
        
        let total_usage: u64 = self.nodes.iter().map(|n| n.usage_count).sum();
        let cutoff = (total_usage as f64 * 0.9) as u64;
        
        println!("📊 Total syn usage: {}, 90% cutoff: {}", total_usage, cutoff);
        
        let mut accumulated = 0u64;
        
        for (i, node) in self.nodes.iter().enumerate() {
            accumulated += node.usage_count;
            
            println!("\n🎯 Node {}: {} (usage: {}, accumulated: {})", 
                    i + 1, node.name, node.usage_count, accumulated);
            
            // Show top HIR symbols
            println!("  📋 Top HIR symbols:");
            for symbol in node.hir_symbols.iter().take(5) {
                println!("    • {}", symbol);
            }
            
            // Show syn patterns
            if !node.syn_patterns.is_empty() {
                println!("  🔍 Syn patterns found:");
                for pattern in node.syn_patterns.iter().take(3) {
                    println!("    • {}", pattern);
                }
            }
            
            if accumulated >= cutoff {
                println!("🎯 Cutoff reached at {}% of total usage", 
                        (accumulated as f64 / total_usage as f64 * 100.0) as u32);
                break;
            }
        }
    }
    
    pub fn generate_syn_walker_code(&self) {
        println!("\n🔧 Generating HIR-based syn walker code...");
        
        let walker_code = format!(r#"
// Auto-generated HIR-based syn walker
use std::collections::HashMap;

pub struct HirSynWalker {{
    syn_patterns: HashMap<String, Vec<String>>,
    hir_mappings: HashMap<String, u64>,
}}

impl HirSynWalker {{
    pub fn new() -> Self {{
        let mut patterns = HashMap::new();
        let mut mappings = HashMap::new();
        
{}
        
        Self {{
            syn_patterns: patterns,
            hir_mappings: mappings,
        }}
    }}
    
    pub fn walk_syn_hir(&self, target: &str) -> Vec<String> {{
        if let Some(patterns) = self.syn_patterns.get(target) {{
            patterns.clone()
        }} else {{
            vec![]
        }}
    }}
}}
"#, self.generate_pattern_code());
        
        std::fs::write("generated_syn_hir_walker.rs", walker_code)
            .expect("Failed to write syn walker");
        
        println!("✅ Generated HIR-based syn walker: generated_syn_hir_walker.rs");
    }
    
    fn generate_pattern_code(&self) -> String {
        let mut code = String::new();
        
        for node in &self.nodes {
            code.push_str(&format!("        mappings.insert(\"{}\".to_string(), {});\n", 
                                  node.name, node.usage_count));
            
            if !node.syn_patterns.is_empty() {
                code.push_str(&format!("        patterns.insert(\"{}\".to_string(), vec![\n", node.name));
                for pattern in node.syn_patterns.iter().take(5) {
                    code.push_str(&format!("            \"{}\".to_string(),\n", 
                                          pattern.replace("\"", "\\\"")));
                }
                code.push_str("        ]);\n");
            }
        }
        
        code
    }
}

fn main() {
    let mut walker = SynHirWalker::new();
    walker.discover_syn_data();
    walker.walk_syn_hir();
    walker.generate_syn_walker_code();
}
