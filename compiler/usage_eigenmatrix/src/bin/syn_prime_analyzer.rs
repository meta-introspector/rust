// syn_prime_analyzer.rs - Parse this program's code and match against prime patterns

use syn::{File, Item, Expr, visit::Visit};
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct SynFingerprint {
    items: HashMap<String, u32>,
    exprs: HashMap<String, u32>,
    types: HashMap<String, u32>,
    patterns: HashMap<String, u32>,
    stmts: HashMap<String, u32>,
}

impl SynFingerprint {
    fn record_item(&mut self, item_type: &str) {
        *self.items.entry(item_type.to_string()).or_insert(0) += 1;
    }
    
    fn record_expr(&mut self, expr_type: &str) {
        *self.exprs.entry(expr_type.to_string()).or_insert(0) += 1;
    }
    
    fn record_type(&mut self, type_name: &str) {
        *self.types.entry(type_name.to_string()).or_insert(0) += 1;
    }
    
    fn record_pattern(&mut self, pat_type: &str) {
        *self.patterns.entry(pat_type.to_string()).or_insert(0) += 1;
    }
    
    fn record_stmt(&mut self, stmt_type: &str) {
        *self.stmts.entry(stmt_type.to_string()).or_insert(0) += 1;
    }
    
    fn print_histogram(&self, file_path: &str) {
        println!("\n📊 SYN FINGERPRINT: {}", file_path);
        println!("═══════════════════════════════════");
        
        println!("📦 Items:");
        for (item_type, count) in &self.items {
            println!("  {}: {} {}", item_type, count, "█".repeat(*count as usize));
        }
        
        println!("\n🔧 Expressions:");
        for (expr_type, count) in &self.exprs {
            println!("  {}: {} {}", expr_type, count, "█".repeat(*count as usize));
        }
        
        println!("\n🏷️ Types:");
        for (type_name, count) in &self.types {
            println!("  {}: {} {}", type_name, count, "█".repeat(*count as usize));
        }
        
        println!("\n🎯 Patterns:");
        for (pat_type, count) in &self.patterns {
            println!("  {}: {} {}", pat_type, count, "█".repeat(*count as usize));
        }
        
        println!("\n📝 Statements:");
        for (stmt_type, count) in &self.stmts {
            println!("  {}: {} {}", stmt_type, count, "█".repeat(*count as usize));
        }
    }
}

struct SynFingerprintVisitor {
    fingerprint: SynFingerprint,
}

impl SynFingerprintVisitor {
    fn new() -> Self {
        Self {
            fingerprint: SynFingerprint::default(),
        }
    }
}

impl<'ast> Visit<'ast> for SynFingerprintVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        let item_type = match item {
            Item::Const(_) => "Const",
            Item::Enum(_) => "Enum", 
            Item::ExternCrate(_) => "ExternCrate",
            Item::Fn(_) => "Fn",
            Item::ForeignMod(_) => "ForeignMod",
            Item::Impl(_) => "Impl",
            Item::Macro(_) => "Macro",
            Item::Mod(_) => "Mod",
            Item::Static(_) => "Static",
            Item::Struct(_) => "Struct",
            Item::Trait(_) => "Trait",
            Item::TraitAlias(_) => "TraitAlias",
            Item::Type(_) => "Type",
            Item::Union(_) => "Union",
            Item::Use(_) => "Use",
            _ => "Other",
        };
        self.fingerprint.record_item(item_type);
        syn::visit::visit_item(self, item);
    }
    
    fn visit_expr(&mut self, expr: &'ast Expr) {
        let expr_type = match expr {
            Expr::Array(_) => "Array",
            Expr::Assign(_) => "Assign",
            Expr::Binary(_) => "Binary",
            Expr::Block(_) => "Block",
            Expr::Call(_) => "Call",
            Expr::Cast(_) => "Cast",
            Expr::Closure(_) => "Closure",
            Expr::Field(_) => "Field",
            Expr::ForLoop(_) => "ForLoop",
            Expr::If(_) => "If",
            Expr::Index(_) => "Index",
            Expr::Let(_) => "Let",
            Expr::Lit(_) => "Lit",
            Expr::Loop(_) => "Loop",
            Expr::Macro(_) => "Macro",
            Expr::Match(_) => "Match",
            Expr::MethodCall(_) => "MethodCall",
            Expr::Path(_) => "Path",
            Expr::Range(_) => "Range",
            Expr::Reference(_) => "Reference",
            Expr::Return(_) => "Return",
            Expr::Struct(_) => "Struct",
            Expr::Try(_) => "Try",
            Expr::Tuple(_) => "Tuple",
            Expr::Unary(_) => "Unary",
            Expr::Unsafe(_) => "Unsafe",
            Expr::While(_) => "While",
            _ => "Other",
        };
        self.fingerprint.record_expr(expr_type);
        syn::visit::visit_expr(self, expr);
    }
    
    fn visit_type(&mut self, ty: &'ast syn::Type) {
        let type_name = match ty {
            syn::Type::Array(_) => "Array",
            syn::Type::BareFn(_) => "BareFn",
            syn::Type::Group(_) => "Group", 
            syn::Type::ImplTrait(_) => "ImplTrait",
            syn::Type::Infer(_) => "Infer",
            syn::Type::Macro(_) => "Macro",
            syn::Type::Never(_) => "Never",
            syn::Type::Paren(_) => "Paren",
            syn::Type::Path(_) => "Path",
            syn::Type::Ptr(_) => "Ptr",
            syn::Type::Reference(_) => "Reference",
            syn::Type::Slice(_) => "Slice",
            syn::Type::TraitObject(_) => "TraitObject",
            syn::Type::Tuple(_) => "Tuple",
            _ => "Other",
        };
        self.fingerprint.record_type(type_name);
        syn::visit::visit_type(self, ty);
    }
    
    fn visit_pat(&mut self, pat: &'ast syn::Pat) {
        let pat_type = match pat {
            syn::Pat::Const(_) => "Const",
            syn::Pat::Ident(_) => "Ident",
            syn::Pat::Lit(_) => "Lit",
            syn::Pat::Macro(_) => "Macro",
            syn::Pat::Or(_) => "Or",
            syn::Pat::Paren(_) => "Paren",
            syn::Pat::Path(_) => "Path",
            syn::Pat::Range(_) => "Range",
            syn::Pat::Reference(_) => "Reference",
            syn::Pat::Rest(_) => "Rest",
            syn::Pat::Slice(_) => "Slice",
            syn::Pat::Struct(_) => "Struct",
            syn::Pat::Tuple(_) => "Tuple",
            syn::Pat::TupleStruct(_) => "TupleStruct",
            syn::Pat::Type(_) => "Type",
            syn::Pat::Wild(_) => "Wild",
            _ => "Other",
        };
        self.fingerprint.record_pattern(pat_type);
        syn::visit::visit_pat(self, pat);
    }
    
    fn visit_stmt(&mut self, stmt: &'ast syn::Stmt) {
        let stmt_type = match stmt {
            syn::Stmt::Local(_) => "Local",
            syn::Stmt::Item(_) => "Item", 
            syn::Stmt::Expr(_, Some(_)) => "ExprSemi",
            syn::Stmt::Expr(_, None) => "Expr",
            syn::Stmt::Macro(_) => "Macro",
        };
        self.fingerprint.record_stmt(stmt_type);
        syn::visit::visit_stmt(self, stmt);
    }
}

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

fn fingerprint_rust_file(file_path: &str) -> Result<SynFingerprint, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let syntax_tree: File = syn::parse_str(&source)?;
    
    let mut visitor = SynFingerprintVisitor::new();
    visitor.visit_file(&syntax_tree);
    
    Ok(visitor.fingerprint)
}

fn extract_prime_literals(file_path: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let syntax_tree: File = syn::parse_str(&source)?;
    
    let mut primes = Vec::new();
    let mut visitor = PrimeLiteralVisitor::new(&mut primes);
    visitor.visit_file(&syntax_tree);
    
    Ok(primes)
}

struct PrimeLiteralVisitor<'a> {
    primes: &'a mut Vec<u32>,
}

impl<'a> PrimeLiteralVisitor<'a> {
    fn new(primes: &'a mut Vec<u32>) -> Self {
        Self { primes }
    }
    
    fn is_prime(n: u32) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        
        let sqrt_n = (n as f64).sqrt() as u32;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
}

impl<'a, 'ast> Visit<'ast> for PrimeLiteralVisitor<'a> {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) = expr {
            if let Ok(value) = lit_int.base10_digits().parse::<u32>() {
                if Self::is_prime(value) {
                    self.primes.push(value);
                }
            }
        }
        syn::visit::visit_expr(self, expr);
    }
}



fn analyze_public_declarations(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let syntax_tree: File = syn::parse_str(&source)?;
    
    println!("\n🔍 PUBLIC DECLARATION ANALYSIS: {}", file_path);
    println!("═══════════════════════════════════════════");
    
    for item in &syntax_tree.items {
        if is_public_item(item) || is_top_level_decl(item) {
            let name = get_item_name(item);
            let complexity = calculate_item_complexity(item);
            let node_histogram = get_node_histogram(item);
            let visibility = if is_public_item(item) { "pub" } else { "private" };
            
            println!("\n📦 {} ({}) [{}]", name, get_item_type(item), visibility);
            println!("  Complexity: {}", complexity);
            println!("  Node histogram:");
            
            let mut sorted_nodes: Vec<_> = node_histogram.iter().collect();
            sorted_nodes.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
            
            for (node_type, count) in sorted_nodes.iter().take(10) {
                println!("    {}: {} {}", node_type, count, "█".repeat(**count as usize));
            }
        }
    }
    
    Ok(())
}

fn is_public_item(item: &Item) -> bool {
    match item {
        Item::Fn(f) => matches!(f.vis, syn::Visibility::Public(_)),
        Item::Struct(s) => matches!(s.vis, syn::Visibility::Public(_)),
        Item::Enum(e) => matches!(e.vis, syn::Visibility::Public(_)),
        Item::Trait(t) => matches!(t.vis, syn::Visibility::Public(_)),
        Item::Const(c) => matches!(c.vis, syn::Visibility::Public(_)),
        Item::Static(s) => matches!(s.vis, syn::Visibility::Public(_)),
        _ => false,
    }
}

fn is_top_level_decl(item: &Item) -> bool {
    matches!(item, 
        Item::Fn(_) | Item::Struct(_) | Item::Enum(_) | 
        Item::Trait(_) | Item::Const(_) | Item::Static(_) |
        Item::Type(_) | Item::Impl(_)
    )
}

fn get_item_name(item: &Item) -> String {
    match item {
        Item::Fn(f) => f.sig.ident.to_string(),
        Item::Struct(s) => s.ident.to_string(),
        Item::Enum(e) => e.ident.to_string(),
        Item::Trait(t) => t.ident.to_string(),
        Item::Const(c) => c.ident.to_string(),
        Item::Static(s) => s.ident.to_string(),
        Item::Type(t) => t.ident.to_string(),
        Item::Impl(_) => "impl_block".to_string(),
        _ => "unknown".to_string(),
    }
}

fn get_item_type(item: &Item) -> &'static str {
    match item {
        Item::Fn(_) => "Function",
        Item::Struct(_) => "Struct",
        Item::Enum(_) => "Enum",
        Item::Trait(_) => "Trait",
        Item::Const(_) => "Const",
        Item::Static(_) => "Static",
        Item::Type(_) => "Type",
        Item::Impl(_) => "Impl",
        _ => "Other",
    }
}

fn calculate_item_complexity(item: &Item) -> u32 {
    let mut complexity = 0;
    let mut visitor = ComplexityVisitor::new(&mut complexity);
    visitor.visit_item(item);
    complexity
}

fn get_node_histogram(item: &Item) -> HashMap<String, u32> {
    let mut histogram = HashMap::new();
    let mut visitor = NodeHistogramVisitor::new(&mut histogram);
    visitor.visit_item(item);
    histogram
}

struct ComplexityVisitor<'a> {
    complexity: &'a mut u32,
}

impl<'a> ComplexityVisitor<'a> {
    fn new(complexity: &'a mut u32) -> Self {
        Self { complexity }
    }
}

impl<'a, 'ast> Visit<'ast> for ComplexityVisitor<'a> {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        *self.complexity += match expr {
            Expr::If(_) | Expr::Match(_) | Expr::ForLoop(_) | Expr::While(_) | Expr::Loop(_) => 2,
            Expr::Binary(_) | Expr::Call(_) | Expr::MethodCall(_) => 1,
            _ => 0,
        };
        syn::visit::visit_expr(self, expr);
    }
}

struct NodeHistogramVisitor<'a> {
    histogram: &'a mut HashMap<String, u32>,
}

impl<'a> NodeHistogramVisitor<'a> {
    fn new(histogram: &'a mut HashMap<String, u32>) -> Self {
        Self { histogram }
    }
    
    fn record(&mut self, node_type: &str) {
        *self.histogram.entry(node_type.to_string()).or_insert(0) += 1;
    }
}

impl<'a, 'ast> Visit<'ast> for NodeHistogramVisitor<'a> {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        let expr_type = match expr {
            Expr::Array(_) => "Array",
            Expr::Binary(_) => "Binary",
            Expr::Call(_) => "Call",
            Expr::Field(_) => "Field",
            Expr::ForLoop(_) => "ForLoop",
            Expr::If(_) => "If",
            Expr::Lit(_) => "Lit",
            Expr::Match(_) => "Match",
            Expr::MethodCall(_) => "MethodCall",
            Expr::Path(_) => "Path",
            _ => "Other",
        };
        self.record(expr_type);
        syn::visit::visit_expr(self, expr);
    }
    
    fn visit_stmt(&mut self, stmt: &'ast syn::Stmt) {
        let stmt_type = match stmt {
            syn::Stmt::Local(_) => "Local",
            syn::Stmt::Item(_) => "Item",
            syn::Stmt::Expr(_, Some(_)) => "ExprSemi",
            syn::Stmt::Expr(_, None) => "Expr",
            syn::Stmt::Macro(_) => "Macro",
        };
        self.record(stmt_type);
        syn::visit::visit_stmt(self, stmt);
    }
    
    fn visit_item(&mut self, item: &'ast Item) {
        let item_type = match item {
            Item::Fn(_) => "Fn",
            Item::Struct(_) => "Struct",
            Item::Enum(_) => "Enum",
            Item::Impl(_) => "Impl",
            _ => "Other",
        };
        self.record(item_type);
        syn::visit::visit_item(self, item);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Syn Prime Analyzer - Multi-File Analysis");
    println!("═══════════════════════════════════════════");
    
    let files_to_analyze = vec![
        "src/lib.rs",
        "src/parse.rs", 
        "src/expr.rs",
        "src/item.rs",
        "src/ty.rs",
        "src/pat.rs",
        "src/stmt.rs",
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
                
                // Generate fingerprint
                if let Ok(fingerprint) = fingerprint_rust_file(file_path) {
                    fingerprint.print_histogram(file_path);
                }
                
                // Analyze public declarations
                let _ = analyze_public_declarations(file_path);
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
    
    // Extract all prime literals from all files
    let mut all_primes = Vec::new();
    println!("\n🔢 PRIME LITERAL EXTRACTION");
    println!("═══════════════════════════");
    
    for file_path in &files_to_analyze {
        if let Ok(primes) = extract_prime_literals(file_path) {
            println!("{}: {} primes found", file_path, primes.len());
            all_primes.extend(primes);
        }
    }
    
    // Create histogram
    let mut prime_counts = HashMap::new();
    for prime in &all_primes {
        *prime_counts.entry(*prime).or_insert(0) += 1;
    }
    
    // Sort by prime value
    let mut sorted_primes: Vec<_> = prime_counts.iter().collect();
    sorted_primes.sort_by_key(|(prime, _)| *prime);
    
    println!("\n📊 PRIME HISTOGRAM (All Code)");
    println!("═══════════════════════════════");
    for (prime, count) in &sorted_primes {
        println!("Prime {:3}: {} {}", prime, count, "█".repeat(**count as usize));
    }
    
    // Save to file
    let prime_data = format!("Total primes found: {}\nUnique primes: {}\n\nHistogram:\n{}", 
        all_primes.len(), 
        prime_counts.len(),
        sorted_primes.iter()
            .map(|(p, c)| format!("{}: {}", p, c))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    fs::write("prime_histogram.txt", prime_data)?;
    println!("\n💾 Saved to prime_histogram.txt");
    
    // Generate complexity bitmap
    generate_complexity_bitmap(&files_to_analyze)?;
    
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
fn analyze_syn_interfaces() -> PrimeScoreVector {
    let mut features = [0.0f32; 8];
    
    // Prime 2: Binary syn types we use
    features[0] += 2.0; // File, Visit
    
    // Prime 3: Three main syn categories  
    features[1] += 3.0; // Item, Expr, Type
    
    // Prime 5: Five operations
    features[2] += 5.0; // parse_str, visit_file, visit_expr, visit_item, match
    
    // Prime 7: Seven expr types we check
    features[3] += 7.0; // If, Binary, Match, ForLoop, Array, MethodCall, Call
    
    // Prime 11: Eleven potential item types
    features[4] += 11.0;
    
    // Prime 13: Thirteen visitor methods available
    features[5] += 13.0;
    
    // Prime 17: Syn crate complexity
    features[6] += 17.0;
    
    // Prime 19: Full syn API surface
    features[7] += 19.0;
    
    PrimeScoreVector::new(features)
}

fn generate_complexity_bitmap(files: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🗺️ COMPLEXITY BITMAP");
    println!("═══════════════════════");
    
    let mut all_complexities = Vec::new();
    
    for file_path in files {
        let source = fs::read_to_string(file_path)?;
        let syntax_tree: File = syn::parse_str(&source)?;
        
        println!("\n📁 {}", file_path);
        let mut file_complexities = Vec::new();
        
        for item in &syntax_tree.items {
            if is_top_level_decl(item) {
                let name = get_item_name(item);
                let complexity = calculate_item_complexity(item);
                file_complexities.push((name, complexity));
                all_complexities.push(complexity);
            }
        }
        
        // Sort by complexity
        file_complexities.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
        
        for (name, complexity) in file_complexities.iter().take(5) {
            let bar = "█".repeat((*complexity as usize / 5).max(1));
            println!("  {:20} {:3} {}", name, complexity, bar);
        }
    }
    
    // Overall complexity distribution
    let max_complexity = *all_complexities.iter().max().unwrap_or(&0);
    let mut buckets = vec![0; (max_complexity / 10 + 1) as usize];
    
    for &complexity in &all_complexities {
        buckets[(complexity / 10) as usize] += 1;
    }
    
    println!("\n📊 COMPLEXITY DISTRIBUTION");
    println!("═══════════════════════════");
    for (i, &count) in buckets.iter().enumerate() {
        if count > 0 {
            let range = format!("{}-{}", i * 10, (i + 1) * 10 - 1);
            let bar = "█".repeat(count);
            println!("  {:6}: {} {}", range, count, bar);
        }
    }
    
    // Save bitmap
    let bitmap_data = format!("Complexity Bitmap\n=================\n\nTotal declarations: {}\nMax complexity: {}\nAverage complexity: {:.1}\n\nDistribution:\n{}", 
        all_complexities.len(),
        max_complexity,
        all_complexities.iter().sum::<u32>() as f32 / all_complexities.len() as f32,
        buckets.iter().enumerate()
            .filter(|(_, &count)| count > 0)
            .map(|(i, count)| format!("{}-{}: {}", i * 10, (i + 1) * 10 - 1, count))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    fs::write("complexity_bitmap.txt", bitmap_data)?;
    println!("\n💾 Saved to complexity_bitmap.txt");
    
    Ok(())
}
