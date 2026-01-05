use std::collections::HashMap;
use serde_json::{json, Value};
use std::fs;

fn main() {
    println!("=== AGGREGATE HARMONIC ANALYSIS ===");
    
    let usage_dir = std::env::var("USAGE_OUTPUT_DIR")
        .unwrap_or_else(|_| "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data".to_string());
    
    let mut crate_stats = HashMap::new();
    let mut total_constructs = 0;
    let mut total_harmonics = 0;
    
    // Scan all usage files
    if let Ok(entries) = fs::read_dir(&usage_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") && !filename.contains("_enums_") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(data) = serde_json::from_str::<Value>(&content) {
                            if let Some(crate_name) = data.get("crate").and_then(|v| v.as_str()) {
                                if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                                    let count = usages.len();
                                    *crate_stats.entry(crate_name.to_string()).or_insert(0) += count;
                                    total_constructs += count;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("--- CRATE ANALYSIS RESULTS ---");
    let mut sorted_crates: Vec<_> = crate_stats.iter().collect();
    sorted_crates.sort_by(|a, b| b.1.cmp(a.1));
    
    for (crate_name, count) in sorted_crates.iter().take(20) {
        println!("  {}: {} constructs", crate_name, count);
    }
    
    println!("\n--- SUMMARY ---");
    println!("Total crates analyzed: {}", crate_stats.len());
    println!("Total constructs: {}", total_constructs);
    println!("Average constructs per crate: {:.1}", 
             total_constructs as f64 / crate_stats.len() as f64);
    
    // Generate aggregate report
    let report = json!({
        "analysis_type": "aggregate_harmonic_analysis",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total_crates": crate_stats.len(),
        "total_constructs": total_constructs,
        "crate_breakdown": crate_stats,
        "top_crates": sorted_crates.iter().take(10).map(|(name, count)| {
            json!({"crate": name, "constructs": count})
        }).collect::<Vec<_>>()
    });
    
    let report_path = format!("{}/aggregate_harmonic_report.json", 
                             std::env::var("HARMONIC_OUTPUT_DIR")
                                 .unwrap_or_else(|_| usage_dir));
    
    if let Ok(report_json) = serde_json::to_string_pretty(&report) {
        fs::write(&report_path, report_json).unwrap();
        println!("✓ Aggregate report saved to: {}", report_path);
    }
}
