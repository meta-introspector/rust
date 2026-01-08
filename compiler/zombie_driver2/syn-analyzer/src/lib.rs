use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn_serde::json;
use serde_json::Value;
use syn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SynAnalysis {
    pub file_path: String,
    pub syn_node_counts: HashMap<String, u32>,
    pub total_nodes: u32,
    pub json_paths: Vec<String>,
    pub path_matrix: HashMap<String, u32>,
}

#[no_mangle]
pub extern "C" fn analyze_rust_file(file_path: *const u8, file_path_len: usize, content: *const u8, content_len: usize) -> *mut u8 {
    let file_path = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(file_path, file_path_len)) };
    let content = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(content, content_len)) };
    
    match analyze_file(file_path, content) {
        Ok(analysis) => {
            let json = serde_json::to_string(&analysis).unwrap_or_default();
            let bytes = json.into_bytes();
            let ptr = bytes.as_ptr() as *mut u8;
            std::mem::forget(bytes);
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

pub fn analyze_file(file_path: &str, content: &str) -> Result<SynAnalysis, Box<dyn std::error::Error>> {
    // Parse Rust code and convert to JSON using syn-serde
    let ast = syn::parse_file(content)?;
    let json_string = syn_serde::json::to_string(&ast);
    let json_value: Value = serde_json::from_str(&json_string)?;
    
    // Flatten JSON to paths (like gron)
    let mut paths = Vec::new();
    flatten_json_paths(&json_value, String::new(), &mut paths);
    
    // Create feature matrix from paths
    let mut node_counts = HashMap::new();
    let mut path_matrix = HashMap::new();
    
    for path in &paths {
        // Count path patterns
        *path_matrix.entry(path.clone()).or_insert(0) += 1;
        
        // Extract node types from paths
        let parts: Vec<&str> = path.split('.').collect();
        for part in parts {
            if part.starts_with("Item") || part.starts_with("Expr") || 
               part.starts_with("Stmt") || part.starts_with("Pat") || 
               part.starts_with("Type") || part.starts_with("Lit") ||
               part.starts_with("Vis") || part.starts_with("Field") ||
               part.starts_with("Arm") || part.starts_with("Block") {
                *node_counts.entry(part.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    let total_nodes = node_counts.values().sum();
    
    Ok(SynAnalysis {
        file_path: file_path.to_string(),
        syn_node_counts: node_counts,
        total_nodes,
        json_paths: paths,
        path_matrix,
    })
}

fn flatten_json_paths(value: &serde_json::Value, prefix: String, paths: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                
                // Add this path
                paths.push(new_prefix.clone());
                
                // Recurse into nested structures
                flatten_json_paths(val, new_prefix, paths);
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, i);
                paths.push(new_prefix.clone());
                flatten_json_paths(item, new_prefix, paths);
            }
        }
        serde_json::Value::String(s) => {
            paths.push(format!("{} = \"{}\"", prefix, s));
        }
        serde_json::Value::Number(n) => {
            paths.push(format!("{} = {}", prefix, n));
        }
        serde_json::Value::Bool(b) => {
            paths.push(format!("{} = {}", prefix, b));
        }
        serde_json::Value::Null => {
            paths.push(format!("{} = null", prefix));
        }
    }
}
