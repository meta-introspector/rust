use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("🧮 Building Usage Eigenmatrix from collected data...");
    
    let usage_dir = Path::new("../../usage_data");
    let mut usage_matrix: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut def_id_counts: HashMap<String, usize> = HashMap::new();
    let mut total_usages = 0;
    
    // Process all JSON files
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let user_id = usage_obj["user_def_id"].as_str().unwrap_or("unknown").to_string();
                    let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown").to_string();
                    let count = usage_obj["usage_count"].as_u64().unwrap_or(1) as usize;
                    
                    // Build usage matrix
                    *usage_matrix.entry(user_id.clone()).or_default().entry(used_id.clone()).or_insert(0) += count;
                    
                    // Count def_id frequencies
                    *def_id_counts.entry(user_id).or_insert(0) += count;
                    *def_id_counts.entry(used_id).or_insert(0) += count;
                    
                    total_usages += count;
                }
            }
        }
    }
    
    println!("📊 Usage Matrix Analysis:");
    println!("Total usage relationships: {}", total_usages);
    println!("Unique def_ids: {}", def_id_counts.len());
    println!("Usage matrix size: {}x{}", usage_matrix.len(), def_id_counts.len());
    
    // Find most used def_ids (eigenvalues)
    let mut sorted_def_ids: Vec<_> = def_id_counts.iter().collect();
    sorted_def_ids.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🔥 Top 20 Most Used DefIds (Core Eigenvalues):");
    for (i, (def_id, count)) in sorted_def_ids.iter().take(20).enumerate() {
        let percentage = (**count as f64 / total_usages as f64) * 100.0;
        println!("{}. {} - {} usages ({:.2}%)", i+1, def_id, count, percentage);
    }
    
    // Calculate sparsity
    let total_possible = usage_matrix.len() * def_id_counts.len();
    let actual_connections: usize = usage_matrix.values().map(|m| m.len()).sum();
    let sparsity = 1.0 - (actual_connections as f64 / total_possible as f64);
    
    println!("\n📈 Matrix Properties:");
    println!("Sparsity: {:.4}", sparsity);
    println!("Density: {:.4}", 1.0 - sparsity);
    
    // Core eigenmatrix (top 1% by usage)
    let threshold = total_usages / 100;
    let core_def_ids: Vec<_> = sorted_def_ids.iter()
        .filter(|(_, count)| **count >= threshold)
        .map(|(id, count)| (id.to_string(), **count))
        .collect();
    
    println!("\n🎯 Core Eigenmatrix ({} fundamental def_ids):", core_def_ids.len());
    
    // Save eigenmatrix
    let eigenmatrix_data = serde_json::json!({
        "total_usages": total_usages,
        "unique_def_ids": def_id_counts.len(),
        "matrix_size": usage_matrix.len(),
        "sparsity": sparsity,
        "core_def_ids": core_def_ids,
        "top_def_ids": sorted_def_ids.iter().take(50).map(|(id, count)| (id, count)).collect::<Vec<_>>()
    });
    
    fs::write("usage_eigenmatrix.json", serde_json::to_string_pretty(&eigenmatrix_data)?)?;
    println!("\n💾 Usage eigenmatrix saved to usage_eigenmatrix.json");
    
    Ok(())
}
