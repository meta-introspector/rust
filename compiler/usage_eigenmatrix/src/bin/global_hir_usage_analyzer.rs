use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug, Clone)]
struct HirUsagePattern {
    method: String,
    global_count: u32,
    our_count: u32,
    coverage_gap: f64,
    suggestion_priority: f64,
}

struct GlobalUsageAnalyzer {
    global_hir_usage: HashMap<String, u32>,
    our_hir_usage: HashMap<String, u32>,
    tyctxt_methods: Vec<String>,
}

impl GlobalUsageAnalyzer {
    fn new() -> Self {
        Self {
            global_hir_usage: HashMap::new(),
            our_hir_usage: HashMap::new(),
            tyctxt_methods: vec![
                "def_path_str".to_string(),
                "def_kind".to_string(), 
                "type_of".to_string(),
                "hir".to_string(),
                "generics_of".to_string(),
                "predicates_of".to_string(),
                "associated_item".to_string(),
                "param_env".to_string(),
                "lang_items".to_string(),
            ],
        }
    }
    
    fn scan_global_usage(&mut self) {
        println!("🔍 Scanning global HIR usage patterns...");
        
        let usage_dir = "../../usage_data";
        if let Ok(entries) = fs::read_dir(usage_dir) {
            let mut file_count = 0;
            for entry in entries.flatten().take(100) { // Sample first 100 files
                if let Some(name) = entry.file_name().to_str() {
                    if name.contains("hir") && name.ends_with(".json") {
                        self.process_usage_file(&entry.path().display().to_string());
                        file_count += 1;
                    }
                }
            }
            println!("  Processed {} HIR-related files", file_count);
        }
    }
    
    fn process_usage_file(&mut self, filepath: &str) {
        if let Ok(content) = fs::read_to_string(filepath) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(usages) = json["usages"].as_array() {
                    for usage in usages {
                        if let Some(usage_str) = usage["usage"].as_str() {
                            self.extract_hir_methods(usage_str);
                        }
                    }
                }
            }
        }
    }
    
    fn extract_hir_methods(&mut self, usage_str: &str) {
        for method in &self.tyctxt_methods {
            if usage_str.contains(method) {
                *self.global_hir_usage.entry(method.clone()).or_insert(0) += 1;
            }
        }
        
        // Extract other HIR patterns
        if usage_str.contains("TyCtxt") {
            *self.global_hir_usage.entry("TyCtxt".to_string()).or_insert(0) += 1;
        }
        if usage_str.contains("ExprKind") {
            *self.global_hir_usage.entry("ExprKind".to_string()).or_insert(0) += 1;
        }
        if usage_str.contains("TyKind") {
            *self.global_hir_usage.entry("TyKind".to_string()).or_insert(0) += 1;
        }
    }
    
    fn load_our_usage(&mut self) {
        // From our previous analysis
        self.our_hir_usage.insert("def_path_str".to_string(), 1);
        self.our_hir_usage.insert("def_kind".to_string(), 1);
        self.our_hir_usage.insert("type_of".to_string(), 1);
        self.our_hir_usage.insert("hir".to_string(), 1);
        self.our_hir_usage.insert("TyCtxt".to_string(), 4);
    }
    
    fn compute_suggestions(&self) -> Vec<HirUsagePattern> {
        let mut patterns = Vec::new();
        
        for (method, global_count) in &self.global_hir_usage {
            let our_count = self.our_hir_usage.get(method).unwrap_or(&0);
            let coverage_gap = if *global_count > 0 {
                1.0 - (*our_count as f64 / *global_count as f64)
            } else {
                0.0
            };
            
            let priority = (*global_count as f64) * coverage_gap;
            
            patterns.push(HirUsagePattern {
                method: method.clone(),
                global_count: *global_count,
                our_count: *our_count,
                coverage_gap,
                suggestion_priority: priority,
            });
        }
        
        patterns.sort_by(|a, b| b.suggestion_priority.partial_cmp(&a.suggestion_priority).unwrap());
        patterns
    }
    
    fn generate_usage_suggestions(&self, patterns: &[HirUsagePattern]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for pattern in patterns.iter().take(5) {
            if pattern.coverage_gap > 0.8 {
                let suggestion = match pattern.method.as_str() {
                    "def_path_str" => format!("tcx.def_path_str(def_id) // Used {} times globally", pattern.global_count),
                    "associated_item" => format!("tcx.associated_item(def_id) // Used {} times globally", pattern.global_count),
                    "param_env" => format!("tcx.param_env(def_id) // Used {} times globally", pattern.global_count),
                    "predicates_of" => format!("tcx.predicates_of(def_id) // Used {} times globally", pattern.global_count),
                    "lang_items" => format!("tcx.lang_items() // Used {} times globally", pattern.global_count),
                    _ => format!("tcx.{}(...) // Used {} times globally", pattern.method, pattern.global_count),
                };
                suggestions.push(suggestion);
            }
        }
        
        suggestions
    }
}

fn main() {
    println!("🚀 Analyzing global HIR usage patterns for auto-suggestions...");
    
    let mut analyzer = GlobalUsageAnalyzer::new();
    analyzer.scan_global_usage();
    analyzer.load_our_usage();
    
    let patterns = analyzer.compute_suggestions();
    let suggestions = analyzer.generate_usage_suggestions(&patterns);
    
    println!("\n📊 HIR Usage Gap Analysis:");
    println!("=========================");
    
    for (i, pattern) in patterns.iter().take(10).enumerate() {
        let status = if pattern.coverage_gap > 0.9 { "🔴" }
                    else if pattern.coverage_gap > 0.5 { "🟡" } 
                    else { "🟢" };
        
        println!("{}. {} {} (global:{}, ours:{}, gap:{:.1}%)", 
                i + 1, status, pattern.method,
                pattern.global_count, pattern.our_count, 
                pattern.coverage_gap * 100.0);
    }
    
    println!("\n🎯 Auto-Generated Usage Suggestions:");
    println!("====================================");
    
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("{}. {}", i + 1, suggestion);
    }
    
    let total_gap = patterns.iter()
        .map(|p| p.coverage_gap)
        .sum::<f64>() / patterns.len() as f64;
    
    println!("\n📈 Summary:");
    println!("  Total HIR methods tracked: {}", patterns.len());
    println!("  High-priority suggestions: {}", suggestions.len());
    println!("  Average coverage gap: {:.1}%", total_gap * 100.0);
    
    if total_gap > 0.7 {
        println!("🚨 Major HIR usage gaps detected - expand collector!");
    } else {
        println!("✅ Good HIR coverage relative to global usage");
    }
}
