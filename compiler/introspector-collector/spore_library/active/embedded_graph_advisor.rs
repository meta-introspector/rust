use std::collections::HashMap;
use std::fs;

struct EmbeddedGraphAdvisor {
    patterns: HashMap<&'static str, PatternInfo>,
}

#[derive(Debug)]
struct PatternInfo {
    frequency: u32,
    optimization_score: f64,
    suggestions: Vec<&'static str>,
    confidence: f64,
}

impl EmbeddedGraphAdvisor {
    fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Embed common patterns from our usage analysis
        patterns.insert("clone()", PatternInfo {
            frequency: 1500,
            optimization_score: 300.0,
            suggestions: vec!["Use references (&) instead", "Consider Cow<> for conditional cloning"],
            confidence: 0.85,
        });
        
        patterns.insert("unwrap()", PatternInfo {
            frequency: 800,
            optimization_score: 144.0,
            suggestions: vec!["Use ? operator", "Use pattern matching", "Use unwrap_or_default()"],
            confidence: 0.90,
        });
        
        patterns.insert("to_string()", PatternInfo {
            frequency: 600,
            optimization_score: 90.0,
            suggestions: vec!["Use &str when possible", "Use String::from() for clarity"],
            confidence: 0.70,
        });
        
        patterns.insert("collect()", PatternInfo {
            frequency: 400,
            optimization_score: 52.0,
            suggestions: vec!["Consider iterator chains", "Use for_each() for side effects"],
            confidence: 0.75,
        });
        
        patterns.insert("Vec::new()", PatternInfo {
            frequency: 300,
            optimization_score: 30.0,
            suggestions: vec!["Use Vec::with_capacity() if size known", "Consider smallvec for small collections"],
            confidence: 0.65,
        });
        
        Self { patterns }
    }
    
    fn analyze_code(&self, code: &str) -> Vec<CodeSuggestion> {
        let mut suggestions = Vec::new();
        
        for (line_num, line) in code.lines().enumerate() {
            for (pattern, info) in &self.patterns {
                if line.contains(pattern) {
                    suggestions.push(CodeSuggestion {
                        line_number: line_num + 1,
                        line_content: line.trim().to_string(),
                        pattern: pattern.to_string(),
                        suggestions: info.suggestions.iter().map(|s| s.to_string()).collect(),
                        confidence: info.confidence,
                        frequency: info.frequency,
                        optimization_score: info.optimization_score,
                    });
                }
            }
        }
        
        // Sort by optimization score
        suggestions.sort_by(|a, b| b.optimization_score.partial_cmp(&a.optimization_score).unwrap());
        suggestions
    }
}

#[derive(Debug)]
struct CodeSuggestion {
    line_number: usize,
    line_content: String,
    pattern: String,
    suggestions: Vec<String>,
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

    println!("🧠 Embedded Graph Code Advisor");
    println!("===============================");
    println!("Using embedded usage patterns from compiler analysis");
    
    let advisor = EmbeddedGraphAdvisor::new();
    
    if let Ok(code) = fs::read_to_string(&args[1]) {
        let suggestions = advisor.analyze_code(&code);
        
        println!("📈 Found {} optimization opportunities:", suggestions.len());
        println!();
        
        for (i, suggestion) in suggestions.iter().enumerate() {
            println!("{}. 🎯 Line {}: {}", i + 1, suggestion.line_number, suggestion.pattern);
            println!("   Code: {}", suggestion.line_content);
            println!("   📊 Frequency: {} | Score: {:.1} | Confidence: {:.0}%", 
                suggestion.frequency, suggestion.optimization_score, suggestion.confidence * 100.0);
            
            for (j, sug) in suggestion.suggestions.iter().enumerate() {
                println!("   💡 {}: {}", j + 1, sug);
            }
            println!();
        }
        
        if suggestions.is_empty() {
            println!("✅ No optimization opportunities found - code looks good!");
        }
    } else {
        eprintln!("❌ Could not read file: {}", args[1]);
    }
}
