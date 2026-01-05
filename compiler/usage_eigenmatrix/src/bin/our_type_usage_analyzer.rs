use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🔍 Analyzing our own type usage patterns...");
    
    let mut type_usage = HashMap::new();
    
    // Scan our source files for type usage
    let source_files = [
        "./src/bin/syn_prime_analyzer.rs",
        "./src/bin/usage_convergence_analyzer.rs", 
        "./src/bin/syn_hir_bijection_prover.rs",
        "./src/bin/global_hir_usage_analyzer.rs",
        "../rustc_driver/enhanced_usage_collector.rs",
    ];
    
    for file in &source_files {
        if let Ok(content) = fs::read_to_string(file) {
            extract_type_usage(&content, &mut type_usage);
        }
    }
    
    // Sort by usage frequency
    let mut sorted_types: Vec<_> = type_usage.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n📊 Our Most Used Types:");
    println!("======================");
    
    for (i, (type_name, count)) in sorted_types.iter().take(15).enumerate() {
        println!("{}. {} (used {} times)", i + 1, type_name, count);
    }
    
    // Generate global search targets
    let top_types: Vec<String> = sorted_types.iter()
        .take(5)
        .map(|(name, _)| name.to_string())
        .collect();
    
    println!("\n🎯 Top 5 types to analyze globally:");
    for (i, type_name) in top_types.iter().enumerate() {
        println!("{}. {}", i + 1, type_name);
    }
    
    // Save results
    let mut output = String::new();
    output.push_str("# Our Type Usage Analysis\n\n");
    output.push_str("## Most Used Types\n");
    for (type_name, count) in &sorted_types {
        output.push_str(&format!("- {}: {} uses\n", type_name, count));
    }
    
    fs::write("our_type_usage_analysis.txt", output).unwrap();
    println!("\n✅ Analysis saved to our_type_usage_analysis.txt");
}

fn extract_type_usage(content: &str, usage_map: &mut HashMap<String, u32>) {
    let lines = content.lines();
    
    for line in lines {
        // Extract Rust types
        extract_rust_types(line, usage_map);
        
        // Extract standard library types
        extract_std_types(line, usage_map);
        
        // Extract external crate types
        extract_external_types(line, usage_map);
    }
}

fn extract_rust_types(line: &str, usage_map: &mut HashMap<String, u32>) {
    let rust_types = [
        "String", "Vec", "HashMap", "Option", "Result", "Box", "Rc", "Arc",
        "u32", "u64", "usize", "f64", "bool", "&str", "PathBuf", "File",
    ];
    
    for rust_type in &rust_types {
        if line.contains(rust_type) {
            *usage_map.entry(rust_type.to_string()).or_insert(0) += 1;
        }
    }
}

fn extract_std_types(line: &str, usage_map: &mut HashMap<String, u32>) {
    let patterns = [
        ("std::collections::", "std::collections"),
        ("std::fs::", "std::fs"),
        ("std::io::", "std::io"),
        ("std::path::", "std::path"),
    ];
    
    for (pattern, name) in &patterns {
        if line.contains(pattern) {
            *usage_map.entry(name.to_string()).or_insert(0) += 1;
        }
    }
}

fn extract_external_types(line: &str, usage_map: &mut HashMap<String, u32>) {
    let external_patterns = [
        ("syn::", "syn"),
        ("serde_json::", "serde_json"),
        ("rustc_hir::", "rustc_hir"),
        ("rustc_middle::", "rustc_middle"),
        ("TyCtxt", "TyCtxt"),
        ("DefId", "DefId"),
        ("HirId", "HirId"),
        ("ExprKind", "ExprKind"),
        ("TyKind", "TyKind"),
    ];
    
    for (pattern, name) in &external_patterns {
        if line.contains(pattern) {
            *usage_map.entry(name.to_string()).or_insert(0) += 1;
        }
    }
}
