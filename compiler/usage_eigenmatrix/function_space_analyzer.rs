// function_space_analyzer.rs - Track function space occupancy only
// Theorem: Programs are equivalent iff they visit the same function space

use std::collections::BTreeSet;
use std::fs;

#[derive(Debug, Clone)]
struct FunctionSpace {
    visited_functions: BTreeSet<String>,
    space_signature: String,
}

#[derive(Debug)]
struct SpaceIndex {
    program_id: String,
    space: FunctionSpace,
}

impl FunctionSpace {
    fn new(functions: BTreeSet<String>) -> Self {
        let signature = functions.iter().cloned().collect::<Vec<_>>().join("|");
        Self {
            visited_functions: functions,
            space_signature: signature,
        }
    }
    
    fn is_equivalent(&self, other: &FunctionSpace) -> bool {
        self.visited_functions == other.visited_functions
    }
    
    fn intersection(&self, other: &FunctionSpace) -> BTreeSet<String> {
        self.visited_functions.intersection(&other.visited_functions).cloned().collect()
    }
    
    fn union(&self, other: &FunctionSpace) -> BTreeSet<String> {
        self.visited_functions.union(&other.visited_functions).cloned().collect()
    }
}

fn main() {
    println!("🔬 Function Space Analyzer - Occupancy-Based Equivalence");
    
    let programs = vec![
        ("const x = 1;", "const_int"),
        ("const y = 1;", "const_int_2"), // Same space
        ("const x = 2;", "const_int_3"), // Same space  
        ("const z = true;", "const_bool"),
        ("let x = 1;", "let_int"),
        ("fn main() {}", "empty_fn"),
    ];
    
    let mut indices = Vec::new();
    
    for (code, id) in programs {
        println!("Analyzing function space for: {} ({})", code, id);
        let index = analyze_function_space(code, id);
        indices.push(index);
    }
    
    // Test space equivalence
    test_space_equivalence(&indices);
    
    // Generate space matrix
    generate_space_matrix(&indices);
}

fn analyze_function_space(code: &str, program_id: &str) -> SpaceIndex {
    let mut visited_functions = BTreeSet::new();
    
    // Analyze what functions this code would visit during compilation/execution
    
    // Lexical analysis functions
    if code.contains("const") {
        visited_functions.insert("lex_const".to_string());
        visited_functions.insert("parse_const_decl".to_string());
    }
    if code.contains("let") {
        visited_functions.insert("lex_let".to_string());
        visited_functions.insert("parse_let_decl".to_string());
    }
    if code.contains("fn") {
        visited_functions.insert("lex_fn".to_string());
        visited_functions.insert("parse_fn_decl".to_string());
    }
    
    // Type analysis functions
    if code.contains("1") || code.contains("2") || code.chars().any(|c| c.is_ascii_digit()) {
        visited_functions.insert("parse_integer".to_string());
        visited_functions.insert("type_check_int".to_string());
        visited_functions.insert("codegen_int".to_string());
    }
    if code.contains("true") || code.contains("false") {
        visited_functions.insert("parse_bool".to_string());
        visited_functions.insert("type_check_bool".to_string());
        visited_functions.insert("codegen_bool".to_string());
    }
    
    // Syntax analysis functions
    if code.contains("=") {
        visited_functions.insert("parse_assignment".to_string());
    }
    if code.contains(";") {
        visited_functions.insert("parse_semicolon".to_string());
    }
    if code.contains("{") || code.contains("}") {
        visited_functions.insert("parse_block".to_string());
    }
    if code.contains("(") || code.contains(")") {
        visited_functions.insert("parse_parens".to_string());
    }
    
    // Identifier analysis
    let identifiers: BTreeSet<char> = code.chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    
    if !identifiers.is_empty() {
        visited_functions.insert("parse_identifier".to_string());
        visited_functions.insert("symbol_table_lookup".to_string());
    }
    
    // Always visit core functions
    visited_functions.insert("tokenize".to_string());
    visited_functions.insert("parse".to_string());
    
    let space = FunctionSpace::new(visited_functions);
    
    SpaceIndex {
        program_id: program_id.to_string(),
        space,
    }
}

fn test_space_equivalence(indices: &[SpaceIndex]) {
    println!("\n🎯 Testing Function Space Equivalence:");
    println!("Theorem: Programs visit same functions ⟺ Programs are equivalent");
    
    for i in 0..indices.len() {
        for j in i+1..indices.len() {
            let equiv = indices[i].space.is_equivalent(&indices[j].space);
            let intersection = indices[i].space.intersection(&indices[j].space);
            let union = indices[i].space.union(&indices[j].space);
            
            println!("  {} ↔ {}: equivalent={}, shared={}, total={}",
                indices[i].program_id, indices[j].program_id, 
                equiv, intersection.len(), union.len());
            
            if equiv {
                println!("    ✅ EQUIVALENT: Same function space signature");
            }
        }
    }
}

fn generate_space_matrix(indices: &[SpaceIndex]) {
    println!("\n📊 Function Space Matrix:");
    
    let mut matrix = String::new();
    matrix.push_str("# Function Space Occupancy Matrix\n\n");
    
    // Function space signatures
    matrix.push_str("## Space Signatures\n\n");
    matrix.push_str("| Program | Functions Visited | Signature |\n");
    matrix.push_str("|---------|-------------------|----------|\n");
    
    for index in indices {
        matrix.push_str(&format!("| {} | {} | {} |\n",
            index.program_id, 
            index.space.visited_functions.len(),
            index.space.space_signature));
    }
    
    // Equivalence matrix
    matrix.push_str("\n## Equivalence Matrix (✅ = Same Space)\n\n");
    matrix.push_str("|");
    for index in indices {
        matrix.push_str(&format!(" {} |", index.program_id));
    }
    matrix.push_str("\n|");
    for _ in indices {
        matrix.push_str("-------|");
    }
    matrix.push_str("\n");
    
    for i in 0..indices.len() {
        matrix.push_str(&format!("| {} |", indices[i].program_id));
        for j in 0..indices.len() {
            let symbol = if i == j { 
                "🔵" 
            } else if indices[i].space.is_equivalent(&indices[j].space) { 
                "✅" 
            } else { 
                "❌" 
            };
            matrix.push_str(&format!(" {} |", symbol));
        }
        matrix.push_str("\n");
    }
    
    // Function space details
    matrix.push_str("\n## Detailed Function Spaces\n\n");
    for index in indices {
        matrix.push_str(&format!("### {} Functions\n", index.program_id));
        for func in &index.space.visited_functions {
            matrix.push_str(&format!("- {}\n", func));
        }
        matrix.push_str("\n");
    }
    
    fs::write("function_space_matrix.md", matrix).expect("Failed to write matrix");
    println!("💾 Function space matrix saved to function_space_matrix.md");
    
    // Summary
    let mut equivalence_classes = Vec::new();
    let mut processed = vec![false; indices.len()];
    
    for i in 0..indices.len() {
        if processed[i] { continue; }
        
        let mut class = vec![indices[i].program_id.clone()];
        processed[i] = true;
        
        for j in i+1..indices.len() {
            if !processed[j] && indices[i].space.is_equivalent(&indices[j].space) {
                class.push(indices[j].program_id.clone());
                processed[j] = true;
            }
        }
        
        equivalence_classes.push(class);
    }
    
    println!("\n🎯 Equivalence Classes Found:");
    for (i, class) in equivalence_classes.iter().enumerate() {
        println!("  Class {}: {:?}", i+1, class);
    }
}
