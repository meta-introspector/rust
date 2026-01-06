use std::collections::HashMap;

// Use the existing libusagedata structures
use introspector_collector::libusagedata::{load_usage_data, CrateUsageData};

fn main() {
    println!("Analyzing DefKinds from both datasets using libusagedata...\n");
    
    // Use the existing libusagedata loader (now loads both datasets)
    let usage_data = load_usage_data();
    
    let mut defkind_counts: HashMap<String, usize> = HashMap::new();
    let mut defkind_examples: HashMap<String, Vec<String>> = HashMap::new();
    let mut total_entries = 0;

    for crate_data in &usage_data {
        for usage in &crate_data.usages {
            total_entries += 1;
            
            // Count by kind (our classification)
            *defkind_counts.entry(usage.kind.clone()).or_insert(0) += usage.usage_count as usize;
            
            // Store examples for each kind
            let examples = defkind_examples.entry(usage.kind.clone()).or_insert_with(Vec::new);
            if examples.len() < 5 {
                examples.push(format!("{}::{} ({})", 
                    crate_data.crate_name,
                    usage.symbol,
                    usage.usage_type
                ));
            }
        }
    }

    println!("=== DEFKIND ANALYSIS RESULTS ===");
    println!("Total entries processed: {}\n", total_entries);

    // Sort by count descending
    let mut sorted_kinds: Vec<_> = defkind_counts.iter().collect();
    sorted_kinds.sort_by(|a, b| b.1.cmp(a.1));

    for (kind, count) in sorted_kinds {
        println!("{}: {} occurrences", kind, count);
        if let Some(examples) = defkind_examples.get(kind) {
            for example in examples.iter().take(3) {
                println!("  - {}", example);
            }
        }
        println!();
    }

    // Look for patterns in other_usage specifically
    println!("=== ANALYZING OTHER_USAGE PATTERNS ===");
    analyze_other_usage_patterns(&usage_data);
}

fn analyze_other_usage_patterns(usage_data: &[CrateUsageData]) {
    let mut other_usage_symbols: HashMap<String, usize> = HashMap::new();
    let mut other_usage_types: HashMap<String, usize> = HashMap::new();
    let mut other_usage_examples: Vec<String> = Vec::new();

    for crate_data in usage_data {
        for usage in &crate_data.usages {
            if usage.kind == "other_usage" {
                *other_usage_symbols.entry(usage.symbol.clone()).or_insert(0) += usage.usage_count as usize;
                *other_usage_types.entry(usage.usage_type.clone()).or_insert(0) += usage.usage_count as usize;
                
                if other_usage_examples.len() < 20 {
                    other_usage_examples.push(format!(
                        "Symbol: {}, Type: {}, Crate: {}", 
                        usage.symbol, 
                        usage.usage_type,
                        crate_data.crate_name
                    ));
                }
            }
        }
    }

    println!("Top other_usage symbols:");
    let mut sorted_symbols: Vec<_> = other_usage_symbols.iter().collect();
    sorted_symbols.sort_by(|a, b| b.1.cmp(a.1));
    for (symbol, count) in sorted_symbols.iter().take(10) {
        println!("  {}: {}", symbol, count);
    }

    println!("\nTop other_usage types:");
    let mut sorted_types: Vec<_> = other_usage_types.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));
    for (usage_type, count) in sorted_types.iter().take(10) {
        println!("  {}: {}", usage_type, count);
    }

    println!("\nExample other_usage entries:");
    for example in other_usage_examples.iter().take(10) {
        println!("  {}", example);
    }
}
