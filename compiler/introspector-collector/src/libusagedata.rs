use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct UsageData {
    pub symbol: String,
    pub kind: String,
    pub usage_count: u32,
    pub usage_type: String,
    pub node_type: String,
}

#[derive(Debug)]
pub struct CrateUsageData {
    pub crate_name: String,
    pub module: String,
    pub usages: Vec<UsageData>,
}

pub fn load_crate_usage(crate_name: &str, base_path: &str) -> Result<Vec<CrateUsageData>, Box<dyn std::error::Error>> {
    let crate_dir = format!("{}/{}-*_usage_data", base_path, crate_name);
    let mut all_usage_data = Vec::new();
    
    // Find matching crate directory
    for entry in glob::glob(&crate_dir)? {
        let dir_path = entry?;
        
        // Load all JSON files in the directory
        for json_entry in glob::glob(&format!("{}/*.json", dir_path.display()))? {
            let json_path = json_entry?;
            let content = fs::read_to_string(&json_path)?;
            let json: Value = serde_json::from_str(&content)?;
            
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
    
    Ok(all_usage_data)
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
