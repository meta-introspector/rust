use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UsageData {
    pub symbol: String,
    pub kind: String,
    pub usage_count: u32,
    pub usage_type: String,
    pub node_type: String,
    pub def_kind: Option<String>,
}

#[derive(Debug)]
pub struct CrateUsageData {
    pub crate_name: String,
    pub module: String,
    pub usages: Vec<UsageData>,
}

pub fn load_usage_data() -> Vec<CrateUsageData> {
    let mut all_usage_data = Vec::new();
    
    // Load from test_usage_data (Rust compiler data)
    let test_data_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    println!("Loading test_usage_data from: {}", test_data_path);
    
    // Load all JSON files directly from test_usage_data directory
    let pattern = format!("{}/*.json", test_data_path);
    if let Ok(entries) = glob::glob(&pattern) {
        for entry in entries {
            if let Ok(json_path) = entry {
                let filename = json_path.file_name().unwrap_or_default().to_string_lossy();
                // Skip manifest, complexity, and other non-usage files
                if filename.contains("manifest") || filename.contains("complexity") || 
                   filename.contains("structs") || filename.contains("enums_classified") {
                    continue;
                }
                
                if let Ok(content) = fs::read_to_string(&json_path) {
                    if let Ok(json) = serde_json::from_str::<Value>(&content) {
                        // Skip files that don't have the expected structure
                        if json.get("crate").is_none() || json.get("usages").is_none() {
                            continue;
                        }
                        
                        let crate_name = json["crate"].as_str().unwrap_or("unknown").to_string();
                        let module = json["module"].as_str().unwrap_or("unknown").to_string();
                        
                        let mut usages = Vec::new();
                        if let Some(usage_array) = json["usages"].as_array() {
                            for usage in usage_array {
                                usages.push(UsageData {
                                    symbol: usage["symbol"].as_str().unwrap_or("").to_string(),
                                    kind: usage["kind"].as_str().unwrap_or("").to_string(),
                                    usage_count: usage["usage_count"].as_u64().unwrap_or(0) as u32,
                                    usage_type: usage["usage_type"].as_str().unwrap_or("").to_string(),
                                    node_type: usage["node_type"].as_str().unwrap_or("").to_string(),
                                    def_kind: usage["def_kind"].as_str().map(|s| s.to_string()),
                                });
                            }
                        }
                        
                        all_usage_data.push(CrateUsageData {
                            crate_name,
                            module,
                            usages,
                        });
                    }
                }
            }
        }
    }
    
    // Load from crate_usage_data (external crates data)
    let crate_data_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/crate_usage_data";
    println!("Loading crate_usage_data from: {}", crate_data_path);
    
    // Load all JSON files from crate subdirectories
    let crate_pattern = format!("{}/*/*.json", crate_data_path);
    if let Ok(entries) = glob::glob(&crate_pattern) {
        for entry in entries {
            if let Ok(json_path) = entry {
                let filename = json_path.file_name().unwrap_or_default().to_string_lossy();
                // Skip manifest and complexity files
                if filename.contains("manifest") || filename.contains("complexity") {
                    continue;
                }
                
                if let Ok(content) = fs::read_to_string(&json_path) {
                    if let Ok(json) = serde_json::from_str::<Value>(&content) {
                        // Skip files that don't have the expected structure
                        if json.get("crate").is_none() || json.get("usages").is_none() {
                            continue;
                        }
                        
                        let crate_name = json["crate"].as_str().unwrap_or("unknown").to_string();
                        let module = json["module"].as_str().unwrap_or("unknown").to_string();
                        
                        let mut usages = Vec::new();
                        if let Some(usage_array) = json["usages"].as_array() {
                            for usage in usage_array {
                                usages.push(UsageData {
                                    symbol: usage["symbol"].as_str().unwrap_or("").to_string(),
                                    kind: usage["kind"].as_str().unwrap_or("").to_string(),
                                    usage_count: usage["usage_count"].as_u64().unwrap_or(0) as u32,
                                    usage_type: usage["usage_type"].as_str().unwrap_or("").to_string(),
                                    node_type: usage["node_type"].as_str().unwrap_or("").to_string(),
                                    def_kind: usage["def_kind"].as_str().map(|s| s.to_string()),
                                });
                            }
                        }
                        
                        all_usage_data.push(CrateUsageData {
                            crate_name,
                            module,
                            usages,
                        });
                    }
                }
            }
        }
    }
    
    println!("Loaded {} total crate modules", all_usage_data.len());
    all_usage_data
}

pub fn get_symbol_weights(usage_data: &[CrateUsageData]) -> HashMap<String, f64> {
    let mut weights = HashMap::new();
    
    for crate_data in usage_data {
        for usage in &crate_data.usages {
            *weights.entry(usage.symbol.clone()).or_insert(0.0) += usage.usage_count as f64;
        }
    }
    
    weights
}
