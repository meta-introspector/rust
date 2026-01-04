use std::collections::HashMap;

#[derive(Debug, Clone)]
struct RustAstClassifier {
    monster_matcher: MonsterPatternMatcher,
    ast_patterns: HashMap<String, u64>,
}

impl RustAstClassifier {
    fn new() -> Self {
        let mut classifier = Self {
            monster_matcher: MonsterPatternMatcher::new(),
            ast_patterns: HashMap::new(),
        };
        classifier.generate_ast_patterns();
        classifier
    }
    
    fn generate_ast_patterns(&mut self) {
        // Map Rust AST nodes to Monster bit patterns
        let ast_mappings = [
            ("Literal", 0x1),           // bool-like → 2^1
            ("Ident", 0x7),             // Option-like → 3^1  
            ("BinOp", 0x1FF),           // Complex → 5^9
            ("UnaryOp", 0x3F << 9),     // 7^6 pattern
            ("FnCall", 0x3 << 15),      // 11^2 pattern
            ("Block", 0x7 << 17),       // 13^3 pattern
            ("If", 0x1 << 20),          // 17^1 pattern
            ("Loop", 0x1 << 21),        // 19^1 pattern
            ("Match", 0x3FF),           // Complex match
            ("Struct", 0x7FF),          // Struct pattern
            ("Enum", 0xFFF),            // Enum pattern
            ("Trait", 0x1FFF),          // Trait pattern
            ("Impl", 0x3FFF),           // Impl pattern
            ("Mod", 0x7FFF),            // Module pattern
            ("Use", 0xFFFF),            // Use pattern
        ];
        
        for &(ast_name, pattern) in &ast_mappings {
            self.ast_patterns.insert(ast_name.to_string(), pattern);
        }
    }
    
    fn classify_ast(&self, ast_node: &str, ast_data: u64) -> Option<(u16, f64)> {
        // Get AST's base pattern
        let base_pattern = self.ast_patterns.get(ast_node)?;
        
        // Combine with actual data
        let combined_pattern = base_pattern ^ ast_data;
        
        // Find best matching lattice point
        let type_idx = self.monster_matcher.match_pattern(combined_pattern)?;
        
        // Calculate resonance score
        let lattice_pattern = self.monster_matcher.get_type_pattern(type_idx);
        let resonance = calculate_resonance(combined_pattern, lattice_pattern);
        
        Some((type_idx, resonance))
    }
}

fn calculate_resonance(pattern1: u64, pattern2: u64) -> f64 {
    let intersection = (pattern1 & pattern2).count_ones();
    let union = (pattern1 | pattern2).count_ones();
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

fn main() {
    println!("🌳 RUST AST → MONSTER LATTICE CLASSIFIER");
    
    let classifier = RustAstClassifier::new();
    
    println!("\n📊 AST Classification Tests:");
    let test_asts = [
        ("Literal", 0x42),
        ("BinOp", 0x123),
        ("FnCall", 0x456),
        ("Match", 0x789),
        ("Struct", 0xABC),
    ];
    
    for &(ast_name, data) in &test_asts {
        if let Some((type_idx, resonance)) = classifier.classify_ast(ast_name, data) {
            println!("   {} + 0x{:x} → Type[{:04x}] (resonance: {:.3})", 
                     ast_name, data, type_idx, resonance);
        }
    }
    
    println!("\n✅ All Rust ASTs classified by Monster lattice resonance!");
}

// Include the MonsterPatternMatcher from previous code
#[derive(Debug, Clone)]
struct MonsterPatternMatcher {
    type_table: [u64; 65536],
    pattern_map: HashMap<u64, u16>,
}

impl MonsterPatternMatcher {
    fn new() -> Self {
        let mut matcher = Self {
            type_table: [0; 65536],
            pattern_map: HashMap::new(),
        };
        matcher.generate_table();
        matcher
    }
    
    fn generate_table(&mut self) {
        let patterns = [
            (0, 0x0), (1, 0x1), (2, 0x7), (3, 0x1FF),
            (4, 0x3F << 9), (5, 0x3 << 15), (6, 0x7 << 17),
            (7, 0x1 << 20), (8, 0x1 << 21),
        ];
        
        for &(type_idx, monster_bits) in &patterns {
            self.type_table[type_idx] = monster_bits;
            self.pattern_map.insert(monster_bits, type_idx as u16);
        }
    }
    
    fn match_pattern(&self, data: u64) -> Option<u16> {
        let mut best_match = None;
        let mut best_score = 0;
        
        for (&pattern, &type_idx) in &self.pattern_map {
            let score = (data & pattern).count_ones();
            if score > best_score {
                best_score = score;
                best_match = Some(type_idx);
            }
        }
        
        best_match
    }
    
    fn get_type_pattern(&self, type_idx: u16) -> u64 {
        self.type_table[type_idx as usize]
    }
}
