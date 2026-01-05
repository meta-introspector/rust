use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct UsagePattern {
    pattern: String,
    syn_count: u32,
    hir_count: u32,
    convergence_score: f64,
    mapping_confidence: f64,
}

struct ConvergenceAnalyzer {
    syn_patterns: HashMap<String, u32>,
    hir_patterns: HashMap<String, u32>,
    bijection_map: HashMap<String, String>,
}

impl ConvergenceAnalyzer {
    fn new() -> Self {
        let mut bijection_map = HashMap::new();
        bijection_map.insert("Type".to_string(), "TyKind".to_string());
        bijection_map.insert("Expr".to_string(), "ExprKind".to_string());
        bijection_map.insert("Array".to_string(), "Array".to_string());
        bijection_map.insert("Path".to_string(), "Path".to_string());
        bijection_map.insert("Reference".to_string(), "Ref".to_string());
        bijection_map.insert("Binary".to_string(), "Binary".to_string());
        bijection_map.insert("Call".to_string(), "Call".to_string());
        
        Self {
            syn_patterns: HashMap::new(),
            hir_patterns: HashMap::new(),
            bijection_map,
        }
    }
    
    fn load_usage_data(&mut self) {
        // Mock data - in practice would load from our collector outputs
        self.syn_patterns.insert("Type::Array".to_string(), 45);
        self.syn_patterns.insert("Type::Path".to_string(), 120);
        self.syn_patterns.insert("Type::Reference".to_string(), 67);
        self.syn_patterns.insert("Expr::Binary".to_string(), 89);
        self.syn_patterns.insert("Expr::Call".to_string(), 156);
        self.syn_patterns.insert("parse_type".to_string(), 234);
        self.syn_patterns.insert("visit_expr".to_string(), 178);
        
        self.hir_patterns.insert("TyKind::Array".to_string(), 43);
        self.hir_patterns.insert("TyKind::Path".to_string(), 118);
        self.hir_patterns.insert("TyKind::Ref".to_string(), 65);
        self.hir_patterns.insert("ExprKind::Binary".to_string(), 91);
        self.hir_patterns.insert("ExprKind::Call".to_string(), 154);
        self.hir_patterns.insert("type_of".to_string(), 198);
        self.hir_patterns.insert("def_path_str".to_string(), 164);
    }
    
    fn compute_convergence(&self) -> Vec<UsagePattern> {
        let mut patterns = Vec::new();
        
        // Direct mappings from bijection
        for (syn_key, syn_count) in &self.syn_patterns {
            if let Some(mapped_key) = self.find_hir_mapping(syn_key) {
                if let Some(hir_count) = self.hir_patterns.get(&mapped_key) {
                    let convergence = 1.0 - ((syn_count.abs_diff(*hir_count) as f64) / (*syn_count.max(hir_count) as f64));
                    
                    patterns.push(UsagePattern {
                        pattern: format!("{} ↔ {}", syn_key, mapped_key),
                        syn_count: *syn_count,
                        hir_count: *hir_count,
                        convergence_score: convergence,
                        mapping_confidence: 1.0,
                    });
                }
            }
        }
        
        // Orphaned patterns (no direct mapping)
        for (syn_key, syn_count) in &self.syn_patterns {
            if self.find_hir_mapping(syn_key).is_none() {
                patterns.push(UsagePattern {
                    pattern: format!("{} → ?", syn_key),
                    syn_count: *syn_count,
                    hir_count: 0,
                    convergence_score: 0.0,
                    mapping_confidence: 0.0,
                });
            }
        }
        
        patterns.sort_by(|a, b| b.convergence_score.partial_cmp(&a.convergence_score).unwrap());
        patterns
    }
    
    fn find_hir_mapping(&self, syn_pattern: &str) -> Option<String> {
        for (syn_type, hir_type) in &self.bijection_map {
            if syn_pattern.contains(syn_type) {
                return Some(syn_pattern.replace(syn_type, hir_type));
            }
        }
        None
    }
    
    fn rank_duplicates(&self, patterns: &[UsagePattern]) -> Vec<(String, f64)> {
        let mut duplicates = Vec::new();
        
        for pattern in patterns {
            if pattern.convergence_score > 0.8 {
                let duplicate_score = pattern.convergence_score * pattern.mapping_confidence;
                duplicates.push((pattern.pattern.clone(), duplicate_score));
            }
        }
        
        duplicates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        duplicates
    }
}

fn main() {
    println!("🔍 Analyzing syn↔hir usage convergence...");
    
    let mut analyzer = ConvergenceAnalyzer::new();
    analyzer.load_usage_data();
    
    let patterns = analyzer.compute_convergence();
    let duplicates = analyzer.rank_duplicates(&patterns);
    
    println!("\n📊 Usage Pattern Convergence Rankings:");
    println!("=====================================");
    
    for (i, pattern) in patterns.iter().enumerate() {
        let status = if pattern.convergence_score > 0.9 { "🟢" }
                    else if pattern.convergence_score > 0.7 { "🟡" }
                    else { "🔴" };
        
        println!("{}. {} {} (syn:{}, hir:{}, conv:{:.2})", 
                i + 1, status, pattern.pattern, 
                pattern.syn_count, pattern.hir_count, pattern.convergence_score);
    }
    
    println!("\n🎯 High-Confidence Duplicates:");
    println!("==============================");
    
    for (i, (pattern, score)) in duplicates.iter().enumerate() {
        println!("{}. {} (duplicate score: {:.3})", i + 1, pattern, score);
    }
    
    let total_convergence = patterns.iter()
        .map(|p| p.convergence_score)
        .sum::<f64>() / patterns.len() as f64;
    
    println!("\n📈 Summary:");
    println!("  Total patterns: {}", patterns.len());
    println!("  High duplicates: {}", duplicates.len());
    println!("  Average convergence: {:.1}%", total_convergence * 100.0);
    
    if total_convergence > 0.8 {
        println!("🎉 Strong syn↔hir convergence detected!");
    } else {
        println!("⚠️  Partial convergence - need more mapping data");
    }
}
