use std::collections::HashMap;
use serde_json::Value;

fn main() {
    println!("🔍 Tag Usage Analysis: Domain/Range/Codomain");
    
    let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
    
    let mut tag_as_domain = Vec::new();    // Tag uses these
    let mut tag_as_range = Vec::new();     // These use Tag
    let mut function_usage = HashMap::new(); // Function -> usage count
    
    // Scan all files for Tag relationships
    for entry in std::fs::read_dir(mycelial_path).unwrap() {
        let path = entry.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.contains("rustc") && name.ends_with(".json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        if let Some(usages) = data["usages"].as_array() {
                            for usage in usages {
                                let symbol = usage["symbol"].as_str().unwrap_or("unknown");
                                let user_crate = usage["user_crate"].as_str().unwrap_or("unknown");
                                let used_crate = usage["used_crate"].as_str();
                                let usage_count = usage["usage_count"].as_u64().unwrap_or(0);
                                
                                // Tag as domain: rustc_proc_macro::Tag uses something
                                if user_crate == "rustc_proc_macro" && symbol == "Tag" {
                                    if let Some(used) = used_crate {
                                        tag_as_domain.push((used.to_string(), usage_count));
                                        let func_key = format!("Tag → {}", used);
                                        *function_usage.entry(func_key).or_insert(0) += usage_count;
                                    }
                                }
                                
                                // Tag as range: something uses Tag
                                if symbol == "Tag" {
                                    tag_as_range.push((user_crate.to_string(), usage_count));
                                    let func_key = format!("{} → Tag", user_crate);
                                    *function_usage.entry(func_key).or_insert(0) += usage_count;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Rank functions by usage
    let mut ranked_functions: Vec<_> = function_usage.iter().collect();
    ranked_functions.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🏆 Ranked Functions with Tag:");
    for (i, (func, count)) in ranked_functions.iter().take(10).enumerate() {
        println!("  {}: {} (usage: {})", i+1, func, count);
    }
    
    // Deduplicate and analyze domains
    tag_as_domain.sort_by(|a, b| b.1.cmp(&a.1));
    tag_as_domain.dedup_by(|a, b| a.0 == b.0);
    
    println!("\n📥 Tag as Domain (Tag uses these):");
    for (i, (target, count)) in tag_as_domain.iter().take(5).enumerate() {
        println!("  D{}: {} (usage: {})", i+1, target, count);
    }
    
    // Deduplicate and analyze ranges
    tag_as_range.sort_by(|a, b| b.1.cmp(&a.1));
    tag_as_range.dedup_by(|a, b| a.0 == b.0);
    
    println!("\n📤 Tag as Range (These use Tag):");
    for (i, (source, count)) in tag_as_range.iter().take(5).enumerate() {
        println!("  R{}: {} (usage: {})", i+1, source, count);
    }
    
    // Analyze codomains (what Tag ultimately maps to)
    println!("\n🎯 Codomains (Ultimate targets):");
    let total_domain_usage: u64 = tag_as_domain.iter().map(|(_, count)| count).sum();
    let total_range_usage: u64 = tag_as_range.iter().map(|(_, count)| count).sum();
    
    println!("  Tag → * (outgoing): {} total usage", total_domain_usage);
    println!("  * → Tag (incoming): {} total usage", total_range_usage);
    
    if total_domain_usage > 0 && total_range_usage > 0 {
        println!("  Flow ratio: {:.2}", total_range_usage as f64 / total_domain_usage as f64);
    }
    
    // Check for bijective patterns
    println!("\n🔄 Bijection Analysis:");
    println!("  |Domain targets|: {}", tag_as_domain.len());
    println!("  |Range sources|: {}", tag_as_range.len());
    
    if tag_as_domain.len() == tag_as_range.len() && tag_as_domain.len() > 0 {
        println!("  ✅ Potential bijection structure detected!");
    } else {
        println!("  ⚠️  Asymmetric structure - not a simple bijection");
    }
}
