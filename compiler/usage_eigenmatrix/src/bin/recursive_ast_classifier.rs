use std::collections::{HashMap, BTreeMap};

#[derive(Debug, Clone)]
struct ComplexityClass {
    monster_index: u16,
    ast_nodes: Vec<AstNode>,
    refinement_level: u8,
    sub_classes: BTreeMap<u16, ComplexityClass>,
}

#[derive(Debug, Clone)]
struct AstNode {
    name: String,
    data: u64,
    size: usize,
    monster_pattern: u64,
}

#[derive(Debug)]
struct RecursiveAstClassifier {
    base_classifier: RustAstClassifier,
    complexity_classes: BTreeMap<u16, ComplexityClass>,
}

impl RecursiveAstClassifier {
    fn new() -> Self {
        Self {
            base_classifier: RustAstClassifier::new(),
            complexity_classes: BTreeMap::new(),
        }
    }
    
    fn classify_and_cluster(&mut self, ast_nodes: Vec<AstNode>) {
        println!("🔄 RECURSIVE AST CLUSTERING BY COMPLEXITY");
        
        // Step 1: Initial Monster index classification
        for node in ast_nodes {
            if let Some((monster_index, _)) = self.base_classifier.classify_ast(&node.name, node.data) {
                self.add_to_complexity_class(monster_index, node, 0);
            }
        }
        
        // Step 2: Recursive refinement within each class
        let mut indices_to_refine = Vec::new();
        for (index, class) in &self.complexity_classes {
            if class.ast_nodes.len() > 1 {
                indices_to_refine.push(*index);
            }
        }
        
        for index in indices_to_refine {
            if let Some(class) = self.complexity_classes.get_mut(&index) {
                self.refine_complexity_class(class);
            }
        }
    }
    
    fn add_to_complexity_class(&mut self, monster_index: u16, node: AstNode, level: u8) {
        let class = self.complexity_classes.entry(monster_index).or_insert(ComplexityClass {
            monster_index,
            ast_nodes: Vec::new(),
            refinement_level: level,
            sub_classes: BTreeMap::new(),
        });
        
        class.ast_nodes.push(node);
    }
    
    fn refine_complexity_class(&mut self, class: &mut ComplexityClass) {
        println!("  🔍 Refining class {} with {} nodes", class.monster_index, class.ast_nodes.len());
        
        // Group by size (complexity metric)
        let mut size_groups: BTreeMap<usize, Vec<AstNode>> = BTreeMap::new();
        
        for node in &class.ast_nodes {
            size_groups.entry(node.size).or_default().push(node.clone());
        }
        
        // Create sub-classes for each size group
        for (size, nodes) in size_groups {
            if nodes.len() > 1 {
                let sub_index = (class.monster_index << 4) | (size as u16 & 0xF);
                
                let sub_class = ComplexityClass {
                    monster_index: sub_index,
                    ast_nodes: nodes,
                    refinement_level: class.refinement_level + 1,
                    sub_classes: BTreeMap::new(),
                };
                
                class.sub_classes.insert(sub_index, sub_class);
            }
        }
        
        // Recursively refine sub-classes
        for sub_class in class.sub_classes.values_mut() {
            if sub_class.ast_nodes.len() > 2 && sub_class.refinement_level < 3 {
                self.refine_complexity_class(sub_class);
            }
        }
    }
    
    fn print_hierarchy(&self) {
        println!("\n🌳 COMPLEXITY CLASS HIERARCHY:");
        for (index, class) in &self.complexity_classes {
            self.print_class(class, 0);
        }
    }
    
    fn print_class(&self, class: &ComplexityClass, indent: usize) {
        let prefix = "  ".repeat(indent);
        println!("{}Class[{:04x}]: {} nodes (level {})", 
                 prefix, class.monster_index, class.ast_nodes.len(), class.refinement_level);
        
        for sub_class in class.sub_classes.values() {
            self.print_class(sub_class, indent + 1);
        }
    }
}

fn main() {
    println!("🎯 RECURSIVE AST COMPLEXITY CLASSIFICATION");
    
    let mut classifier = RecursiveAstClassifier::new();
    
    // Sample AST nodes with varying complexity
    let test_nodes = vec![
        AstNode { name: "Literal".to_string(), data: 0x1, size: 1, monster_pattern: 0x1 },
        AstNode { name: "Literal".to_string(), data: 0x2, size: 1, monster_pattern: 0x1 },
        AstNode { name: "BinOp".to_string(), data: 0x123, size: 3, monster_pattern: 0x1FF },
        AstNode { name: "BinOp".to_string(), data: 0x456, size: 3, monster_pattern: 0x1FF },
        AstNode { name: "FnCall".to_string(), data: 0x789, size: 5, monster_pattern: 0x3 << 15 },
        AstNode { name: "Match".to_string(), data: 0xABC, size: 8, monster_pattern: 0x3FF },
    ];
    
    classifier.classify_and_cluster(test_nodes);
    classifier.print_hierarchy();
    
    println!("\n✅ ASTs clustered recursively by Monster complexity classes!");
}

// Include previous classifier code
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
        let ast_mappings = [
            ("Literal", 0x1), ("BinOp", 0x1FF), ("FnCall", 0x3 << 15),
            ("Match", 0x3FF), ("Struct", 0x7FF),
        ];
        
        for &(ast_name, pattern) in &ast_mappings {
            self.ast_patterns.insert(ast_name.to_string(), pattern);
        }
    }
    
    fn classify_ast(&self, ast_node: &str, ast_data: u64) -> Option<(u16, f64)> {
        let base_pattern = self.ast_patterns.get(ast_node)?;
        let combined_pattern = base_pattern ^ ast_data;
        let type_idx = self.monster_matcher.match_pattern(combined_pattern)?;
        Some((type_idx, 0.8)) // Simplified
    }
}

#[derive(Debug, Clone)]
struct MonsterPatternMatcher {
    pattern_map: HashMap<u64, u16>,
}

impl MonsterPatternMatcher {
    fn new() -> Self {
        let mut pattern_map = HashMap::new();
        pattern_map.insert(0x1, 1);
        pattern_map.insert(0x1FF, 3);
        pattern_map.insert(0x3 << 15, 5);
        Self { pattern_map }
    }
    
    fn match_pattern(&self, data: u64) -> Option<u16> {
        self.pattern_map.get(&(data & 0xFFFF)).copied().or(Some(0))
    }
}
