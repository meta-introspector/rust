use std::collections::HashMap;
use std::fs;
use serde_json::Value;

struct FullGraphAdvisor {
    usage_graph: HashMap<String, UsageNode>,
    pattern_correlations: HashMap<String, f64>,
}

#[derive(Debug)]
struct UsageNode {
    frequency: u32,
    contexts: Vec<String>,
    alternatives: Vec<Alternative>,
    optimization_score: f64,
}

#[derive(Debug)]
struct Alternative {
    suggestion: String,
    confidence: f64,
    reason: String,
}

impl FullGraphAdvisor {
    fn new() -> Self {
        let mut advisor = Self {
            usage_graph: HashMap::new(),
            pattern_correlations: HashMap::new(),
        };
        advisor.load_full_usage_graph();
        advisor
    }

    fn load_full_usage_graph(&mut self) {
        // Load all usage data from test_usage_data
        if let Ok(entries) = fs::read_dir("./test_usage_data") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with("_manifest.json") {
                        self.process_manifest(&entry.path());
                    }
                }
            }
        }
        println!("📊 Loaded {} usage patterns into full graph", self.usage_graph.len());
    }

    fn process_manifest(&mut self, path: &std::path::Path) {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(data) = serde_json::from_str::<Value>(&content) {
                if let Some(usages) = data["usages"].as_array() {
                    for usage in usages {
                        self.extract_usage_patterns(usage);
                    }
                }
            }
        }
    }

    fn extract_usage_patterns(&mut self, usage: &Value) {
        if let (Some(pattern), Some(freq)) = (usage["pattern"].as_str(), usage["frequency"].as_u64()) {
            let optimization_score = self.calculate_optimization_score(pattern, freq as u32);
            let alternatives = self.generate_alternatives(pattern);
            
            let node = self.usage_graph.entry(pattern.to_string()).or_insert(UsageNode {
                frequency: 0,
                contexts: Vec::new(),
                alternatives: Vec::new(),
                optimization_score: 0.0,
            });
            node.frequency += freq as u32;
            node.optimization_score = optimization_score;
            node.alternatives = alternatives;
        }
    }

    fn calculate_optimization_score(&self, pattern: &str, frequency: u32) -> f64 {
        let base_score = frequency as f64 * 0.1;
        
        // Boost score for common optimization patterns
        let multiplier = if pattern.contains("clone()") { 2.0 }
        else if pattern.contains("unwrap()") { 1.8 }
        else if pattern.contains("to_string()") { 1.5 }
        else if pattern.contains("collect()") { 1.3 }
        else { 1.0 };
        
        base_score * multiplier
    }

    fn generate_alternatives(&self, pattern: &str) -> Vec<Alternative> {
        let mut alternatives = Vec::new();
        
        // Pattern-based suggestions
        if pattern.contains("clone()") {
            alternatives.push(Alternative {
                suggestion: "Consider using references (&) instead of cloning".to_string(),
                confidence: 0.8,
                reason: "Reduces memory allocation and improves performance".to_string(),
            });
        }
        
        if pattern.contains("unwrap()") {
            alternatives.push(Alternative {
                suggestion: "Use pattern matching or ? operator for error handling".to_string(),
                confidence: 0.9,
                reason: "Prevents panics and improves error handling".to_string(),
            });
        }
        
        if pattern.contains("to_string()") && pattern.contains("&str") {
            alternatives.push(Alternative {
                suggestion: "Use &str directly or String::from() for clarity".to_string(),
                confidence: 0.7,
                reason: "More explicit about string allocation intent".to_string(),
            });
        }
        
        alternatives
    }

    fn analyze_code(&self, code: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        for line in code.lines() {
            for (pattern, node) in &self.usage_graph {
                if line.contains(pattern) && !node.alternatives.is_empty() {
                    for alt in &node.alternatives {
                        suggestions.push(Suggestion {
                            line: line.to_string(),
                            pattern: pattern.clone(),
                            suggestion: alt.suggestion.clone(),
                            confidence: alt.confidence,
                            frequency: node.frequency,
                            optimization_score: node.optimization_score,
                        });
                    }
                }
            }
        }
        
        // Sort by optimization score
        suggestions.sort_by(|a, b| b.optimization_score.partial_cmp(&a.optimization_score).unwrap());
        suggestions
    }
}

#[derive(Debug)]
struct Suggestion {
    line: String,
    pattern: String,
    suggestion: String,
    confidence: f64,
    frequency: u32,
    optimization_score: f64,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        return;
    }

    println!("🌐 Full Graph Code Advisor");
    println!("==========================");
    
    let advisor = FullGraphAdvisor::new();
    
    if let Ok(code) = fs::read_to_string(&args[1]) {
        let suggestions = advisor.analyze_code(&code);
        
        println!("📈 Found {} optimization opportunities:", suggestions.len());
        println!();
        
        for (i, suggestion) in suggestions.iter().take(10).enumerate() {
            println!("{}. 🎯 Pattern: {} (freq: {}, score: {:.1})", 
                i + 1, suggestion.pattern, suggestion.frequency, suggestion.optimization_score);
            println!("   Line: {}", suggestion.line.trim());
            println!("   💡 {}", suggestion.suggestion);
            println!("   🎲 Confidence: {:.0}%", suggestion.confidence * 100.0);
            println!();
        }
        
        if suggestions.len() > 10 {
            println!("... and {} more suggestions", suggestions.len() - 10);
        }
    } else {
        eprintln!("❌ Could not read file: {}", args[1]);
    }
}
