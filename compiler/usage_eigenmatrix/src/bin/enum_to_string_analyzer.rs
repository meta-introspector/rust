use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    println!("🔍 Analyzing enum-to-string conversion functions...");
    
    let usage_data_dir = "../../usage_data";
    let mut enum_to_string_functions = HashMap::new();
    let mut total_files = 0;
    let mut processed_files = 0;
    
    // Read all JSON files in usage_data directory
    if let Ok(entries) = fs::read_dir(usage_data_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    total_files += 1;
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            analyze_file(&json, &mut enum_to_string_functions, &path);
                            processed_files += 1;
                        }
                    }
                }
            }
        }
    }
    
    println!("📊 Analysis Results:");
    println!("   Files processed: {}/{}", processed_files, total_files);
    println!("   Enum-to-string functions found: {}", enum_to_string_functions.len());
    println!();
    
    // Sort by frequency
    let mut sorted_functions: Vec<_> = enum_to_string_functions.into_iter().collect();
    sorted_functions.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    println!("🎯 Top enum-to-string conversion patterns:");
    for (pattern, locations) in sorted_functions.iter().take(20) {
        println!("   {} (used {} times)", pattern, locations.len());
        for location in locations.iter().take(3) {
            println!("     - {}", location);
        }
        if locations.len() > 3 {
            println!("     ... and {} more", locations.len() - 3);
        }
        println!();
    }
    
    // Save detailed results
    let output = serde_json::json!({
        "analysis_type": "enum_to_string_conversions",
        "total_files_processed": processed_files,
        "total_patterns_found": sorted_functions.len(),
        "patterns": sorted_functions.into_iter().map(|(pattern, locations)| {
            serde_json::json!({
                "pattern": pattern,
                "usage_count": locations.len(),
                "locations": locations
            })
        }).collect::<Vec<_>>()
    });
    
    fs::write("enum_to_string_analysis.json", serde_json::to_string_pretty(&output).unwrap())
        .expect("Failed to write analysis results");
    
    println!("💾 Detailed results saved to enum_to_string_analysis.json");
}

fn analyze_file(json: &Value, results: &mut HashMap<String, Vec<String>>, file_path: &Path) {
    if let Some(usages) = json.get("usages").and_then(|u| u.as_array()) {
        for usage in usages {
            if let Some(usage_str) = usage.get("usage").and_then(|u| u.as_str()) {
                if let Some(used_def_id) = usage.get("used_def_id").and_then(|u| u.as_str()) {
                    // Look for enum-to-string conversion patterns
                    if is_enum_to_string_pattern(usage_str, used_def_id) {
                        let pattern = extract_pattern(usage_str, used_def_id);
                        let location = format!("{}:{}", 
                            file_path.file_name().unwrap_or_default().to_string_lossy(),
                            usage_str
                        );
                        
                        results.entry(pattern)
                            .or_insert_with(Vec::new)
                            .push(location);
                    }
                }
            }
        }
    }
}

fn is_enum_to_string_pattern(usage: &str, used_def_id: &str) -> bool {
    // Check for common enum-to-string conversion patterns
    let string_conversion_indicators = [
        "to_string", "ToString", "Display", "fmt", "as_str", "into_string",
        "format!", "write!", "writeln!", "print!", "println!"
    ];
    
    let enum_indicators = [
        "ENUM_", "enum", "Kind", "Type", "Variant", "State", "Mode"
    ];
    
    // Check if this looks like a string conversion
    let has_string_conversion = string_conversion_indicators.iter()
        .any(|indicator| usage.contains(indicator) || used_def_id.contains(indicator));
    
    // Check if this involves an enum
    let has_enum_context = enum_indicators.iter()
        .any(|indicator| usage.contains(indicator) || used_def_id.contains(indicator));
    
    // Also look for specific patterns like "match" with string literals
    let has_match_pattern = usage.contains("match") && usage.contains("\"");
    
    has_string_conversion && (has_enum_context || has_match_pattern)
}

fn extract_pattern(usage: &str, used_def_id: &str) -> String {
    // Extract a normalized pattern for grouping similar conversions
    if usage.contains("to_string") {
        format!("enum.to_string() -> {}", used_def_id)
    } else if usage.contains("Display") || usage.contains("fmt") {
        format!("Display/fmt -> {}", used_def_id)
    } else if usage.contains("as_str") {
        format!("enum.as_str() -> {}", used_def_id)
    } else if usage.contains("match") {
        format!("match enum -> string -> {}", used_def_id)
    } else if usage.contains("format!") {
        format!("format! macro -> {}", used_def_id)
    } else {
        format!("other conversion -> {}", used_def_id)
    }
}
