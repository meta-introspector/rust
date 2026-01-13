use std::collections::HashMap;
use std::fs;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ContinuousExpansion {
    sources: Vec<String>,
    last_run: String,
    total_concepts: u32,
    expansion_rate: f64,
}

fn main() {
    println!("🔄 CONTINUOUS MODEL EXPANSION");
    println!("=============================");

    let mut expansion = load_expansion_config();
    
    // Run model expander
    run_tool("model_expander");
    
    // Run all analysis tools to enhance the model
    let tools = vec![
        "basic_block_analyzer",
        "io_matrix_analyzer", 
        "typed_domain_analyzer",
        "function_lattice_analyzer",
        "domain_model_analyzer"
    ];

    for tool in tools {
        println!("🔧 Running: {}", tool);
        run_tool(tool);
    }

    // Load and analyze the expanded model
    if let Ok(model_data) = fs::read_to_string("expanding_model.json") {
        if let Ok(model) = serde_json::from_str::<serde_json::Value>(&model_data) {
            if let Some(concepts) = model.get("concepts").and_then(|c| c.as_object()) {
                let new_total = concepts.len() as u32;
                expansion.expansion_rate = if expansion.total_concepts > 0 {
                    (new_total as f64 / expansion.total_concepts as f64) - 1.0
                } else { 0.0 };
                expansion.total_concepts = new_total;
                expansion.last_run = chrono::Utc::now().to_rfc3339();
                
                println!("📊 Model expanded to {} concepts (+{:.1}%)", 
                         new_total, expansion.expansion_rate * 100.0);
            }
        }
    }

    save_expansion_config(&expansion);
    
    // Schedule next expansion
    println!("⏰ Next expansion scheduled for continuous operation");
    println!("💡 Run this tool periodically to keep expanding the model");
}

fn load_expansion_config() -> ContinuousExpansion {
    if let Ok(data) = fs::read_to_string("expansion_config.json") {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        ContinuousExpansion::default()
    }
}

fn save_expansion_config(config: &ContinuousExpansion) {
    let json = serde_json::to_string_pretty(config).unwrap();
    fs::write("expansion_config.json", json).unwrap();
}

fn run_tool(tool_name: &str) {
    let output = Command::new("./target/debug/".to_string() + tool_name)
        .output();
    
    match output {
        Ok(result) if result.status.success() => {
            println!("✅ {} completed", tool_name);
        }
        _ => {
            println!("⚠️  {} failed or not found", tool_name);
        }
    }
}

impl Default for ContinuousExpansion {
    fn default() -> Self {
        Self {
            sources: vec![
                "/mnt/data1/nix/vendor/rust".to_string(),
                "/mnt/data1/meta-introspector".to_string(),
                ".".to_string(),
            ],
            last_run: chrono::Utc::now().to_rfc3339(),
            total_concepts: 0,
            expansion_rate: 0.0,
        }
    }
}
