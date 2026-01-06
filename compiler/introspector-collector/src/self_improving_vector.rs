use std::collections::{HashMap, HashSet};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CoverageGap {
    pub pattern: String,
    pub frequency: usize,
    pub implementation_priority: u8, // 1-10
    pub rust_code: String,
    pub estimated_impact: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SelfImprovingVector {
    pub current_patterns: HashSet<String>,
    pub discovered_gaps: Vec<CoverageGap>,
    pub implementation_history: Vec<String>,
    pub coverage_score: f64,
}

impl SelfImprovingVector {
    pub fn new() -> Self {
        Self {
            current_patterns: HashSet::from([
                "StructDecl".to_string(),
                "EnumDecl".to_string(), 
                "GenericParam".to_string(),
                "LifetimeParam".to_string(),
                "ConstParam".to_string(),
            ]),
            discovered_gaps: Vec::new(),
            implementation_history: Vec::new(),
            coverage_score: 0.0,
        }
    }
    
    pub fn analyze_usage_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut pattern_frequencies = HashMap::new();
        
        // Scan all usage data files
        if let Ok(entries) = fs::read_dir("../../test_usage_data") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") 
                    && !path.file_name().unwrap().to_str().unwrap().contains("manifest") {
                    
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                                for usage in usages {
                                    if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                        *pattern_frequencies.entry(symbol.to_string()).or_insert(0) += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Identify high-frequency patterns not yet implemented
        self.discover_gaps(&pattern_frequencies);
        self.prioritize_gaps();
        self.generate_implementations();
        
        Ok(())
    }
    
    fn discover_gaps(&mut self, frequencies: &HashMap<String, usize>) {
        self.discovered_gaps.clear();
        
        for (pattern, &freq) in frequencies {
            if freq > 5 && !self.current_patterns.contains(pattern) {
                let gap = self.classify_pattern(pattern, freq);
                if gap.implementation_priority > 3 {
                    self.discovered_gaps.push(gap);
                }
            }
        }
    }
    
    fn classify_pattern(&self, pattern: &str, frequency: usize) -> CoverageGap {
        let pattern_lower = pattern.to_lowercase();
        
        // Field access patterns
        if pattern_lower.contains("index") || pattern_lower.contains("get") || pattern_lower.contains("field") {
            return CoverageGap {
                pattern: pattern.to_string(),
                frequency,
                implementation_priority: 8,
                rust_code: format!(r#"
rustc_hir::ExprKind::Field(expr, field) => {{
    self.add_usage("field_access", "{}", "field_access".to_string(),
                   "FieldAccess".to_string(), "Expression".to_string(), ...);
}}"#, pattern),
                estimated_impact: frequency as f64 * 0.3,
            };
        }
        
        // Memory safety patterns
        if pattern_lower.contains("unsafe") || pattern_lower.contains("mut") || pattern_lower.contains("ptr") {
            return CoverageGap {
                pattern: pattern.to_string(),
                frequency,
                implementation_priority: 7,
                rust_code: format!(r#"
rustc_hir::ExprKind::Call(func, args) if is_unsafe_call(func) => {{
    self.add_usage("memory_safety", "{}", "unsafe_call".to_string(),
                   "UnsafeUsage".to_string(), "Expression".to_string(), ...);
}}"#, pattern),
                estimated_impact: frequency as f64 * 0.4,
            };
        }
        
        // Pattern matching
        if pattern_lower.contains("match") || pattern_lower.contains("pattern") || pattern_lower.contains("arm") {
            return CoverageGap {
                pattern: pattern.to_string(),
                frequency,
                implementation_priority: 9,
                rust_code: format!(r#"
rustc_hir::ExprKind::Match(expr, arms, _) => {{
    self.add_usage("patterns", "{}", "pattern_match".to_string(),
                   "PatternMatch".to_string(), "Expression".to_string(), ...);
}}"#, pattern),
                estimated_impact: frequency as f64 * 0.5,
            };
        }
        
        // Default low priority
        CoverageGap {
            pattern: pattern.to_string(),
            frequency,
            implementation_priority: 2,
            rust_code: format!("// TODO: Implement {}", pattern),
            estimated_impact: frequency as f64 * 0.1,
        }
    }
    
    fn prioritize_gaps(&mut self) {
        self.discovered_gaps.sort_by(|a, b| {
            let score_a = a.implementation_priority as f64 * a.estimated_impact;
            let score_b = b.implementation_priority as f64 * b.estimated_impact;
            score_b.partial_cmp(&score_a).unwrap()
        });
    }
    
    fn generate_implementations(&mut self) {
        let top_gaps = self.discovered_gaps.iter().take(3).cloned().collect::<Vec<_>>();
        
        if !top_gaps.is_empty() {
            self.create_expansion_module(&top_gaps);
        }
    }
    
    fn create_expansion_module(&self, gaps: &[CoverageGap]) {
        let mut module_code = String::from(r#"
// AUTO-GENERATED EXPANSION MODULE
use crate::usage_types::*;
use rustc_hir;

pub struct AutoExpansion;

impl AutoExpansion {
    pub fn expand_coverage<'tcx>(
        collector: &mut crate::usage_collector::UsageCollector,
        expr: &'tcx rustc_hir::Expr<'tcx>
    ) {
        match &expr.kind {
"#);
        
        for gap in gaps {
            module_code.push_str(&format!("            // Pattern: {} (frequency: {}, priority: {})\n", 
                                         gap.pattern, gap.frequency, gap.implementation_priority));
            module_code.push_str(&gap.rust_code);
            module_code.push_str("\n\n");
        }
        
        module_code.push_str(r#"            _ => {}
        }
    }
}
"#);
        
        if let Ok(mut file) = fs::File::create("src/auto_expansion.rs") {
            use std::io::Write;
            let _ = file.write_all(module_code.as_bytes());
        }
        
        println!("🤖 AUTO-GENERATED: src/auto_expansion.rs with {} new patterns", gaps.len());
    }
    
    pub fn update_coverage_score(&mut self) {
        let total_patterns = self.current_patterns.len() + self.discovered_gaps.len();
        let implemented_patterns = self.current_patterns.len();
        self.coverage_score = (implemented_patterns as f64 / total_patterns as f64) * 100.0;
    }
    
    pub fn self_improve(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Starting self-improvement cycle...");
        
        self.analyze_usage_data()?;
        self.update_coverage_score();
        
        println!("📊 Coverage Analysis:");
        println!("   Current patterns: {}", self.current_patterns.len());
        println!("   Discovered gaps: {}", self.discovered_gaps.len());
        println!("   Coverage score: {:.1}%", self.coverage_score);
        
        if !self.discovered_gaps.is_empty() {
            println!("\n🎯 Top expansion opportunities:");
            for (i, gap) in self.discovered_gaps.iter().take(5).enumerate() {
                println!("   {}. {} (freq: {}, priority: {}, impact: {:.1})", 
                         i + 1, gap.pattern, gap.frequency, gap.implementation_priority, gap.estimated_impact);
            }
        }
        
        Ok(())
    }
}
