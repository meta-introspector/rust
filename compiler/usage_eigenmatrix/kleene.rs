// kleene.rs - Lattice of Regex Features (Kleene Lattice)
// Each regex forms its own language in the lattice hierarchy

use std::collections::{HashMap, BTreeSet, BTreeMap};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct KleeneNode {
    level: usize,
    features: BTreeSet<String>,
    regex_language: String,
    examples: Vec<String>,
}

#[derive(Debug)]
struct KleeneLattice {
    nodes: BTreeMap<usize, Vec<KleeneNode>>,
    feature_hierarchy: HashMap<String, usize>,
    language_relations: Vec<(String, String)>, // (subset, superset)
}

impl KleeneLattice {
    fn new() -> Self {
        let mut lattice = Self {
            nodes: BTreeMap::new(),
            feature_hierarchy: HashMap::new(),
            language_relations: Vec::new(),
        };
        
        lattice.build_feature_hierarchy();
        lattice.generate_lattice_levels();
        lattice.compute_language_relations();
        
        lattice
    }
    
    fn build_feature_hierarchy(&mut self) {
        // Level 0: Empty language
        self.feature_hierarchy.insert("empty".to_string(), 0);
        
        // Level 1: Basic atoms
        self.feature_hierarchy.insert("literal".to_string(), 1);
        self.feature_hierarchy.insert("dot".to_string(), 1);
        
        // Level 2: Character classes
        self.feature_hierarchy.insert("char_class".to_string(), 2);
        self.feature_hierarchy.insert("digit".to_string(), 2);
        self.feature_hierarchy.insert("word".to_string(), 2);
        
        // Level 3: Quantifiers
        self.feature_hierarchy.insert("optional".to_string(), 3);
        self.feature_hierarchy.insert("star".to_string(), 3);
        self.feature_hierarchy.insert("plus".to_string(), 3);
        
        // Level 4: Anchors
        self.feature_hierarchy.insert("start_anchor".to_string(), 4);
        self.feature_hierarchy.insert("end_anchor".to_string(), 4);
        
        // Level 5: Groups and alternation
        self.feature_hierarchy.insert("group".to_string(), 5);
        self.feature_hierarchy.insert("alternation".to_string(), 5);
        
        // Level 6: Advanced features
        self.feature_hierarchy.insert("backreference".to_string(), 6);
        self.feature_hierarchy.insert("lookahead".to_string(), 6);
    }
    
    fn generate_lattice_levels(&mut self) {
        // Level 0: Empty language ∅
        self.add_node(0, vec!["empty"], "∅", vec!["(no matches)"]);
        
        // Level 1: Literal characters
        self.add_node(1, vec!["literal"], "L", vec!["a", "hello", "123"]);
        self.add_node(1, vec!["dot"], ".", vec![".", "any single char"]);
        
        // Level 2: Character classes
        self.add_node(2, vec!["literal", "char_class"], "[L]", vec!["[abc]", "[0-9]", "[a-z]"]);
        self.add_node(2, vec!["literal", "digit"], "\\d", vec!["\\d", "0", "9"]);
        self.add_node(2, vec!["literal", "word"], "\\w", vec!["\\w", "a", "Z", "_"]);
        
        // Level 3: Quantified expressions
        self.add_node(3, vec!["literal", "optional"], "L?", vec!["a?", "colou?r"]);
        self.add_node(3, vec!["literal", "star"], "L*", vec!["a*", ".*", "\\w*"]);
        self.add_node(3, vec!["literal", "plus"], "L+", vec!["a+", ".+", "\\w+"]);
        self.add_node(3, vec!["char_class", "star"], "[L]*", vec!["[abc]*", "\\d*"]);
        self.add_node(3, vec!["word", "plus"], "\\w+", vec!["\\w+", "hello", "var123"]);
        
        // Level 4: Anchored expressions
        self.add_node(4, vec!["literal", "start_anchor"], "^L", vec!["^hello", "^\\w+"]);
        self.add_node(4, vec!["literal", "end_anchor"], "L$", vec!["world$", "\\d+$"]);
        self.add_node(4, vec!["literal", "start_anchor", "end_anchor"], "^L$", vec!["^hello$", "^\\w+$"]);
        
        // Level 5: Groups and alternation
        self.add_node(5, vec!["literal", "group"], "(L)", vec!["(abc)", "(\\w+)"]);
        self.add_node(5, vec!["literal", "alternation"], "L|L", vec!["a|b", "cat|dog", "\\d+|\\w+"]);
        self.add_node(5, vec!["group", "alternation"], "(L|L)", vec!["(a|b)", "(cat|dog)+"]);
        
        // Level 6: Advanced regex languages
        self.add_node(6, vec!["group", "alternation", "star"], "(L|L)*", vec!["(a|b)*", "(\\w+|\\d+)*"]);
        self.add_node(6, vec!["literal", "backreference"], "L\\1", vec!["(\\w+)\\1", "([a-z])\\1"]);
        self.add_node(6, vec!["literal", "lookahead"], "L(?=L)", vec!["\\w+(?=@)", "test(?=ing)"]);
    }
    
    fn add_node(&mut self, level: usize, features: Vec<&str>, language: &str, examples: Vec<&str>) {
        let feature_set: BTreeSet<String> = features.iter().map(|s| s.to_string()).collect();
        let example_vec: Vec<String> = examples.iter().map(|s| s.to_string()).collect();
        
        let node = KleeneNode {
            level,
            features: feature_set,
            regex_language: language.to_string(),
            examples: example_vec,
        };
        
        self.nodes.entry(level).or_insert_with(Vec::new).push(node);
    }
    
    fn compute_language_relations(&mut self) {
        // Compute subset relations between regex languages
        for level in 0..=6 {
            if let Some(nodes) = self.nodes.get(&level) {
                for node in nodes {
                    // Find supersets in higher levels
                    for higher_level in (level+1)..=6 {
                        if let Some(higher_nodes) = self.nodes.get(&higher_level) {
                            for higher_node in higher_nodes {
                                if node.features.is_subset(&higher_node.features) {
                                    self.language_relations.push((
                                        node.regex_language.clone(),
                                        higher_node.regex_language.clone()
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn generate_lattice_diagram(&self) -> String {
        let mut diagram = String::new();
        diagram.push_str("# Kleene Lattice - Regex Feature Hierarchy\n\n");
        diagram.push_str("```\n");
        diagram.push_str("Level 6: Advanced     (L|L)*  L\\1  L(?=L)\n");
        diagram.push_str("         │              │      │      │\n");
        diagram.push_str("Level 5: Groups       (L)    L|L   (L|L)\n");
        diagram.push_str("         │              │      │      │\n");
        diagram.push_str("Level 4: Anchors      ^L     L$    ^L$\n");
        diagram.push_str("         │              │      │      │\n");
        diagram.push_str("Level 3: Quantifiers  L?     L*     L+\n");
        diagram.push_str("         │              │      │      │\n");
        diagram.push_str("Level 2: Classes      [L]    \\d     \\w\n");
        diagram.push_str("         │              │      │      │\n");
        diagram.push_str("Level 1: Atoms         L      .      \n");
        diagram.push_str("         │              │      │      \n");
        diagram.push_str("Level 0: Empty         ∅             \n");
        diagram.push_str("```\n\n");
        
        diagram.push_str("## Language Hierarchy\n\n");
        for level in 0..=6 {
            if let Some(nodes) = self.nodes.get(&level) {
                diagram.push_str(&format!("### Level {}\n\n", level));
                diagram.push_str("| Language | Features | Examples |\n");
                diagram.push_str("|----------|----------|----------|\n");
                
                for node in nodes {
                    let features_str = node.features.iter().cloned().collect::<Vec<_>>().join(", ");
                    let examples_str = node.examples.join(", ");
                    diagram.push_str(&format!("| {} | {} | {} |\n", 
                        node.regex_language, features_str, examples_str));
                }
                diagram.push_str("\n");
            }
        }
        
        diagram
    }
}

fn main() {
    println!("🔬 Kleene Lattice - Regex Feature Hierarchy");
    
    let kleene = KleeneLattice::new();
    
    // Display lattice structure
    println!("📊 Kleene Lattice Levels:");
    for level in 0..=6 {
        if let Some(nodes) = kleene.nodes.get(&level) {
            println!("  Level {}: {} regex languages", level, nodes.len());
            for node in nodes {
                println!("    {} → {:?}", node.regex_language, node.examples);
            }
        }
    }
    
    // Show language relations
    println!("\n🔗 Language Subset Relations:");
    for (subset, superset) in &kleene.language_relations {
        println!("  {} ⊆ {}", subset, superset);
    }
    
    // Generate and save lattice diagram
    let diagram = kleene.generate_lattice_diagram();
    fs::write("kleene_lattice.md", diagram).expect("Failed to write lattice");
    
    // Test regex language membership
    test_regex_membership(&kleene);
    
    // Generate kleene algebra operations
    generate_kleene_algebra(&kleene);
    
    println!("\n💾 Kleene lattice saved to kleene_lattice.md");
}

fn test_regex_membership(kleene: &KleeneLattice) {
    println!("\n🎯 Testing Regex Language Membership:");
    
    let test_regexes = vec![
        ("a", 1),           // Literal
        ("\\w+", 3),        // Word class + plus
        ("^hello$", 4),     // Anchored literal
        ("(a|b)*", 6),      // Alternation + star
        ("\\d+@\\w+", 5),   // Complex pattern
    ];
    
    for (regex, expected_level) in test_regexes {
        let actual_level = classify_regex_level(regex, kleene);
        println!("  {} → Level {} (expected {})", regex, actual_level, expected_level);
    }
}

fn classify_regex_level(regex: &str, kleene: &KleeneLattice) -> usize {
    let mut max_level = 0;
    
    // Simple classification based on features present
    if regex.contains("(?=") || regex.contains("\\1") { max_level = 6; }
    else if regex.contains("|") || regex.contains("(") { max_level = 5; }
    else if regex.contains("^") || regex.contains("$") { max_level = 4; }
    else if regex.contains("*") || regex.contains("+") || regex.contains("?") { max_level = 3; }
    else if regex.contains("\\w") || regex.contains("\\d") || regex.contains("[") { max_level = 2; }
    else if regex.contains(".") { max_level = 1; }
    else if !regex.is_empty() { max_level = 1; }
    
    max_level
}

fn generate_kleene_algebra(kleene: &KleeneLattice) {
    let mut algebra = String::new();
    algebra.push_str("# Kleene Algebra Operations\n\n");
    
    algebra.push_str("## Lattice Operations\n\n");
    algebra.push_str("- **Join (∨)**: L₁ ∨ L₂ = L₁ ∪ L₂ (union of languages)\n");
    algebra.push_str("- **Meet (∧)**: L₁ ∧ L₂ = L₁ ∩ L₂ (intersection of languages)\n");
    algebra.push_str("- **Complement**: ¬L = Σ* \\ L (complement language)\n");
    algebra.push_str("- **Kleene Star**: L* = ε ∪ L ∪ L² ∪ L³ ∪ ...\n\n");
    
    algebra.push_str("## Lattice Properties\n\n");
    algebra.push_str("1. **Partial Order**: L₁ ⊆ L₂ iff L₁ ∨ L₂ = L₂\n");
    algebra.push_str("2. **Bottom Element**: ∅ (empty language)\n");
    algebra.push_str("3. **Top Element**: Σ* (all strings)\n");
    algebra.push_str("4. **Distributivity**: L₁ ∧ (L₂ ∨ L₃) = (L₁ ∧ L₂) ∨ (L₁ ∧ L₃)\n\n");
    
    algebra.push_str("## Feature Composition Rules\n\n");
    for level in 1..=6 {
        if let Some(nodes) = kleene.nodes.get(&level) {
            algebra.push_str(&format!("**Level {}**: ", level));
            let languages: Vec<String> = nodes.iter().map(|n| n.regex_language.clone()).collect();
            algebra.push_str(&format!("{}\n", languages.join(", ")));
        }
    }
    
    fs::write("kleene_algebra.md", algebra).expect("Failed to write algebra");
    println!("📐 Kleene algebra operations saved to kleene_algebra.md");
}
