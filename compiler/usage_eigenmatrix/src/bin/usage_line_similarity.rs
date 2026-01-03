use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔍 Finding similar usage lines for our collector...");
    
    // Load our collector's usage lines
    let our_usages = load_our_usage_lines()?;
    println!("📊 Our collector has {} usage lines", our_usages.len());
    
    // Load all other usage lines
    let all_usages = load_all_usage_lines()?;
    println!("📈 Found {} other usage lines to compare", all_usages.len());
    
    // For each of our usage lines, find 3 most similar
    for (i, our_usage) in our_usages.iter().enumerate() {
        println!("\n🎯 Usage #{}: {}", i+1, truncate(&our_usage.usage, 80));
        
        let similarities = find_similar_usages(our_usage, &all_usages);
        
        for (j, (similar_usage, similarity)) in similarities.iter().take(3).enumerate() {
            println!("  {}. {:.1}% - {} (from {})", 
                     j+1, similarity * 100.0, 
                     truncate(&similar_usage.usage, 60),
                     &similar_usage.program);
        }
        
        // Suggest next line based on similar patterns
        suggest_next_line(&similarities);
    }
    
    Ok(())
}

#[derive(Clone)]
struct UsageLine {
    usage: String,
    usage_type: String,
    user_def_id: String,
    used_def_id: String,
    program: String,
}

fn load_our_usage_lines() -> Result<Vec<UsageLine>> {
    let mut usages = Vec::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if filename.starts_with("working_usage_collector") && filename.ends_with(".json") {
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usage_array) = data["usages"].as_array() {
                for usage_obj in usage_array {
                    usages.push(UsageLine {
                        usage: usage_obj["usage"].as_str().unwrap_or("").to_string(),
                        usage_type: usage_obj["usage_type"].as_str().unwrap_or("").to_string(),
                        user_def_id: usage_obj["user_def_id"].as_str().unwrap_or("").to_string(),
                        used_def_id: usage_obj["used_def_id"].as_str().unwrap_or("").to_string(),
                        program: "our_collector".to_string(),
                    });
                }
            }
        }
    }
    
    Ok(usages)
}

fn load_all_usage_lines() -> Result<Vec<UsageLine>> {
    let mut all_usages = Vec::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if filename.ends_with(".json") && !filename.starts_with("working_usage_collector") {
            let program_name = filename.split('_').next().unwrap_or("unknown").to_string();
            let content = fs::read_to_string(&path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            if let Some(usage_array) = data["usages"].as_array() {
                for usage_obj in usage_array.iter().take(5) { // Limit to avoid too much output
                    all_usages.push(UsageLine {
                        usage: usage_obj["usage"].as_str().unwrap_or("").to_string(),
                        usage_type: usage_obj["usage_type"].as_str().unwrap_or("").to_string(),
                        user_def_id: usage_obj["user_def_id"].as_str().unwrap_or("").to_string(),
                        used_def_id: usage_obj["used_def_id"].as_str().unwrap_or("").to_string(),
                        program: program_name.clone(),
                    });
                }
            }
        }
    }
    
    Ok(all_usages)
}

fn find_similar_usages(our_usage: &UsageLine, all_usages: &[UsageLine]) -> Vec<(UsageLine, f64)> {
    let mut similarities = Vec::new();
    
    for other_usage in all_usages {
        // Filter for compiler-relevant programs only
        if !is_compiler_relevant(&other_usage.program) {
            continue;
        }
        
        // Filter for compiler-relevant usage patterns
        if !is_compiler_usage(&other_usage.usage, &other_usage.used_def_id) {
            continue;
        }
        
        let similarity = calculate_usage_similarity(our_usage, other_usage);
        if similarity > 0.3 { // Higher threshold for quality
            similarities.push((other_usage.clone(), similarity));
        }
    }
    
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    similarities
}

fn is_compiler_relevant(program: &str) -> bool {
    matches!(program, "rustc" | "clippy" | "miri" | "rustdoc" | "rustfmt")
}

fn is_compiler_usage(usage: &str, def_id: &str) -> bool {
    // Look for compiler-specific patterns
    usage.contains("hir") || usage.contains("tcx") || usage.contains("DefId") ||
    def_id.contains("rustc_") || def_id.contains("hir") || def_id.contains("middle") ||
    usage.contains("typeck") || usage.contains("visit") || usage.contains("def_path")
}

fn calculate_usage_similarity(a: &UsageLine, b: &UsageLine) -> f64 {
    let mut score = 0.0;
    
    // Same DefId = high similarity
    if a.used_def_id == b.used_def_id {
        score += 0.8;
    }
    
    // Same usage type = medium similarity  
    if a.usage_type == b.usage_type {
        score += 0.3;
    }
    
    // Similar usage text = low similarity
    let text_sim = text_similarity(&a.usage, &b.usage);
    score += text_sim * 0.2;
    
    score.min(1.0)
}

fn text_similarity(a: &str, b: &str) -> f64 {
    let words_a: std::collections::HashSet<_> = a.split_whitespace().collect();
    let words_b: std::collections::HashSet<_> = b.split_whitespace().collect();
    
    let intersection = words_a.intersection(&words_b).count();
    let union = words_a.union(&words_b).count();
    
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

fn suggest_next_line(similarities: &[(UsageLine, f64)]) {
    if let Some((best_match, _)) = similarities.first() {
        println!("  💡 Suggested: Add similar pattern from {} program", best_match.program);
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len-3])
    } else {
        s.to_string()
    }
}
