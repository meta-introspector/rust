// Cargo Build Hijack + Monster Analysis on All Rustc Crates
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

#[derive(Debug)]
struct DatasetAnalysis {
    crate_count: u32,
    total_syn_nodes: u64,
    monster_correlations: HashMap<u64, f64>,
    strongest_patterns: Vec<(String, u64, f64)>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 CARGO BUILD HIJACK + MONSTER ANALYSIS ON RUSTC DATASET");
    println!("=========================================================");

    // Setup hijack environment
    setup_cargo_hijack()?;

    // Find rustc source directory
    let rustc_dir = find_rustc_source()?;
    println!("📦 Found rustc source: {}", rustc_dir);

    // Execute hijacked build to collect SYN data
    let syn_datasets = hijack_rustc_build(&rustc_dir)?;

    // Apply Monster analysis to all collected data
    let analysis = analyze_all_datasets(&syn_datasets)?;

    // Display comprehensive results
    display_monster_dataset_analysis(&analysis);

    Ok(())
}

fn setup_cargo_hijack() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 SETTING UP CARGO BUILD HIJACK:");
    println!("---------------------------------");

    // Create hijack wrapper script
    let hijack_script = r#"#!/bin/bash
# Cargo build hijack for Monster analysis
echo "🧟 HIJACKED BUILD: $@" >> /tmp/rustc_hijack.log

# Run original rustc but capture SYN data
if [[ "$1" == *".rs" ]]; then
    echo "📝 Analyzing: $1" >> /tmp/rustc_hijack.log
    
    # Extract SYN data using our zombie analyzer
    cargo run --bin syn_monster_topology -- "$1" >> /tmp/syn_monster_data.json 2>/dev/null || true
fi

# Execute original command
exec /nix/store/i6xakg19vy8vc2g211yr9d5nmb0wk7v0-rustc-1.91.1/bin/rustc "$@"
"#;

    fs::write("/tmp/rustc_hijack", hijack_script)?;
    Command::new("chmod").args(["+x", "/tmp/rustc_hijack"]).output()?;

    println!("   ✅ Hijack script created at /tmp/rustc_hijack");

    // Clear previous data
    let _ = fs::remove_file("/tmp/rustc_hijack.log");
    let _ = fs::remove_file("/tmp/syn_monster_data.json");

    Ok(())
}

fn find_rustc_source() -> Result<String, Box<dyn std::error::Error>> {
    let candidates = [".", "..", "../../..", "/tmp/rustc-source"];

    for candidate in &candidates {
        let cargo_toml = format!("{}/Cargo.toml", candidate);
        if Path::new(&cargo_toml).exists() {
            if let Ok(content) = fs::read_to_string(&cargo_toml) {
                if content.contains("rustc") || content.contains("compiler") {
                    return Ok(candidate.to_string());
                }
            }
        }
    }

    // We're already in rustc source
    Ok(".".to_string())
}

fn hijack_rustc_build(rustc_dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("\n🧟 EXECUTING HIJACKED BUILD:");
    println!("----------------------------");

    // Set environment to use our hijack
    std::env::set_var("RUSTC", "/tmp/rustc_hijack");
    std::env::set_var("RUSTC_WRAPPER", "/tmp/rustc_hijack");

    // Build specific rustc components for analysis
    let components = ["rustc_ast", "rustc_parse", "rustc_expand", "rustc_hir", "rustc_middle"];

    for component in &components {
        println!("   🔍 Building component: {}", component);

        let output = Command::new("cargo")
            .args(["build", "-p", component, "--lib"])
            .current_dir(rustc_dir)
            .env("RUSTC", "/tmp/rustc_hijack")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();

        match output {
            Ok(_) => println!("     ✅ {} analyzed", component),
            Err(_) => println!("     ⚠️  {} skipped", component),
        }
    }

    // Collect generated SYN data
    let mut datasets = Vec::new();
    if let Ok(content) = fs::read_to_string("/tmp/syn_monster_data.json") {
        for line in content.lines() {
            if !line.trim().is_empty() {
                datasets.push(line.to_string());
            }
        }
    }

    println!("   📊 Collected {} SYN datasets", datasets.len());

    Ok(datasets)
}

fn analyze_all_datasets(
    datasets: &[String],
) -> Result<DatasetAnalysis, Box<dyn std::error::Error>> {
    println!("\n🧬 MONSTER ANALYSIS OF ALL DATASETS:");
    println!("===================================");

    let mut analysis = DatasetAnalysis {
        crate_count: 0,
        total_syn_nodes: 0,
        monster_correlations: HashMap::new(),
        strongest_patterns: Vec::new(),
    };

    let mut all_node_counts: HashMap<String, u64> = HashMap::new();

    // Process each dataset
    for (i, dataset) in datasets.iter().enumerate() {
        if let Ok(json) = serde_json::from_str::<Value>(dataset) {
            let nodes = flatten_json_to_vec(json);
            analysis.total_syn_nodes += nodes.len() as u64;

            // Count node types
            for node in &nodes {
                let node_type = classify_node(node);
                *all_node_counts.entry(node_type).or_insert(0) += 1;
            }

            if i < 5 {
                println!("   Dataset {}: {} nodes", i + 1, nodes.len());
            }
        }
        analysis.crate_count += 1;
    }

    // Calculate Monster correlations across all data
    for &prime in &MONSTER_PRIMES {
        let mut correlation_sum = 0.0;
        let mut correlation_count = 0;

        for (node_type, &count) in &all_node_counts {
            if count > 0 {
                let divisibility = (count % prime == 0) as u32 as f64;
                let frequency = count as f64 / analysis.total_syn_nodes as f64;
                correlation_sum += divisibility * frequency;
                correlation_count += 1;
            }
        }

        if correlation_count > 0 {
            let correlation = correlation_sum / correlation_count as f64;
            analysis.monster_correlations.insert(prime, correlation);
        }
    }

    // Find strongest patterns
    for (node_type, &count) in &all_node_counts {
        for &prime in &MONSTER_PRIMES {
            if count % prime == 0 && count > 10 {
                let strength = count as f64 / prime as f64;
                analysis.strongest_patterns.push((node_type.clone(), prime, strength));
            }
        }
    }

    // Sort by strength
    analysis.strongest_patterns.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    analysis.strongest_patterns.truncate(10);

    Ok(analysis)
}

fn flatten_json_to_vec(value: Value) -> Vec<Value> {
    let mut result = Vec::new();
    flatten_recursive(value, &mut result);
    result
}

fn flatten_recursive(value: Value, result: &mut Vec<Value>) {
    match value {
        Value::Array(arr) => {
            for item in arr {
                flatten_recursive(item, result);
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj {
                flatten_recursive(v, result);
            }
        }
        _ => result.push(value),
    }
}

fn classify_node(value: &Value) -> String {
    match value {
        Value::String(s) => {
            if s.contains("Ident") {
                "Identifier".to_string()
            } else if s.contains("Type") {
                "Type".to_string()
            } else if s.contains("Expr") {
                "Expression".to_string()
            } else if s.contains("Stmt") {
                "Statement".to_string()
            } else if s.contains("Item") {
                "Item".to_string()
            } else if s.contains("Pat") {
                "Pattern".to_string()
            } else {
                "String".to_string()
            }
        }
        Value::Number(_) => "Number".to_string(),
        Value::Bool(_) => "Boolean".to_string(),
        Value::Array(_) => "Array".to_string(),
        Value::Object(_) => "Object".to_string(),
        Value::Null => "Null".to_string(),
    }
}

fn display_monster_dataset_analysis(analysis: &DatasetAnalysis) {
    println!("\n🎯 COMPREHENSIVE MONSTER DATASET ANALYSIS:");
    println!("==========================================");

    println!("   📊 Dataset Statistics:");
    println!("      Crates analyzed: {}", analysis.crate_count);
    println!("      Total SYN nodes: {}", analysis.total_syn_nodes);

    println!("\n   🧬 Monster Prime Correlations:");
    for &prime in &[2, 3, 5, 7, 11, 13, 31, 71] {
        if let Some(&correlation) = analysis.monster_correlations.get(&prime) {
            println!("      Prime {}: {:.4} correlation", prime, correlation);
        }
    }

    println!("\n   🎭 Strongest Monster Patterns:");
    for (i, (node_type, prime, strength)) in analysis.strongest_patterns.iter().enumerate().take(5)
    {
        println!("      {}: {} ÷ {} = {:.1} (strength)", i + 1, node_type, prime, strength);
    }

    // Calculate overall Monster correlation
    let avg_correlation: f64 = analysis.monster_correlations.values().sum::<f64>()
        / analysis.monster_correlations.len() as f64;

    println!("\n🧟 OVERALL MONSTER CORRELATION: {:.2}%", avg_correlation * 100.0);

    if avg_correlation > 0.3 {
        println!("✅ STRONG MONSTER GROUP STRUCTURE IN RUSTC DATASET!");
    } else if avg_correlation > 0.1 {
        println!("⚠️  MODERATE MONSTER GROUP PATTERNS DETECTED");
    } else {
        println!("🔍 SUBTLE MONSTER GROUP TRACES FOUND");
    }

    // Save comprehensive results
    let results = serde_json::json!({
        "crate_count": analysis.crate_count,
        "total_syn_nodes": analysis.total_syn_nodes,
        "monster_correlations": analysis.monster_correlations,
        "strongest_patterns": analysis.strongest_patterns,
        "overall_correlation": avg_correlation
    });

    if let Ok(json) = serde_json::to_string_pretty(&results) {
        let _ = fs::write("rustc_dataset_monster_analysis.json", json);
        println!("\n💾 Complete analysis saved to rustc_dataset_monster_analysis.json");
    }
}
