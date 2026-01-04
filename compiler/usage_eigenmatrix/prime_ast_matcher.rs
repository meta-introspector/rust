// prime_ast_matcher.rs - Match AST structures against prime sieve patterns

/// The first 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_SIEVE: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

#[derive(Debug, Clone)]
struct PrimeSieveStructure {
    data: [bool; 8],
    encoding: u8,
}

impl PrimeSieveStructure {
    fn from_encoding(encoding: u8) -> Self {
        let mut data = [false; 8];
        for i in 0..8 {
            data[i] = (encoding & (1 << i)) != 0;
        }
        Self { data, encoding }
    }
    
    fn active_primes(&self) -> Vec<u32> {
        self.data.iter().enumerate()
            .filter_map(|(i, &active)| if active { Some(PRIME_SIEVE[i]) } else { None })
            .collect()
    }
}

#[derive(Debug)]
struct AstNode {
    node_type: String,
    children: Vec<AstNode>,
    depth: usize,
}

impl AstNode {
    fn new(node_type: &str) -> Self {
        Self {
            node_type: node_type.to_string(),
            children: Vec::new(),
            depth: 0,
        }
    }
    
    fn add_child(&mut self, child: AstNode) {
        self.children.push(child);
    }
    
    fn calculate_depth(&mut self) {
        for child in &mut self.children {
            child.calculate_depth();
            child.depth = self.depth + 1;
        }
    }
}

struct PrimeAstMatcher;

impl PrimeAstMatcher {
    /// Extract prime-resonant features from AST
    fn extract_features(ast: &AstNode) -> [f32; 8] {
        let mut features = [0.0; 8];
        
        // Feature 0 (prime 2): Binary structures (if/else, match arms)
        features[0] = Self::count_binary_structures(ast) as f32;
        
        // Feature 1 (prime 3): Ternary patterns (3-way branches, triangular structures)
        features[1] = Self::count_ternary_patterns(ast) as f32;
        
        // Feature 2 (prime 5): Pentagonal patterns (5-element collections, loops)
        features[2] = Self::count_pentagonal_patterns(ast) as f32;
        
        // Feature 3 (prime 7): Septenary patterns (7-element structures, weekly cycles)
        features[3] = Self::count_septenary_patterns(ast) as f32;
        
        // Feature 4 (prime 11): Hendecagonal patterns (11-element structures)
        features[4] = Self::count_hendecagonal_patterns(ast) as f32;
        
        // Feature 5 (prime 13): Tridecagonal patterns (13-element structures)
        features[5] = Self::count_tridecagonal_patterns(ast) as f32;
        
        // Feature 6 (prime 17): Heptadecagonal patterns (17-element structures)
        features[6] = Self::count_heptadecagonal_patterns(ast) as f32;
        
        // Feature 7 (prime 19): Enneadecagonal patterns (19-element structures)
        features[7] = Self::count_enneadecagonal_patterns(ast) as f32;
        
        features
    }
    
    fn count_binary_structures(ast: &AstNode) -> u32 {
        let mut count = 0;
        if ast.node_type.contains("if") || ast.node_type.contains("match") {
            count += 1;
        }
        if ast.children.len() == 2 { count += 1; }
        for child in &ast.children {
            count += Self::count_binary_structures(child);
        }
        count
    }
    
    fn count_ternary_patterns(ast: &AstNode) -> u32 {
        let mut count = 0;
        if ast.children.len() == 3 { count += 1; }
        if ast.depth % 3 == 0 { count += 1; }
        for child in &ast.children {
            count += Self::count_ternary_patterns(child);
        }
        count
    }
    
    fn count_pentagonal_patterns(ast: &AstNode) -> u32 {
        let mut count = 0;
        if ast.children.len() == 5 { count += 2; }
        if ast.node_type.contains("for") || ast.node_type.contains("loop") { count += 1; }
        for child in &ast.children {
            count += Self::count_pentagonal_patterns(child);
        }
        count
    }
    
    fn count_septenary_patterns(ast: &AstNode) -> u32 {
        let mut count = 0;
        if ast.children.len() == 7 { count += 2; }
        if ast.depth == 7 { count += 1; }
        for child in &ast.children {
            count += Self::count_septenary_patterns(child);
        }
        count
    }
    
    fn count_hendecagonal_patterns(ast: &AstNode) -> u32 {
        let mut count = 0;
        if ast.children.len() == 11 { count += 3; }
        for child in &ast.children {
            count += Self::count_hendecagonal_patterns(child);
        }
        count
    }
    
    fn count_tridecagonal_patterns(ast: &AstNode) -> u32 {
        let mut count = 0;
        if ast.children.len() == 13 { count += 3; }
        for child in &ast.children {
            count += Self::count_tridecagonal_patterns(child);
        }
        count
    }
    
    fn count_heptadecagonal_patterns(ast: &AstNode) -> u32 {
        let mut count = 0;
        if ast.children.len() == 17 { count += 4; }
        for child in &ast.children {
            count += Self::count_heptadecagonal_patterns(child);
        }
        count
    }
    
    fn count_enneadecagonal_patterns(ast: &AstNode) -> u32 {
        let mut count = 0;
        if ast.children.len() == 19 { count += 4; }
        for child in &ast.children {
            count += Self::count_enneadecagonal_patterns(child);
        }
        count
    }
    
    /// Calculate resonance score between AST and prime sieve structure
    fn calculate_resonance(ast_features: &[f32; 8], sieve: &PrimeSieveStructure) -> f32 {
        let mut score = 0.0;
        
        for i in 0..8 {
            if sieve.data[i] {
                // Prime is active - boost score by feature strength
                score += ast_features[i] * PRIME_SIEVE[i] as f32;
            } else {
                // Prime is inactive - penalize if feature is present
                score -= ast_features[i] * 0.1;
            }
        }
        
        score.max(0.0) // No negative scores
    }
    
    /// Match AST against all 256 prime sieve structures
    pub fn match_ast(ast: &AstNode) -> Vec<(u8, f32)> {
        let features = Self::extract_features(ast);
        let mut results = Vec::new();
        
        for encoding in 0..=255u8 {
            let sieve = PrimeSieveStructure::from_encoding(encoding);
            let score = Self::calculate_resonance(&features, &sieve);
            results.push((encoding, score));
        }
        
        // Sort by score descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results
    }
}

fn main() {
    println!("🎯 Prime AST Matcher - Resonance Analysis");
    println!("═══════════════════════════════════════");
    
    // Create a sample AST
    let mut ast = AstNode::new("function");
    
    let mut if_node = AstNode::new("if_statement");
    if_node.add_child(AstNode::new("condition"));
    if_node.add_child(AstNode::new("then_branch"));
    
    let mut for_loop = AstNode::new("for_loop");
    for i in 0..5 {
        for_loop.add_child(AstNode::new(&format!("item_{}", i)));
    }
    
    ast.add_child(if_node);
    ast.add_child(for_loop);
    ast.calculate_depth();
    
    println!("Sample AST: function with if-statement and 5-item for-loop");
    
    // Match against all prime structures
    let matches = PrimeAstMatcher::match_ast(&ast);
    
    println!("\n🏆 Top 10 Prime Resonances:");
    println!("Rank | Encoding | Binary   | Active Primes | Score");
    println!("-----|----------|----------|---------------|-------");
    
    for (rank, (encoding, score)) in matches.iter().take(10).enumerate() {
        let sieve = PrimeSieveStructure::from_encoding(*encoding);
        let binary = format!("{:08b}", encoding);
        let primes = sieve.active_primes();
        let primes_str = if primes.is_empty() {
            "∅".to_string()
        } else {
            primes.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(",")
        };
        
        println!("{:4} | {:8} | {} | {:13} | {:6.1}", 
                 rank + 1, encoding, binary, primes_str, score);
    }
    
    println!("\n🔍 Analysis:");
    let top_match = &matches[0];
    let top_sieve = PrimeSieveStructure::from_encoding(top_match.0);
    println!("Best match: Encoding {} with primes {:?}", 
             top_match.0, top_sieve.active_primes());
    println!("This AST resonates most with prime pattern: {}", 
             top_sieve.active_primes().iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" + "));
}
