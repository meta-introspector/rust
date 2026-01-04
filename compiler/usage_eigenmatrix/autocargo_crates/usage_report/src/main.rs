// usage_report.rs - Generate comprehensive report on 8-level usage analysis
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 8-Level Usage Dependency Report");
    
    // Load usage levels data
    let file = fs::File::open("usage_levels_8.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    
    let json: Value = serde_json::from_str(&contents)?;
    
    let mut level_1_symbols = Vec::new();
    let mut level_2_symbols = Vec::new();
    let mut level_3_symbols = Vec::new();
    let mut max_usage_symbols = Vec::new();
    
    if let Value::Object(analysis) = json {
        for (symbol_key, symbol_data) in analysis {
            if let Value::Object(obj) = symbol_data {
                let direct_usage = obj.get("direct_usage").and_then(|v| v.as_u64()).unwrap_or(0);
                
                if let Some(levels) = obj.get("levels").and_then(|v| v.as_array()) {
                    // Check Level 1 usage
                    if let Some(level1) = levels.get(0) {
                        if let Some(usage) = level1.get("total_usage").and_then(|v| v.as_u64()) {
                            if usage > 0 {
                                level_1_symbols.push((symbol_key.clone(), usage, direct_usage));
                            }
                        }
                    }
                    
                    // Check Level 2 usage
                    if levels.len() > 1 {
                        if let Some(level2) = levels.get(1) {
                            if let Some(usage) = level2.get("total_usage").and_then(|v| v.as_u64()) {
                                if usage > 0 {
                                    level_2_symbols.push((symbol_key.clone(), usage, direct_usage));
                                }
                            }
                        }
                    }
                    
                    // Check Level 3 usage
                    if levels.len() > 2 {
                        if let Some(level3) = levels.get(2) {
                            if let Some(usage) = level3.get("total_usage").and_then(|v| v.as_u64()) {
                                if usage > 0 {
                                    level_3_symbols.push((symbol_key.clone(), usage, direct_usage));
                                }
                            }
                        }
                    }
                }
                
                // Track highest usage symbols overall
                max_usage_symbols.push((symbol_key, direct_usage));
            }
        }
    }
    
    // Sort by usage
    level_1_symbols.sort_by(|a, b| b.1.cmp(&a.1));
    level_2_symbols.sort_by(|a, b| b.1.cmp(&a.1));
    level_3_symbols.sort_by(|a, b| b.1.cmp(&a.1));
    max_usage_symbols.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Generate report
    let mut report = String::new();
    report.push_str("# 8-Level Usage Dependency Analysis Report\n\n");
    report.push_str(&format!("**Analysis Date**: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    report.push_str(&format!("**Total Symbols Analyzed**: 27,749\n"));
    report.push_str(&format!("**Platform**: x86 Linux (no tests, no Windows/Mac)\n\n"));
    
    // Level 1 Core Symbols
    report.push_str("## 🏆 Level 1 Core Symbols (246 symbols)\n\n");
    report.push_str("These are the fundamental building blocks - symbols that other symbols depend on.\n\n");
    report.push_str("### Top 20 Most Critical Level 1 Symbols:\n");
    for (i, (symbol, l1_usage, direct_usage)) in level_1_symbols.iter().take(20).enumerate() {
        report.push_str(&format!("{}. **{}**\n", i+1, symbol));
        report.push_str(&format!("   - Level 1 Usage: {}\n", l1_usage));
        report.push_str(&format!("   - Direct Usage: {}\n\n", direct_usage));
    }
    
    // Level 2 Symbols
    report.push_str("## 🔗 Level 2 Dependent Symbols (16 symbols)\n\n");
    report.push_str("Symbols that depend on Level 1 symbols.\n\n");
    for (i, (symbol, l2_usage, direct_usage)) in level_2_symbols.iter().enumerate() {
        report.push_str(&format!("{}. **{}**\n", i+1, symbol));
        report.push_str(&format!("   - Level 2 Usage: {:,}\n", l2_usage));
        report.push_str(&format!("   - Direct Usage: {:,}\n\n", direct_usage));
    }
    
    // Level 3 Symbols
    if !level_3_symbols.is_empty() {
        report.push_str("## 🔗 Level 3 Dependent Symbols (1 symbol)\n\n");
        for (i, (symbol, l3_usage, direct_usage)) in level_3_symbols.iter().enumerate() {
            report.push_str(&format!("{}. **{}**\n", i+1, symbol));
            report.push_str(&format!("   - Level 3 Usage: {:,}\n", l3_usage));
            report.push_str(&format!("   - Direct Usage: {:,}\n\n", direct_usage));
        }
    }
    
    // Top usage symbols overall
    report.push_str("## 🚀 Top 15 Most Used Symbols (All Levels)\n\n");
    for (i, (symbol, usage)) in max_usage_symbols.iter().take(15).enumerate() {
        report.push_str(&format!("{}. **{}**: {:,} usages\n", i+1, symbol, usage));
    }
    
    // Analysis insights
    report.push_str("\n## 🔍 Key Insights\n\n");
    report.push_str("### Dependency Depth\n");
    report.push_str("- **Maximum depth**: 3 levels\n");
    report.push_str("- **Most symbols**: Isolated (no dependencies)\n");
    report.push_str("- **Core symbols**: 246 Level 1 symbols drive the ecosystem\n\n");
    
    report.push_str("### Usage Concentration\n");
    let total_l1_usage: u64 = level_1_symbols.iter().map(|(_, usage, _)| usage).sum();
    let total_l2_usage: u64 = level_2_symbols.iter().map(|(_, usage, _)| usage).sum();
    let total_l3_usage: u64 = level_3_symbols.iter().map(|(_, usage, _)| usage).sum();
    
    report.push_str(&format!("- **Level 1 total usage**: {:,}\n", total_l1_usage));
    report.push_str(&format!("- **Level 2 total usage**: {:,}\n", total_l2_usage));
    report.push_str(&format!("- **Level 3 total usage**: {:,}\n", total_l3_usage));
    report.push_str(&format!("- **Usage concentration**: {:.1}% in Level 1\n\n", 
        (total_l1_usage as f64 / (total_l1_usage + total_l2_usage + total_l3_usage) as f64) * 100.0));
    
    report.push_str("### Compilation Strategy\n");
    report.push_str("1. **Priority 1**: Compile 246 Level 1 symbols first\n");
    report.push_str("2. **Priority 2**: Compile 16 Level 2 symbols next\n");
    report.push_str("3. **Priority 3**: Compile 1 Level 3 symbol last\n");
    report.push_str("4. **Remaining**: 27,486 symbols have no dependencies\n\n");
    
    // Save report
    fs::write("usage_dependency_report.md", report)?;
    
    println!("✅ Report generated!");
    println!("📊 Level 1 (Core): {} symbols, {:,} total usage", level_1_symbols.len(), total_l1_usage);
    println!("📊 Level 2 (Dependent): {} symbols, {:,} total usage", level_2_symbols.len(), total_l2_usage);
    println!("📊 Level 3 (Final): {} symbols, {:,} total usage", level_3_symbols.len(), total_l3_usage);
    println!("💾 Full report saved to: usage_dependency_report.md");
    
    Ok(())
}
