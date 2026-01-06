use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Extracting Liftable Usage Examples from Existing Data");
    println!("========================================================");
    
    let mut symbol_examples: HashMap<String, Vec<(String, String, String, String)>> = HashMap::new();
    let mut usage_patterns: HashMap<String, usize> = HashMap::new();
    
    // Process all usage files to find concrete examples
    if let Ok(entries) = fs::read_dir("../../test_usage_data") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") 
                && !path.file_name().unwrap().to_str().unwrap().contains("manifest") {
                
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        let module = data.get("module").or_else(|| data.get("crate"))
                            .and_then(|m| m.as_str()).unwrap_or("unknown");
                        
                        if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                            for usage in usages {
                                if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                    let usage_type = usage.get("usage_type").and_then(|t| t.as_str()).unwrap_or("unknown");
                                    let kind = usage.get("kind").and_then(|k| k.as_str()).unwrap_or("unknown");
                                    
                                    // Categorize by potential expansion areas
                                    let category = categorize_symbol(symbol, usage_type, kind);
                                    if category != "current" {
                                        symbol_examples.entry(category.to_string())
                                            .or_insert_with(Vec::new)
                                            .push((symbol.to_string(), usage_type.to_string(), kind.to_string(), module.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Print examples by category
    for (category, examples) in &symbol_examples {
        println!("\n🎯 {} EXAMPLES:", category.to_uppercase());
        let mut unique_examples: Vec<_> = examples.iter().collect::<HashSet<_>>().into_iter().collect();
        unique_examples.sort();
        
        for (symbol, usage_type, kind, module) in unique_examples.iter().take(10) {
            println!("  📋 {} | {} | {} | {}", symbol, usage_type, kind, module);
        }
        
        if examples.len() > 10 {
            println!("  ... and {} more examples", examples.len() - 10);
        }
    }
    
    // Generate implementation suggestions
    println!("\n💡 LIFTABLE IMPLEMENTATION EXAMPLES:");
    println!("====================================");
    
    if let Some(pattern_examples) = symbol_examples.get("pattern_matching") {
        println!("\n🎯 Pattern Matching (add to working_usage_collector.rs):");
        println!("```rust");
        println!("rustc_hir::ExprKind::Match(expr, arms, _) => {{");
        println!("    self.add_usage(module, \"match_expr\".to_string(), \"pattern_match\".to_string(),");
        println!("                   \"PatternMatch\".to_string(), \"Expression\".to_string(), ...);");
        println!("    for arm in arms {{");
        println!("        // Track pattern complexity");
        println!("    }}");
        println!("}}");
        println!("```");
        println!("Examples found: {}", pattern_examples.len());
    }
    
    if let Some(type_examples) = symbol_examples.get("type_system") {
        println!("\n🔧 Type System (add to working_usage_collector.rs):");
        println!("```rust");
        println!("rustc_hir::TyKind::Path(_) => {{");
        println!("    self.add_usage(module, ty_name, \"type_usage\".to_string(),");
        println!("                   \"TypeAnnotation\".to_string(), \"Type\".to_string(), ...);");
        println!("}}");
        println!("```");
        println!("Examples found: {}", type_examples.len());
    }
    
    if let Some(macro_examples) = symbol_examples.get("macro_usage") {
        println!("\n📊 Macro Usage (expand existing coverage):");
        println!("```rust");
        println!("rustc_hir::ExprKind::MacCall(mac) => {{");
        println!("    let macro_name = mac.path.segments.last().unwrap().ident.name.to_string();");
        println!("    self.add_usage(module, macro_name, \"macro_call\".to_string(),");
        println!("                   \"MacroCall\".to_string(), \"Expression\".to_string(), ...);");
        println!("}}");
        println!("```");
        println!("Examples found: {}", macro_examples.len());
    }
    
    if let Some(control_examples) = symbol_examples.get("control_flow") {
        println!("\n🏗️ Control Flow (add to working_usage_collector.rs):");
        println!("```rust");
        println!("rustc_hir::ExprKind::Loop(block, label, source, _) => {{");
        println!("    self.add_usage(module, \"loop\".to_string(), \"control_flow\".to_string(),");
        println!("                   \"LoopUsage\".to_string(), \"Expression\".to_string(), ...);");
        println!("}}");
        println!("```");
        println!("Examples found: {}", control_examples.len());
    }
    
    Ok(())
}

fn categorize_symbol(symbol: &str, usage_type: &str, kind: &str) -> &'static str {
    let symbol_lower = symbol.to_lowercase();
    let usage_lower = usage_type.to_lowercase();
    let kind_lower = kind.to_lowercase();
    
    // Pattern matching indicators
    if symbol_lower.contains("match") || symbol_lower.contains("pattern") || 
       symbol_lower.contains("arm") || kind_lower.contains("match") {
        return "pattern_matching";
    }
    
    // Type system indicators  
    if symbol_lower.contains("ty") || symbol_lower.contains("type") ||
       symbol_lower.contains("generic") || symbol_lower.contains("trait") ||
       usage_lower.contains("type") || kind_lower.contains("type") {
        return "type_system";
    }
    
    // Macro indicators
    if symbol_lower.contains("macro") || symbol_lower.contains("derive") ||
       symbol_lower.contains("attr") || kind_lower.contains("macro") {
        return "macro_usage";
    }
    
    // Control flow indicators
    if symbol_lower.contains("loop") || symbol_lower.contains("if") ||
       symbol_lower.contains("while") || symbol_lower.contains("for") ||
       symbol_lower.contains("async") || symbol_lower.contains("await") {
        return "control_flow";
    }
    
    // Memory/safety indicators
    if symbol_lower.contains("unsafe") || symbol_lower.contains("ptr") ||
       symbol_lower.contains("raw") || symbol_lower.contains("box") {
        return "memory_safety";
    }
    
    // Field/access indicators
    if symbol_lower.contains("field") || symbol_lower.contains("access") ||
       symbol_lower.contains("index") || symbol_lower.contains("get") {
        return "field_access";
    }
    
    "current"
}
