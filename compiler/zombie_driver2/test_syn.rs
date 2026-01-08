use syn_analyzer::analyze_file;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("complex_test.rs")?;
    let analysis = analyze_file("complex_test.rs", &content)?;
    
    println!("🔬 Enhanced Syn Analysis Results");
    println!("================================");
    println!("📊 Structs: {}", analysis.structs.len());
    println!("📊 Enums: {}", analysis.enums.len());
    println!("📊 Functions: {}", analysis.functions.len());
    println!("📊 Impls: {}", analysis.impls.len());
    println!("📊 Modules: {}", analysis.modules.len());
    
    println!("\n🏗️  Nesting Statistics:");
    println!("   Max depth: {}", analysis.nesting_stats.max_depth);
    println!("   Avg depth: {:.2}", analysis.nesting_stats.avg_depth);
    println!("   Depth distribution: {:?}", analysis.nesting_stats.depth_counts);
    
    println!("\n🔗 Nested Pairs:");
    for (pair, count) in &analysis.nesting_stats.nested_pairs {
        println!("   {}: {}", pair, count);
    }
    
    println!("\n📈 Sequence Patterns:");
    for (seq, count) in &analysis.sequence_patterns {
        println!("   {}: {}", seq, count);
    }
    
    println!("\n📋 Syntax Patterns:");
    for (pattern, count) in &analysis.syntax_patterns {
        println!("   {}: {}", pattern, count);
    }
    
    // Save detailed analysis
    let json = serde_json::to_string_pretty(&analysis)?;
    fs::write("complex_test.syn_analysis.json", json)?;
    println!("\n📁 Detailed analysis saved to: complex_test.syn_analysis.json");
    
    Ok(())
}
