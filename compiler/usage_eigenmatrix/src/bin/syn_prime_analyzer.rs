// syn_prime_analyzer.rs - Parse this program's code and match against prime patterns

use syn::{File, Item, Expr, visit::Visit};
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
        let expr_complexity = match expr {
            Expr::Call(call) => call.args.len(),
            Expr::MethodCall(method) => method.args.len(),
            Expr::Array(arr) => arr.elems.len(),
            Expr::Tuple(tup) => tup.elems.len(),
            _ => 0,
        };
        if expr_complexity > 11 {
            self.features[4] += 1.0;
        }
        
        // Prime 13: Unlucky patterns (error handling)
        match expr {
            Expr::MethodCall(method) => {
                if method.method == "unwrap" || method.method == "expect" {
                    self.features[5] += 1.0;
                }
            }
            _ => {}
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
            
            // Prime 8 primes: Const arrays (simplified detection)
            Item::Const(_) => {
                self.features[7] += 1.0; // Any const gets some score
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

fn analyze_syn_interfaces() -> PrimeScoreVector {
    let mut features = [0.0f32; 8];
    
    println!("\n🔬 Analyzing Syn Crate Interfaces Used");
    println!("═════════════════════════════════════");
    
    // Prime 2: Binary syn types we use
    features[0] += 2.0; // File, Visit
    println!("Prime 2: Core types (File, Visit)");
    
    // Prime 3: Three main syn categories  
    features[1] += 3.0; // Item, Expr, Type
    println!("Prime 3: Categories (Item, Expr, Type)");
    
    // Prime 5: Five operations
    features[2] += 5.0; // parse_str, visit_file, visit_expr, visit_item, match
    println!("Prime 5: Operations (parse, visit, match)");
    
    // Prime 7: Seven expr types we check
    features[3] += 7.0; // If, Binary, Match, ForLoop, Array, MethodCall, Call
    println!("Prime 7: Expression types analyzed");
    
    // Prime 11: Eleven potential item types
    features[4] += 11.0;
    println!("Prime 11: Item types available");
    
    // Prime 13: Thirteen visitor methods available
    features[5] += 13.0;
    println!("Prime 13: Visitor methods");
    
    // Prime 17: Syn crate complexity
    features[6] += 17.0;
    println!("Prime 17: Crate complexity");
    
    // Prime 19: Full syn API surface
    features[7] += 19.0;
    println!("Prime 19: Full API surface");
    
    PrimeScoreVector::new(features)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Syn Prime Analyzer - Multi-File Analysis");
    println!("═══════════════════════════════════════════");
    
    let files_to_analyze = vec![
        "observe_bits.rs",
        "src/bin/prime_sieve_table.rs", 
        "src/bin/clean_graph_interpreter.rs",
        "src/bin/syn_prime_analyzer.rs",
        "src/bin/uncontainable_meme.rs",
        "src/bin/flexible_scp_system.rs",
        "src/bin/recursive_ast_classifier.rs",
    ];
    
    let mut total_scores = [0.0f32; 8];
    let mut analyzed_count = 0;
    
    for file_path in &files_to_analyze {
        println!("\n📁 Analyzing: {}", file_path);
        match analyze_rust_file(file_path) {
            Ok(score_vector) => {
                analyzed_count += 1;
                println!("  Total Score: {:.1}, Confidence: {:.2}", 
                         score_vector.total_score, score_vector.confidence);
                
                // Show top 3 patterns for this file
                let mut indexed_scores: Vec<_> = score_vector.scores.iter().enumerate().collect();
                indexed_scores.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
                
                print!("  Top patterns: ");
                for (i, (idx, &score)) in indexed_scores.iter().take(3).enumerate() {
                    if i > 0 { print!(", "); }
                    print!("{}({:.1})", PRIME_SIEVE[*idx], score);
                }
                println!();
                
                // Add to totals
                for (i, &score) in score_vector.scores.iter().enumerate() {
                    total_scores[i] += score;
                }
            }
            Err(e) => {
                println!("  ❌ Error: {}", e);
            }
        }
    }
    
    if analyzed_count > 0 {
        println!("\n🎯 AGGREGATE ANALYSIS ({} files)", analyzed_count);
        println!("═══════════════════════════════════");
        
        for (i, &total_score) in total_scores.iter().enumerate() {
            let avg_score = total_score / analyzed_count as f32;
            let prime = PRIME_SIEVE[i];
            let pattern_name = match i {
                0 => "Binary",
                1 => "Ternary", 
                2 => "Pentagonal",
                3 => "Septenary",
                4 => "Hendecagonal",
                5 => "Tridecagonal", 
                6 => "Heptadecagonal",
                7 => "Enneadecagonal",
                _ => "Unknown",
            };
            
            println!("Prime {:2} ({}): {:6.1} avg - {}", 
                     prime, pattern_name, avg_score, "█".repeat((avg_score as usize).min(20)));
        }
        
        let max_avg = total_scores.iter().map(|&s| s / analyzed_count as f32).fold(0.0f32, |a, b| a.max(b));
        let dominant_primes: Vec<_> = total_scores.iter().enumerate()
            .map(|(i, &score)| (i, score / analyzed_count as f32))
            .filter(|(_, avg)| *avg > max_avg * 0.6)
            .map(|(i, _)| PRIME_SIEVE[i])
            .collect();
        
        println!("\n🌟 Codebase Prime Signature: {:?}", dominant_primes);
    }
    
    // Analyze the syn interfaces themselves
    let syn_analysis = analyze_syn_interfaces();
    println!("\n🔬 SYN INTERFACE ANALYSIS");
    println!("═══════════════════════════");
    println!("Syn API Prime Signature: {:.1} total score", syn_analysis.total_score);
    
    for (i, &score) in syn_analysis.scores.iter().enumerate() {
        let prime = PRIME_SIEVE[i];
        let pattern = match i {
            0 => "Core types (File, Visit)",
            1 => "Categories (Item, Expr, Type)", 
            2 => "Operations (parse, visit, match)",
            3 => "Expression types analyzed",
            4 => "Item types available",
            5 => "Visitor methods",
            6 => "Crate complexity",
            7 => "Full API surface",
            _ => "Unknown",
        };
        println!("Prime {:2}: {:4.0} - {}", prime, score, pattern);
    }
    
    Ok(())
}
