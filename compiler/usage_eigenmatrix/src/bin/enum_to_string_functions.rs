use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("🔍 Finding functions: Enum → String");
    
    let usage_data_dir = "../../usage_data";
    let mut enum_to_string_functions = HashMap::new();
    let mut total_files = 0;
    let mut processed_files = 0;
    
    if let Ok(entries) = fs::read_dir(usage_data_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    total_files += 1;
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            analyze_enum_to_string_functions(&json, &mut enum_to_string_functions);
                            processed_files += 1;
                        }
                    }
                }
            }
        }
    }
    
    println!("📊 Analysis Results:");
    println!("   Files processed: {}/{}", processed_files, total_files);
    println!("   Enum→String functions found: {}", enum_to_string_functions.len());
    println!();
    
    // Sort by frequency
    let mut sorted_functions: Vec<_> = enum_to_string_functions.into_iter().collect();
    sorted_functions.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    println!("🎯 Functions with domain Enum and codomain String:");
    for (function_sig, usages) in sorted_functions.iter().take(30) {
        println!("   {} (used {} times)", function_sig, usages.len());
        for usage in usages.iter().take(2) {
            println!("     - {}", usage);
        }
        if usages.len() > 2 {
            println!("     ... and {} more", usages.len() - 2);
        }
        println!();
    }
    
    // Save results
    let output = serde_json::json!({
        "analysis_type": "enum_to_string_functions",
        "total_functions": sorted_functions.len(),
        "functions": sorted_functions.into_iter().map(|(sig, usages)| {
            serde_json::json!({
                "function_signature": sig,
                "usage_count": usages.len(),
                "sample_usages": usages.into_iter().take(5).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>()
    });
    
    fs::write("enum_to_string_functions.json", serde_json::to_string_pretty(&output).unwrap())
        .expect("Failed to write results");
    
    println!("💾 Results saved to enum_to_string_functions.json");
}

fn analyze_enum_to_string_functions(json: &Value, results: &mut HashMap<String, Vec<String>>) {
    if let Some(usages) = json.get("usages").and_then(|u| u.as_array()) {
        for usage in usages {
            if let (Some(usage_str), Some(used_def_id)) = (
                usage.get("usage").and_then(|u| u.as_str()),
                usage.get("used_def_id").and_then(|u| u.as_str())
            ) {
                // Look for function signatures that map enums to strings
                if is_enum_to_string_function(usage_str, used_def_id) {
                    let function_sig = extract_function_signature(usage_str, used_def_id);
                    results.entry(function_sig)
                        .or_insert_with(Vec::new)
                        .push(format!("{}: {}", used_def_id, usage_str));
                }
            }
        }
    }
}

fn is_enum_to_string_function(usage: &str, used_def_id: &str) -> bool {
    // Function must return string-like type
    let returns_string = [
        "String", "str", "&str", "Cow<str>", "ToString", "Display", "Debug"
    ].iter().any(|s| used_def_id.contains(s) || usage.contains(s));
    
    // Function must take enum-like input
    let takes_enum = [
        "Kind", "Type", "Variant", "State", "Mode", "Status", "Level",
        "Visibility", "Item", "Expr", "Pat", "Stmt", "Ty", "Path",
        "Mutability", "Safety", "Constness", "Async", "Unsafety"
    ].iter().any(|s| used_def_id.contains(s) || usage.contains(s));
    
    // Look for specific function patterns
    let is_conversion_function = [
        "to_string", "as_str", "name", "kind_name", "description", 
        "display", "fmt", "serialize", "encode", "render"
    ].iter().any(|s| used_def_id.contains(s) || usage.contains(s));
    
    // Look for AST-specific patterns
    let is_ast_function = [
        "rustc_ast", "rustc_hir", "syn::", "ast::", "hir::",
        "visit", "walk", "fold", "transform"
    ].iter().any(|s| used_def_id.contains(s) || usage.contains(s));
    
    (returns_string && takes_enum) || is_conversion_function || is_ast_function
}

fn extract_function_signature(usage: &str, used_def_id: &str) -> String {
    // Extract meaningful function signature
    if used_def_id.contains("::") {
        // Parse DefId format: DefId(crate_id:item_id ~ crate[hash]::path::function)
        if let Some(start) = used_def_id.find("::") {
            if let Some(end) = used_def_id.rfind(")") {
                let path = &used_def_id[..end];
                if let Some(tilde_pos) = path.find(" ~ ") {
                    let function_path = &path[tilde_pos + 3..];
                    return function_path.to_string();
                }
            }
        }
    }
    
    // Fallback to DefId
    used_def_id.to_string()
}
