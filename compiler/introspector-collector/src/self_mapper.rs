use std::collections::HashMap;
use serde_json;
use crate::data_structures::*;

pub struct SelfMapper {
    collector_usage: HashMap<String, UsageEntry>,
    rust_usage: HashMap<String, UsageEntry>,
    mappings: HashMap<String, String>, // collector_element -> rust_element
}

impl SelfMapper {
    pub fn new() -> Self {
        Self {
            collector_usage: HashMap::new(),
            rust_usage: HashMap::new(),
            mappings: HashMap::new(),
        }
    }
    
    pub fn load_self_usage(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let module_data: ModuleData = serde_json::from_str(&data)?;
        
        for usage in module_data.usages {
            self.collector_usage.insert(usage.symbol.clone(), usage);
        }
        Ok(())
    }
    
    pub fn load_rust_usage(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let module_data: ModuleData = serde_json::from_str(&data)?;
        
        for usage in module_data.usages {
            self.rust_usage.insert(usage.symbol.clone(), usage);
        }
        Ok(())
    }
    
    pub fn map_to_rust_eigenspace(&mut self) {
        println!("=== MAPPING COLLECTOR → RUST EIGENSPACE ===");
        
        for (collector_symbol, collector_usage) in &self.collector_usage {
            // Find similar patterns in Rust usage
            let rust_match = self.find_rust_analog(collector_symbol, collector_usage);
            
            if let Some(rust_symbol) = rust_match {
                self.mappings.insert(collector_symbol.clone(), rust_symbol.clone());
                println!("MAPPED: {} → {}", collector_symbol, rust_symbol);
            } else {
                println!("NOVEL: {} (no Rust analog found)", collector_symbol);
            }
        }
    }
    
    fn find_rust_analog(&self, symbol: &str, usage: &UsageEntry) -> Option<String> {
        // Look for similar usage patterns in Rust
        for (rust_symbol, rust_usage) in &self.rust_usage {
            if self.usage_similarity(usage, rust_usage) > 0.8 {
                return Some(rust_symbol.clone());
            }
        }
        None
    }
    
    fn usage_similarity(&self, a: &UsageEntry, b: &UsageEntry) -> f64 {
        let mut score = 0.0;
        
        // Compare usage types
        if a.usage_type == b.usage_type { score += 0.3; }
        if a.kind == b.kind { score += 0.3; }
        if a.node_type == b.node_type { score += 0.2; }
        
        // Compare usage patterns
        if a.usage_count == b.usage_count { score += 0.2; }
        
        score
    }
    
    pub fn generate_eigendecomposition(&self) {
        println!("\n=== EIGENDECOMPOSITION ANALYSIS ===");
        
        let mapped_count = self.mappings.len();
        let novel_count = self.collector_usage.len() - mapped_count;
        
        println!("Mapped elements: {} (existing eigenspace)", mapped_count);
        println!("Novel elements: {} (eigenspace extension)", novel_count);
        
        let coverage = mapped_count as f64 / self.collector_usage.len() as f64;
        println!("Eigenspace coverage: {:.2}%", coverage * 100.0);
        
        if coverage < 0.5 {
            println!("→ Collector extends Rust eigenspace significantly");
        } else {
            println!("→ Collector mostly within existing eigenspace");
        }
    }
}
