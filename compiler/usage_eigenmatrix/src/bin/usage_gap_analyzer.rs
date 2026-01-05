use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct UsageComparison {
    item: String,
    global_usage: u32,
    our_usage: u32,
    coverage_percentage: f64,
    priority_score: f64,
}

fn main() {
    println!("📊 Comparing our usage against global baseline...");
    
    // Global usage baseline (from previous analysis)
    let mut global_baseline = HashMap::new();
    global_baseline.insert("DefId".to_string(), 423550);
    global_baseline.insert("hir".to_string(), 40279);
    global_baseline.insert("TyCtxt".to_string(), 7573);
    global_baseline.insert("predicates_of".to_string(), 3098);
    global_baseline.insert("associated_item".to_string(), 1579);
    global_baseline.insert("type_of".to_string(), 1439);
    global_baseline.insert("param_env".to_string(), 1276);
    global_baseline.insert("generics_of".to_string(), 962);
    global_baseline.insert("def_kind".to_string(), 422);
    global_baseline.insert("def_path_str".to_string(), 223);
    global_baseline.insert("HirId".to_string(), 40);
    global_baseline.insert("TyKind".to_string(), 24);
    global_baseline.insert("ExprKind".to_string(), 6);
    
    // Our current usage (from enhanced collector)
    let mut our_usage = HashMap::new();
    our_usage.insert("DefId".to_string(), 2);  // We use it minimally
    our_usage.insert("hir".to_string(), 1);    // Basic usage
    our_usage.insert("TyCtxt".to_string(), 4); // Some usage
    our_usage.insert("def_path_str".to_string(), 1);
    our_usage.insert("def_kind".to_string(), 1);
    our_usage.insert("type_of".to_string(), 1);
    our_usage.insert("generics_of".to_string(), 0); // Missing!
    our_usage.insert("predicates_of".to_string(), 0); // Missing!
    our_usage.insert("associated_item".to_string(), 0); // Missing!
    our_usage.insert("param_env".to_string(), 0); // Missing!
    
    // Calculate comparisons
    let mut comparisons: Vec<UsageComparison> = global_baseline.iter()
        .map(|(item, global_count)| {
            let our_count = our_usage.get(item).unwrap_or(&0);
            let coverage = if *global_count > 0 {
                (*our_count as f64 / *global_count as f64) * 100.0
            } else {
                0.0
            };
            
            // Priority = global importance × coverage gap
            let priority = (*global_count as f64) * (1.0 - coverage / 100.0);
            
            UsageComparison {
                item: item.clone(),
                global_usage: *global_count,
                our_usage: *our_count,
                coverage_percentage: coverage,
                priority_score: priority,
            }
        })
        .collect();
    
    // Sort by priority (biggest gaps in most important items)
    comparisons.sort_by(|a, b| b.priority_score.partial_cmp(&a.priority_score).unwrap());
    
    println!("\n🎯 USAGE GAP ANALYSIS (Sorted by Priority):");
    println!("===========================================");
    
    for (i, comp) in comparisons.iter().take(10).enumerate() {
        let status = if comp.coverage_percentage < 0.01 { "🔴" }
                    else if comp.coverage_percentage < 1.0 { "🟡" }
                    else { "🟢" };
        
        println!("{}. {} {} | Global: {} | Ours: {} | Coverage: {:.3}% | Priority: {:.0}", 
                i + 1, status, comp.item, comp.global_usage, comp.our_usage, 
                comp.coverage_percentage, comp.priority_score);
    }
    
    println!("\n💡 TOP 5 IMPROVEMENT TARGETS:");
    println!("=============================");
    
    for (i, comp) in comparisons.iter().take(5).enumerate() {
        let suggestion = generate_usage_suggestion(&comp.item, comp.global_usage);
        println!("{}. {} (used {} times globally)", i + 1, suggestion, comp.global_usage);
    }
    
    // Calculate overall coverage
    let total_global: u32 = comparisons.iter().map(|c| c.global_usage).sum();
    let total_ours: u32 = comparisons.iter().map(|c| c.our_usage).sum();
    let overall_coverage = (total_ours as f64 / total_global as f64) * 100.0;
    
    println!("\n📈 OVERALL COVERAGE ANALYSIS:");
    println!("============================");
    println!("Global total usage: {}", total_global);
    println!("Our total usage: {}", total_ours);
    println!("Overall coverage: {:.4}%", overall_coverage);
    
    if overall_coverage < 0.1 {
        println!("🚨 CRITICAL: We're using <0.1% of global patterns!");
        println!("🎯 Focus on top 3 items for maximum impact");
    }
    
    // Save detailed analysis
    save_comparison_report(&comparisons, overall_coverage);
}

fn generate_usage_suggestion(item: &str, global_count: u32) -> String {
    match item {
        "DefId" => format!("Add DefId tracking in all TyCtxt operations"),
        "predicates_of" => format!("tcx.predicates_of(def_id) - track type predicates"),
        "associated_item" => format!("tcx.associated_item(def_id) - track trait items"),
        "param_env" => format!("tcx.param_env(def_id) - track parameter environments"),
        "generics_of" => format!("tcx.generics_of(def_id) - track generic parameters"),
        "hir" => format!("Expand HIR traversal and analysis"),
        _ => format!("Increase {} usage patterns", item),
    }
}

fn save_comparison_report(comparisons: &[UsageComparison], overall_coverage: f64) {
    let mut report = String::new();
    report.push_str("# Usage Gap Analysis Report\n\n");
    report.push_str(&format!("Overall Coverage: {:.4}%\n\n", overall_coverage));
    report.push_str("## Priority Rankings\n");
    
    for (i, comp) in comparisons.iter().enumerate() {
        report.push_str(&format!("{}. {} - Global: {}, Ours: {}, Coverage: {:.3}%\n", 
                               i + 1, comp.item, comp.global_usage, comp.our_usage, 
                               comp.coverage_percentage));
    }
    
    fs::write("usage_gap_analysis.txt", report).unwrap();
    println!("📁 Detailed report saved to usage_gap_analysis.txt");
}
