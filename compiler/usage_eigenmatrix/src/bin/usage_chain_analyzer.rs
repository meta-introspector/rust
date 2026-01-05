use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug, Clone)]
struct UsageChain {
    step1: String,
    step2: String,
    chain_count: u32,
    pattern: String,
}

fn main() {
    println!("🔗 Calculating 2-step usage chains from global patterns...");
    
    let mut usage_chains: HashMap<String, u32> = HashMap::new();
    let mut processed_files = 0;
    
    // Focus on top usage items
    let target_items = ["DefId", "hir", "TyCtxt", "predicates_of", "associated_item"];
    
    let usage_dir = "../../usage_data";
    if let Ok(entries) = fs::read_dir(usage_dir) {
        for entry in entries.flatten().take(1000) { // Sample for speed
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    extract_usage_chains(&entry.path().display().to_string(), 
                                       &mut usage_chains, &target_items);
                    processed_files += 1;
                    
                    if processed_files % 100 == 0 {
                        println!("  Processed {} files...", processed_files);
                    }
                }
            }
        }
    }
    
    // Convert to structured chains
    let mut chains: Vec<UsageChain> = usage_chains.iter()
        .filter_map(|(pattern, count)| {
            if let Some((step1, step2)) = pattern.split_once(" -> ") {
                Some(UsageChain {
                    step1: step1.to_string(),
                    step2: step2.to_string(),
                    chain_count: *count,
                    pattern: pattern.clone(),
                })
            } else {
                None
            }
        })
        .collect();
    
    chains.sort_by(|a, b| b.chain_count.cmp(&a.chain_count));
    
    println!("\n🔗 TOP 2-STEP USAGE CHAINS:");
    println!("===========================");
    
    for (i, chain) in chains.iter().take(15).enumerate() {
        println!("{}. {} → {} (used {} times)", 
                i + 1, chain.step1, chain.step2, chain.chain_count);
    }
    
    // Group by first step
    println!("\n📊 CHAINS BY STARTING POINT:");
    println!("============================");
    
    for target in &target_items {
        let target_chains: Vec<_> = chains.iter()
            .filter(|c| c.step1.contains(target))
            .take(3)
            .collect();
        
        if !target_chains.is_empty() {
            println!("\n🎯 {} leads to:", target);
            for (i, chain) in target_chains.iter().enumerate() {
                println!("  {}. {} ({} times)", i + 1, chain.step2, chain.chain_count);
            }
        }
    }
    
    // Generate implementation suggestions
    println!("\n💡 IMPLEMENTATION SUGGESTIONS:");
    println!("==============================");
    
    for (i, chain) in chains.iter().take(5).enumerate() {
        let suggestion = generate_chain_suggestion(&chain.step1, &chain.step2);
        println!("{}. {}", i + 1, suggestion);
    }
    
    save_chain_analysis(&chains);
}

fn extract_usage_chains(filepath: &str, 
                       chains: &mut HashMap<String, u32>,
                       targets: &[&str]) {
    if let Ok(content) = fs::read_to_string(filepath) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            if let Some(usages) = json["usages"].as_array() {
                let mut file_usage = Vec::new();
                
                // Extract all usage items from this file
                for usage in usages {
                    if let Some(usage_str) = usage["usage"].as_str() {
                        if let Some(item) = extract_primary_item(usage_str, targets) {
                            file_usage.push(item);
                        }
                    }
                }
                
                // Create chains from consecutive usage
                for window in file_usage.windows(2) {
                    if window[0] != window[1] { // Different items
                        let chain = format!("{} -> {}", window[0], window[1]);
                        *chains.entry(chain).or_insert(0) += 1;
                    }
                }
            }
        }
    }
}

fn extract_primary_item(usage_str: &str, targets: &[&str]) -> Option<String> {
    for target in targets {
        if usage_str.contains(target) {
            return Some(target.to_string());
        }
    }
    
    // Extract method names
    if usage_str.contains("::") {
        if let Some(start) = usage_str.rfind("::") {
            if let Some(end) = usage_str[start+2..].find(" ") {
                let method = &usage_str[start+2..start+2+end];
                if method.len() > 2 && method.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Some(method.to_string());
                }
            }
        }
    }
    
    None
}

fn generate_chain_suggestion(step1: &str, step2: &str) -> String {
    match (step1, step2) {
        ("DefId", "hir") => "let hir_node = tcx.hir().get(hir_id_from_def_id(def_id));".to_string(),
        ("hir", "TyCtxt") => "let ty = tcx.type_of(def_id_from_hir(hir_node));".to_string(),
        ("TyCtxt", "predicates_of") => "let predicates = tcx.predicates_of(def_id);".to_string(),
        ("DefId", "type_of") => "let ty = tcx.type_of(def_id);".to_string(),
        ("predicates_of", "param_env") => "let env = tcx.param_env(def_id);".to_string(),
        _ => format!("// Chain: {} -> {}", step1, step2),
    }
}

fn save_chain_analysis(chains: &[UsageChain]) {
    let mut output = String::new();
    output.push_str("# 2-Step Usage Chain Analysis\n\n");
    
    for (i, chain) in chains.iter().take(20).enumerate() {
        output.push_str(&format!("{}. {} → {} ({} times)\n", 
                               i + 1, chain.step1, chain.step2, chain.chain_count));
    }
    
    fs::write("usage_chains_analysis.txt", output).unwrap();
    println!("📁 Chain analysis saved to usage_chains_analysis.txt");
}
