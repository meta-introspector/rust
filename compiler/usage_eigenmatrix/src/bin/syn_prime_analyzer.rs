// syn_prime_analyzer.rs - Parse this program's code and match against prime patterns

use syn::{File, Item, Expr, Stmt, visit::Visit};
use std::fs;

/// The first 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_SIEVE: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

#[derive(Debug, Clone)]
struct PrimeScoreVector {
    scores: [f32; 8],
    confidence: f32,
    total_score: f32,
}

impl PrimeScoreVector {
    fn new(scores: [f32; 8]) -> Self {
        let total_score = scores.iter().sum();
        let confidence = if total_score > 0.0 {
            scores.iter().map(|&s| s * s).sum::<f32>().sqrt() / total_score
        } else {
            0.0
        };
        
        Self { scores, confidence, total_score }
    }
}

struct PrimePatternVisitor {
    features: [f32; 8],
    depth: usize,
}

impl PrimePatternVisitor {
    fn new() -> Self {
        Self {
            features: [0.0; 8],
            depth: 0,
        }
    }
    
    fn enter_scope(&mut self) {
        self.depth += 1;
    }
    
    fn exit_scope(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }
}

impl<'ast> Visit<'ast> for PrimePatternVisitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            // Prime 2: Binary patterns
            Expr::If(_) | Expr::Binary(_) => {
                self.features[0] += 1.0;
            }
            
            // Prime 3: Ternary patterns  
            Expr::Match(match_expr) => {
                if match_expr.arms.len() == 3 {
                    self.features[1] += 2.0;
                } else {
                    self.features[1] += 0.5;
                }
            }
            
            // Prime 5: Pentagonal patterns (loops, 5-element structures)
            Expr::ForLoop(_) | Expr::Loop(_) | Expr::While(_) => {
                self.features[2] += 2.0;
            }
            
            // Prime 7: Septenary patterns (7-deep nesting, weekly cycles)
            _ => {
                if self.depth == 7 {
                    self.features[3] += 1.0;
                }
            }
        }
        
        // Prime 11: Complex expressions (11+ tokens)
        let expr_str = format!("{:?}", expr);
        if expr_str.len() > 110 {
            self.features[4] += 1.0;
        }
        
        // Prime 13: Unlucky patterns (error handling, unwrap)
        if expr_str.contains("unwrap") || expr_str.contains("expect") {
            self.features[5] += 1.0;
        }
        
        // Prime 17: Large structures (17+ elements)
        match expr {
            Expr::Array(arr) if arr.elems.len() >= 17 => {
                self.features[6] += 3.0;
            }
            Expr::Tuple(tup) if tup.elems.len() >= 17 => {
                self.features[6] += 3.0;
            }
            _ => {}
        }
        
        // Prime 19: Mega structures (19+ elements)
        match expr {
            Expr::Array(arr) if arr.elems.len() >= 19 => {
                self.features[7] += 4.0;
            }
            Expr::Tuple(tup) if tup.elems.len() >= 19 => {
                self.features[7] += 4.0;
            }
            _ => {}
        }
        
        self.enter_scope();
        syn::visit::visit_expr(self, expr);
        self.exit_scope();
    }
    
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            // Prime 2: Binary items (enums with 2 variants)
            Item::Enum(enum_item) if enum_item.variants.len() == 2 => {
                self.features[0] += 2.0;
            }
            
            // Prime 3: Ternary items (3 variants, 3 fields)
            Item::Enum(enum_item) if enum_item.variants.len() == 3 => {
                self.features[1] += 3.0;
            }
            Item::Struct(struct_item) => {
                if let syn::Fields::Named(fields) = &struct_item.fields {
                    if fields.named.len() == 3 {
                        self.features[1] += 2.0;
                    }
                }
            }
            
            // Prime 5: Pentagonal items (5 variants/fields)
            Item::Enum(enum_item) if enum_item.variants.len() == 5 => {
                self.features[2] += 5.0;
            }
            
            // Prime 8 primes: Const arrays matching our prime count
            Item::Const(const_item) => {
                let const_str = format!("{:?}", const_item);
                if const_str.contains("[") && const_str.contains("8") {
                    self.features[7] += 2.0; // Meta-pattern: 8-element arrays
                }
            }
            
            _ => {}
        }
        
        syn::visit::visit_item(self, item);
    }
}

fn analyze_rust_file(file_path: &str) -> Result<PrimeScoreVector, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let syntax_tree: File = syn::parse_str(&source)?;
    
    let mut visitor = PrimePatternVisitor::new();
    visitor.visit_file(&syntax_tree);
    
    Ok(PrimeScoreVector::new(visitor.features))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Syn Prime Analyzer - Self-Analysis");
    println!("═══════════════════════════════════════");
    
    // Analyze this very file!
    let current_file = "prime_ast_matcher.rs";
    
    match analyze_rust_file(current_file) {
        Ok(score_vector) => {
            println!("📊 Prime Pattern Analysis for {}:", current_file);
            println!("Total Score: {:.2}", score_vector.total_score);
            println!("Confidence: {:.2}", score_vector.confidence);
            println!();
            
            println!("Prime Resonance Breakdown:");
            for (i, &score) in score_vector.scores.iter().enumerate() {
                let prime = PRIME_SIEVE[i];
                let pattern_name = match i {
                    0 => "Binary (if/else, 2-variants)",
                    1 => "Ternary (3-way, triangular)",
                    2 => "Pentagonal (loops, 5-elements)",
                    3 => "Septenary (7-deep nesting)",
                    4 => "Hendecagonal (complex expressions)",
                    5 => "Tridecagonal (error handling)",
                    6 => "Heptadecagonal (17+ elements)",
                    7 => "Enneadecagonal (19+ elements)",
                    _ => "Unknown",
                };
                
                println!("  Prime {:2} ({}): {:6.1} - {}", 
                         prime, pattern_name, score, "█".repeat((score as usize).min(20)));
            }
            
            // Find dominant patterns
            let max_score = score_vector.scores.iter().fold(0.0f32, |a, &b| a.max(b));
            let dominant_primes: Vec<_> = score_vector.scores.iter().enumerate()
                .filter(|(_, &score)| score > max_score * 0.7)
                .map(|(i, _)| PRIME_SIEVE[i])
                .collect();
            
            println!("\n🎯 Dominant Prime Patterns: {:?}", dominant_primes);
            println!("This code resonates most strongly with primes: {}", 
                     dominant_primes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" + "));
        }
        Err(e) => {
            println!("❌ Error analyzing {}: {}", current_file, e);
            
            // Fallback: analyze our prime sieve demo
            println!("\n🔄 Analyzing prime_sieve_demo.rs instead...");
            match analyze_rust_file("prime_sieve_demo.rs") {
                Ok(score_vector) => {
                    println!("✅ Analysis complete!");
                    println!("Prime signature: {:?}", score_vector.scores);
                }
                Err(e2) => println!("❌ Fallback failed: {}", e2),
            }
        }
    }
    
    Ok(())
}
