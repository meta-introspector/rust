use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug, Clone)]
struct GlobalUsageStats {
    item_name: String,
    total_usage: u32,
    crate_count: u32,
    usage_density: f64,
}

fn main() {
    println!("🌍 Computing global usage statistics from 17,728 files...");
    
    let mut global_usage: HashMap<String, u32> = HashMap::new();
    let mut crate_usage: HashMap<String, HashMap<String, u32>> = HashMap::new();
    let mut processed_files = 0;
    
    let usage_dir = "../../usage_data";
    if let Ok(entries) = fs::read_dir(usage_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    process_usage_file(&entry.path().display().to_string(), 
                                     &mut global_usage, &mut crate_usage);
                    processed_files += 1;
                    
                    if processed_files % 1000 == 0 {
                        println!("  Processed {} files...", processed_files);
                    }
                }
            }
        }
    }
    
    println!("✅ Processed {} files total", processed_files);
    
    // Calculate usage statistics
    let mut stats: Vec<GlobalUsageStats> = global_usage.iter()
        .map(|(item, count)| {
            let crate_count = crate_usage.values()
                .filter(|crate_map| crate_map.contains_key(item))
                .count() as u32;
            
            let density = if crate_count > 0 {
                *count as f64 / crate_count as f64
            } else {
                0.0
            };
            
            GlobalUsageStats {
                item_name: item.clone(),
                total_usage: *count,
                crate_count,
                usage_density: density,
            }
        })
        .collect();
    
    // Sort by total usage
    stats.sort_by(|a, b| b.total_usage.cmp(&a.total_usage));
    
    println!("\n📊 TOP 20 MOST USED ITEMS GLOBALLY:");
    println!("===================================");
    
    for (i, stat) in stats.iter().take(20).enumerate() {
        println!("{}. {} (used {} times across {} crates, density: {:.1})", 
                i + 1, stat.item_name, stat.total_usage, 
                stat.crate_count, stat.usage_density);
    }
    
    // Focus on compiler-related items
    println!("\n🔧 TOP COMPILER/HIR ITEMS:");
    println!("==========================");
    
    let compiler_items: Vec<_> = stats.iter()
        .filter(|s| is_compiler_related(&s.item_name))
        .take(15)
        .collect();
    
    for (i, stat) in compiler_items.iter().enumerate() {
        println!("{}. {} (used {} times across {} crates)", 
                i + 1, stat.item_name, stat.total_usage, stat.crate_count);
    }
    
    // Save results for comparison
    save_global_stats(&stats);
    
    println!("\n✅ Global usage statistics computed!");
    println!("📁 Results saved to global_usage_stats.txt");
    println!("🎯 Ready for comparison with our usage patterns");
}

fn process_usage_file(filepath: &str, 
                     global_usage: &mut HashMap<String, u32>,
                     crate_usage: &mut HashMap<String, HashMap<String, u32>>) {
    if let Ok(content) = fs::read_to_string(filepath) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            let crate_name = json["crate"].as_str().unwrap_or("unknown").to_string();
            
            if let Some(usages) = json["usages"].as_array() {
                let mut crate_items = HashMap::new();
                
                for usage in usages {
                    if let Some(usage_str) = usage["usage"].as_str() {
                        let items = extract_items_from_usage(usage_str);
                        
                        for item in items {
                            *global_usage.entry(item.clone()).or_insert(0) += 1;
                            *crate_items.entry(item).or_insert(0) += 1;
                        }
                    }
                }
                
                crate_usage.insert(crate_name, crate_items);
            }
        }
    }
}

fn extract_items_from_usage(usage_str: &str) -> Vec<String> {
    let mut items = Vec::new();
    
    // Extract function/method names
    if let Some(start) = usage_str.find("::") {
        if let Some(end) = usage_str[start+2..].find(" ") {
            let item = &usage_str[start+2..start+2+end];
            if !item.is_empty() && item.chars().all(|c| c.is_alphanumeric() || c == '_') {
                items.push(item.to_string());
            }
        }
    }
    
    // Extract type names
    let type_patterns = ["TyCtxt", "DefId", "HirId", "ExprKind", "TyKind", "ItemKind"];
    for pattern in &type_patterns {
        if usage_str.contains(pattern) {
            items.push(pattern.to_string());
        }
    }
    
    // Extract common methods
    let method_patterns = ["def_path_str", "def_kind", "type_of", "hir", "generics_of", 
                          "predicates_of", "param_env", "associated_item"];
    for pattern in &method_patterns {
        if usage_str.contains(pattern) {
            items.push(pattern.to_string());
        }
    }
    
    items
}

fn is_compiler_related(item_name: &str) -> bool {
    let compiler_keywords = ["rustc", "hir", "ty", "def", "tcx", "Ty", "Hir", "Def"];
    compiler_keywords.iter().any(|keyword| item_name.contains(keyword))
}

fn save_global_stats(stats: &[GlobalUsageStats]) {
    let mut output = String::new();
    output.push_str("# Global Usage Statistics\n\n");
    output.push_str("## Top 50 Most Used Items\n");
    
    for (i, stat) in stats.iter().take(50).enumerate() {
        output.push_str(&format!("{}. {} - {} uses across {} crates (density: {:.2})\n", 
                                i + 1, stat.item_name, stat.total_usage, 
                                stat.crate_count, stat.usage_density));
    }
    
    fs::write("global_usage_stats.txt", output).unwrap();
}
