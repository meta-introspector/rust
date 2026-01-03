use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔍 Finding code similar to our usage collector...");
    
    // Load our collector's usage pattern
    let our_pattern = load_program_pattern("working_usage_collector")?;
    println!("📊 Our collector uses {} unique DefIds", our_pattern.len());
    
    // Load all other programs
    let all_patterns = load_all_patterns()?;
    println!("📈 Found {} other programs to compare", all_patterns.len());
    
    // Find most similar programs
    let similarities = find_similarities(&our_pattern, &all_patterns)?;
    
    println!("\n🎯 Top 10 most similar programs to our usage collector:");
    for (i, (program, similarity, shared_patterns)) in similarities.iter().take(10).enumerate() {
        println!("{}. {} - {:.2}% similarity ({} shared patterns)", 
                 i+1, program, similarity * 100.0, shared_patterns);
    }
    
    Ok(())
}

fn load_program_pattern(program_name: &str) -> Result<HashMap<String, usize>> {
    let mut pattern = HashMap::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if filename.starts_with(program_name) && filename.ends_with(".json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let used_def_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown").to_string();
                    *pattern.entry(used_def_id).or_insert(0) += 1;
                }
            }
        }
    }
    
    Ok(pattern)
}

fn load_all_patterns() -> Result<HashMap<String, HashMap<String, usize>>> {
    let mut all_patterns: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if filename.ends_with(".json") {
            let program_name = filename.split('_').next().unwrap_or("unknown").to_string();
            
            // Skip our own collector
            if program_name == "working" { continue; }
            
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usages) = data["usages"].as_array() {
                for usage_obj in usages {
                    let used_def_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown").to_string();
                    *all_patterns.entry(program_name.clone()).or_default().entry(used_def_id).or_insert(0) += 1;
                }
            }
        }
    }
    
    Ok(all_patterns)
}

fn find_similarities(our_pattern: &HashMap<String, usize>, all_patterns: &HashMap<String, HashMap<String, usize>>) -> Result<Vec<(String, f64, usize)>> {
    let mut similarities = Vec::new();
    
    for (program, pattern) in all_patterns {
        let similarity = cosine_similarity(our_pattern, pattern);
        let shared_count = count_shared_patterns(our_pattern, pattern);
        similarities.push((program.clone(), similarity, shared_count));
    }
    
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    Ok(similarities)
}

fn cosine_similarity(a: &HashMap<String, usize>, b: &HashMap<String, usize>) -> f64 {
    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    
    let all_keys: std::collections::HashSet<_> = a.keys().chain(b.keys()).collect();
    
    for key in all_keys {
        let val_a = *a.get(key).unwrap_or(&0) as f64;
        let val_b = *b.get(key).unwrap_or(&0) as f64;
        
        dot_product += val_a * val_b;
        norm_a += val_a * val_a;
        norm_b += val_b * val_b;
    }
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a.sqrt() * norm_b.sqrt())
    }
}

fn count_shared_patterns(a: &HashMap<String, usize>, b: &HashMap<String, usize>) -> usize {
    a.keys().filter(|k| b.contains_key(*k)).count()
}
