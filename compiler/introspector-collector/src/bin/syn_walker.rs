use std::collections::HashMap;
use std::env;
use std::fs;
use syn::{File, Item};
use quote::ToTokens;
use rand::{thread_rng, Rng};
use introspector_collector::libusagedata::{load_crate_usage, get_symbol_weights};

fn get_mycelial_data_path() -> Result<String, Box<dyn std::error::Error>> {
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let parsed: toml::Value = toml::from_str(&cargo_toml)?;
    
    if let Some(metadata) = parsed.get("package").and_then(|p| p.get("metadata")) {
        if let Some(path) = metadata.get("mycelial_data_path").and_then(|p| p.as_str()) {
            return Ok(format!("{}/crate_usage_data", path));
        }
    }
    
    Err("mycelial_data_path not found in Cargo.toml metadata".into())
}

struct SynWalker {
    symbol_weights: HashMap<String, f64>,
    rng: rand::rngs::ThreadRng,
    visited_items: Vec<String>,
}

impl SynWalker {
    fn new(symbol_weights: &HashMap<String, f64>) -> Self {
        Self {
            symbol_weights: symbol_weights.clone(),
            rng: thread_rng(),
            visited_items: Vec::new(),
        }
    }
    
    fn choose_next_item(&mut self, available_items: &[String]) -> Option<String> {
        let total_weight: f64 = available_items.iter()
            .filter_map(|item| self.symbol_weights.get(item))
            .sum();
        
        if total_weight == 0.0 {
            return available_items.first().cloned();
        }
        
        let mut random_point = self.rng.gen_range(0.0..total_weight);
        
        for item in available_items {
            if let Some(&weight) = self.symbol_weights.get(item) {
                if random_point <= weight {
                    return Some(item.clone());
                }
                random_point -= weight;
            }
        }
        
        available_items.first().cloned()
    }
    
    fn walk_items(&mut self, items: &[Item]) {
        let mut available_items: Vec<String> = items.iter()
            .filter_map(|item| match item {
                Item::Fn(f) => Some(f.sig.ident.to_string()),
                Item::Struct(s) => Some(s.ident.to_string()),
                Item::Enum(e) => Some(e.ident.to_string()),
                Item::Impl(i) => i.self_ty.as_ref().to_token_stream().to_string().split("::").last().map(|s| s.to_string()),
                _ => None,
            })
            .collect();
        
        println!("🔍 Syn Walker: Usage-Driven AST Exploration");
        println!("📊 Found {} items to explore", available_items.len());
        
        while !available_items.is_empty() {
            if let Some(chosen_item) = self.choose_next_item(&available_items) {
                println!("🎯 Exploring: {}", chosen_item);
                
                if let Some(&weight) = self.symbol_weights.get(&chosen_item) {
                    println!("   📈 Usage weight: {:.2}", weight);
                }
                
                self.visited_items.push(chosen_item.clone());
                available_items.retain(|item| item != &chosen_item);
            }
        }
        
        println!("\n✅ Exploration complete. Visited {} items", self.visited_items.len());
        
        // Self-improvement suggestions
        self.suggest_improvements();
    }
    
    fn suggest_improvements(&self) {
        println!("\n🔧 Self-Improvement Suggestions:");
        
        if self.visited_items.len() < 5 {
            println!("   • Add more functions to increase exploration surface");
        }
        
        if !self.visited_items.iter().any(|item| item.contains("error")) {
            println!("   • Add error handling functions for robustness");
        }
        
        if !self.visited_items.iter().any(|item| item.contains("test")) {
            println!("   • Add test functions for better coverage");
        }
        
        println!("   • Consider adding weighted exploration based on usage frequency");
        println!("   • Implement recursive AST traversal for deeper analysis");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <rust_file.rs>", args[0]);
        std::process::exit(1);
    }
    
    let rust_file = &args[1];
    
    // Get data path from Cargo.toml metadata
    let data_path = match get_mycelial_data_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("❌ Failed to read mycelial data path from Cargo.toml: {}", e);
            std::process::exit(1);
        }
    };
    
    // Load syn usage data from mycelial network
    let usage_data = match load_crate_usage("syn", &data_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("❌ Failed to load syn usage data: {}", e);
            std::process::exit(1);
        }
    };
    
    if usage_data.is_empty() {
        eprintln!("❌ No syn usage data found");
        std::process::exit(1);
    }
    
    // Get symbol weights
    let symbol_weights = get_symbol_weights(&usage_data);
    
    // Parse Rust file
    let content = match std::fs::read_to_string(rust_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read {}: {}", rust_file, e);
            std::process::exit(1);
        }
    };
    
    let syntax_tree = match syn::parse_file(&content) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("❌ Failed to parse {}: {}", rust_file, e);
            std::process::exit(1);
        }
    };
    
    // Create walker and explore
    let mut walker = SynWalker::new(&symbol_weights);
    walker.walk_items(&syntax_tree.items);
}
