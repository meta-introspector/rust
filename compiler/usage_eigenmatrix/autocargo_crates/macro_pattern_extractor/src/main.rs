// macro_pattern_extractor.rs - Extract common usage patterns and generate macros
use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(Debug, Clone)]
struct UsagePattern {
    symbol: String,
    depth: usize,
    usage_count: u64,
    dependencies: Vec<String>,
    common_contexts: Vec<String>,
}

#[derive(Debug)]
struct MacroTemplate {
    name: String,
    pattern: String,
    args: Vec<String>,
    usage_examples: Vec<String>,
    frequency: u64,
}

fn extract_symbol_name_parts(symbol: &str) -> (String, String, Vec<String>) {
    let parts: Vec<&str> = symbol.split("::").collect();
    let crate_name = parts.first().unwrap_or(&"unknown").to_string();
    let symbol_name = parts.last().unwrap_or(&"unknown").to_string();
    let path_parts = parts.iter().map(|s| s.to_string()).collect();
    (crate_name, symbol_name, path_parts)
}

fn generate_macro_name(pattern: &UsagePattern) -> String {
    let (crate_name, symbol_name, _) = extract_symbol_name_parts(&pattern.symbol);
    let depth_suffix = match pattern.depth {
        0 => "direct",
        1 => "level1",
        2 => "level2", 
        3 => "level3",
        _ => "deep"
    };
    
    format!("use_{}_{}__{}", 
           crate_name.replace("-", "_"), 
           symbol_name.replace("-", "_"),
           depth_suffix)
}

fn analyze_usage_patterns(data: &Value) -> Vec<UsagePattern> {
    let mut patterns = Vec::new();
    
    if let Value::Object(analysis) = data {
        for (symbol_key, symbol_data) in analysis {
            if let Value::Object(obj) = symbol_data {
                let direct_usage = obj.get("direct_usage")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                
                // Skip low-usage symbols
                if direct_usage < 10 {
                    continue;
                }
                
                let mut dependencies = Vec::new();
                let mut max_depth = 0;
                
                if let Some(levels) = obj.get("levels").and_then(|v| v.as_array()) {
                    for (level, level_data) in levels.iter().enumerate() {
                        if let Some(usage) = level_data.get("total_usage").and_then(|v| v.as_u64()) {
                            if usage > 0 {
                                max_depth = level + 1;
                            }
                        }
                        
                        if let Some(deps) = level_data.get("dependencies").and_then(|v| v.as_array()) {
                            for dep in deps {
                                if let Some(dep_str) = dep.as_str() {
                                    dependencies.push(dep_str.to_string());
                                }
                            }
                        }
                    }
                }
                
                patterns.push(UsagePattern {
                    symbol: symbol_key.clone(),
                    depth: max_depth,
                    usage_count: direct_usage,
                    dependencies,
                    common_contexts: vec![], // TODO: extract from source analysis
                });
            }
        }
    }
    
    // Sort by usage frequency
    patterns.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
    patterns
}

fn generate_macro_templates(patterns: &[UsagePattern]) -> Vec<MacroTemplate> {
    let mut templates = Vec::new();
    
    // Group patterns by similar structure
    let mut pattern_groups: HashMap<String, Vec<&UsagePattern>> = HashMap::new();
    
    for pattern in patterns {
        let (crate_name, _, _) = extract_symbol_name_parts(&pattern.symbol);
        let key = format!("{}_{}", crate_name, pattern.depth);
        pattern_groups.entry(key).or_default().push(pattern);
    }
    
    for (group_key, group_patterns) in pattern_groups {
        if group_patterns.len() < 3 {
            continue; // Skip groups with too few patterns
        }
        
        let total_usage: u64 = group_patterns.iter().map(|p| p.usage_count).sum();
        let avg_deps = group_patterns.iter().map(|p| p.dependencies.len()).sum::<usize>() / group_patterns.len();
        
        let macro_name = format!("use_pattern_{}", group_key);
        
        // Generate macro based on common patterns
        let macro_pattern = if avg_deps == 0 {
            "pub use $crate::*;".to_string()
        } else if avg_deps < 5 {
            "pub use $crate::{$($items:ident),*};".to_string()
        } else {
            "pub use $crate::prelude::*;\npub use $crate::{$($items:ident),*};".to_string()
        };
        
        let args = if avg_deps == 0 {
            vec!["crate".to_string()]
        } else {
            vec!["crate".to_string(), "items".to_string()]
        };
        
        let examples: Vec<String> = group_patterns.iter().take(3)
            .map(|p| format!("{}!({})", macro_name, extract_symbol_name_parts(&p.symbol).0))
            .collect();
        
        templates.push(MacroTemplate {
            name: macro_name,
            pattern: macro_pattern,
            args,
            usage_examples: examples,
            frequency: total_usage,
        });
    }
    
    // Sort by frequency
    templates.sort_by(|a, b| b.frequency.cmp(&a.frequency));
    templates
}

fn generate_specific_symbol_macros(patterns: &[UsagePattern]) -> Vec<MacroTemplate> {
    let mut templates = Vec::new();
    
    // Generate macros for top 20 most used symbols
    for pattern in patterns.iter().take(20) {
        let macro_name = generate_macro_name(pattern);
        let (crate_name, symbol_name, _path_parts) = extract_symbol_name_parts(&pattern.symbol);
        
        let macro_pattern = if pattern.dependencies.is_empty() {
            format!("pub use {}::{};", crate_name, symbol_name)
        } else if pattern.dependencies.len() < 3 {
            format!("pub use {}::{{{}}};", crate_name, 
                   pattern.dependencies.iter().map(|d| d.split("::").last().unwrap_or(d))
                   .collect::<Vec<_>>().join(", "))
        } else {
            format!("pub use {}::prelude::*;\npub use {}::{};", crate_name, crate_name, symbol_name)
        };
        
        templates.push(MacroTemplate {
            name: macro_name.clone(),
            pattern: macro_pattern,
            args: vec![],
            usage_examples: vec![format!("{}!()", macro_name)],
            frequency: pattern.usage_count,
        });
    }
    
    templates
}

fn write_macro_file(templates: &[MacroTemplate]) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    content.push_str("// Generated usage pattern macros\n");
    content.push_str("// Auto-generated from 8D vector usage analysis\n\n");
    
    for template in templates {
        content.push_str(&format!("/// Usage frequency: {}\n", template.frequency));
        content.push_str(&format!("/// Examples: {}\n", template.usage_examples.join(", ")));
        
        if template.args.is_empty() {
            content.push_str(&format!("macro_rules! {} {{\n", template.name));
            content.push_str("    () => {\n");
            content.push_str(&format!("        {}\n", template.pattern));
            content.push_str("    };\n");
            content.push_str("}\n\n");
        } else {
            content.push_str(&format!("macro_rules! {} {{\n", template.name));
            content.push_str(&format!("    (${}:ident) => {{\n", template.args[0]));
            content.push_str(&format!("        {}\n", template.pattern));
            content.push_str("    };\n");
            content.push_str("}\n\n");
        }
    }
    
    fs::write("src/generated_usage_macros.rs", content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Extracting Usage Pattern Macros");
    
    // Load usage data
    let file = fs::File::open("usage_levels_8.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    
    let json: Value = serde_json::from_str(&contents)?;
    
    // Analyze patterns
    let patterns = analyze_usage_patterns(&json);
    println!("📊 Found {} usage patterns", patterns.len());
    
    // Generate macro templates
    let pattern_templates = generate_macro_templates(&patterns);
    let specific_templates = generate_specific_symbol_macros(&patterns);
    
    println!("🔧 Generated {} pattern macros", pattern_templates.len());
    println!("🔧 Generated {} specific symbol macros", specific_templates.len());
    
    // Combine and write
    let mut all_templates = pattern_templates;
    all_templates.extend(specific_templates);
    
    write_macro_file(&all_templates)?;
    
    // Print summary
    println!("\n🏆 Top 10 Generated Macros:");
    for (i, template) in all_templates.iter().take(10).enumerate() {
        println!("{}. {} (frequency: {})", i + 1, template.name, template.frequency);
        println!("   Pattern: {}", template.pattern.lines().next().unwrap_or(""));
    }
    
    println!("\n💾 Macros saved to: src/generated_usage_macros.rs");
    
    Ok(())
}
