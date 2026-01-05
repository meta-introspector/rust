use syn::{visit::Visit, *};
use std::collections::HashMap;

/// Homotopy mapping: f: AST → PrimeComponent
/// Maps every AST node to its topological equivalence class

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PrimeBox {
    Algebraic = 2,      // Option, Result, enum variants
    Control = 3,        // if, match, loop, break, continue
    Relational = 5,     // ==, !=, <, >, comparisons
    Recursive = 7,      // for loops, iterators, recursion
    Presentation = 11,  // println!, format!, Display
    Memory = 13,        // Box, Rc, Arc, references
    Collections = 19,   // Vec, HashMap, arrays
    Concurrency = 17,   // async, await, threads
}

#[derive(Debug)]
struct AstHomotopy {
    mappings: HashMap<String, PrimeBox>,
    node_counts: HashMap<PrimeBox, u32>,
    homotopy_signature: u64,
}

impl AstHomotopy {
    fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            node_counts: HashMap::new(),
            homotopy_signature: 1,
        }
    }
    
    /// The fundamental homotopy mapping function
    fn classify_node(&self, node_type: &str, content: &str) -> PrimeBox {
        // Pattern matching for homotopy classification
        match node_type {
            // Algebraic structures (Prime 2)
            "Option" | "Result" | "Some" | "None" | "Ok" | "Err" => PrimeBox::Algebraic,
            "Enum" | "Variant" if content.contains("Option") || content.contains("Result") => PrimeBox::Algebraic,
            
            // Control flow (Prime 3)
            "If" | "Match" | "Loop" | "While" | "Break" | "Continue" | "Return" => PrimeBox::Control,
            "Block" if content.contains("if") || content.contains("match") => PrimeBox::Control,
            
            // Relational operations (Prime 5)
            "BinOp" if content.contains("==") || content.contains("!=") || 
                      content.contains("<") || content.contains(">") => PrimeBox::Relational,
            "Cmp" | "Eq" | "Ord" => PrimeBox::Relational,
            
            // Recursive structures (Prime 7)
            "ForLoop" | "Iterator" | "IntoIterator" => PrimeBox::Recursive,
            "Path" if content.contains("iter") => PrimeBox::Recursive,
            "Fn" if content.contains("recursive") => PrimeBox::Recursive,
            
            // Presentation layer (Prime 11)
            "Macro" if content.contains("println") || content.contains("format") => PrimeBox::Presentation,
            "Impl" if content.contains("Display") || content.contains("Debug") => PrimeBox::Presentation,
            
            // Memory management (Prime 13)
            "Box" | "Rc" | "Arc" | "Reference" => PrimeBox::Memory,
            "Type" if content.contains("Box") || content.contains("&") => PrimeBox::Memory,
            
            // Collections (Prime 19)
            "Vec" | "HashMap" | "Array" | "Slice" => PrimeBox::Collections,
            "Type" if content.contains("Vec") || content.contains("HashMap") => PrimeBox::Collections,
            
            // Concurrency (Prime 17)
            "Async" | "Await" | "Future" | "Mutex" => PrimeBox::Concurrency,
            
            // Default to algebraic for unknown nodes
            _ => PrimeBox::Algebraic,
        }
    }
    
    fn add_mapping(&mut self, node_id: String, node_type: &str, content: &str) {
        let prime_box = self.classify_node(node_type, content);
        self.mappings.insert(node_id, prime_box);
        *self.node_counts.entry(prime_box).or_insert(0) += 1;
        self.recalculate_homotopy_signature();
    }
    
    fn recalculate_homotopy_signature(&mut self) {
        // Signature = product of (prime^count) for each component
        self.homotopy_signature = 1;
        for (prime_box, count) in &self.node_counts {
            let prime = *prime_box as u64;
            self.homotopy_signature *= prime.pow(*count);
        }
    }
    
    fn get_homotopy_class(&self, node_id: &str) -> Option<PrimeBox> {
        self.mappings.get(node_id).copied()
    }
    
    fn generate_homotopy_map(&self) -> String {
        let mut map = String::new();
        map.push_str("# AST Homotopy Classification Map\n\n");
        
        for prime_box in [
            PrimeBox::Algebraic, PrimeBox::Control, PrimeBox::Relational,
            PrimeBox::Recursive, PrimeBox::Presentation, PrimeBox::Memory,
            PrimeBox::Collections, PrimeBox::Concurrency
        ] {
            let count = self.node_counts.get(&prime_box).unwrap_or(&0);
            let prime = prime_box as u64;
            
            map.push_str(&format!("## Prime {} ({:?}) - {} nodes\n", 
                                prime, prime_box, count));
            
            // Show sample mappings for this prime box
            let samples: Vec<_> = self.mappings.iter()
                .filter(|(_, &pb)| pb == prime_box)
                .take(3)
                .collect();
            
            for (node_id, _) in samples {
                map.push_str(&format!("- {}\n", node_id));
            }
            map.push_str("\n");
        }
        
        map.push_str(&format!("**Homotopy Signature**: {}\n", self.homotopy_signature));
        map
    }
}

/// Visitor that applies homotopy mapping to AST nodes
struct HomotopyVisitor {
    homotopy: AstHomotopy,
    node_counter: u32,
}

impl HomotopyVisitor {
    fn new() -> Self {
        Self {
            homotopy: AstHomotopy::new(),
            node_counter: 0,
        }
    }
    
    fn next_node_id(&mut self) -> String {
        self.node_counter += 1;
        format!("node_{}", self.node_counter)
    }
}

impl<'ast> Visit<'ast> for HomotopyVisitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        let node_id = self.next_node_id();
        let content = "expr_content"; // Simplified since syn doesn't implement Debug
        
        match expr {
            Expr::If(_) => self.homotopy.add_mapping(node_id, "If", content),
            Expr::Match(_) => self.homotopy.add_mapping(node_id, "Match", content),
            Expr::Loop(_) => self.homotopy.add_mapping(node_id, "Loop", content),
            Expr::ForLoop(_) => self.homotopy.add_mapping(node_id, "ForLoop", content),
            Expr::While(_) => self.homotopy.add_mapping(node_id, "While", content),
            Expr::Break(_) => self.homotopy.add_mapping(node_id, "Break", content),
            Expr::Continue(_) => self.homotopy.add_mapping(node_id, "Continue", content),
            Expr::Return(_) => self.homotopy.add_mapping(node_id, "Return", content),
            Expr::Binary(_) => self.homotopy.add_mapping(node_id, "BinOp", content),
            Expr::Path(path) => {
                let path_content = if let Some(ident) = path.path.get_ident() {
                    ident.to_string()
                } else {
                    "path".to_string()
                };
                self.homotopy.add_mapping(node_id, "Path", &path_content);
            },
            Expr::Macro(_) => self.homotopy.add_mapping(node_id, "Macro", content),
            Expr::Reference(_) => self.homotopy.add_mapping(node_id, "Reference", content),
            _ => self.homotopy.add_mapping(node_id, "Expr", content),
        }
        
        syn::visit::visit_expr(self, expr);
    }
    
    fn visit_type(&mut self, ty: &'ast Type) {
        let node_id = self.next_node_id();
        let content = "type_content";
        
        match ty {
            Type::Path(type_path) => {
                let path_str = if let Some(ident) = type_path.path.get_ident() {
                    ident.to_string()
                } else {
                    "path_type".to_string()
                };
                
                if path_str.contains("Option") {
                    self.homotopy.add_mapping(node_id, "Option", &path_str);
                } else if path_str.contains("Result") {
                    self.homotopy.add_mapping(node_id, "Result", &path_str);
                } else if path_str.contains("Vec") {
                    self.homotopy.add_mapping(node_id, "Vec", &path_str);
                } else if path_str.contains("HashMap") {
                    self.homotopy.add_mapping(node_id, "HashMap", &path_str);
                } else if path_str.contains("Box") {
                    self.homotopy.add_mapping(node_id, "Box", &path_str);
                } else {
                    self.homotopy.add_mapping(node_id, "Type", &path_str);
                }
            },
            Type::Reference(_) => self.homotopy.add_mapping(node_id, "Reference", content),
            Type::Array(_) => self.homotopy.add_mapping(node_id, "Array", content),
            Type::Slice(_) => self.homotopy.add_mapping(node_id, "Slice", content),
            _ => self.homotopy.add_mapping(node_id, "Type", content),
        }
        
        syn::visit::visit_type(self, ty);
    }
    
    fn visit_item(&mut self, item: &'ast Item) {
        let node_id = self.next_node_id();
        let content = "item_content";
        
        match item {
            Item::Enum(_) => self.homotopy.add_mapping(node_id, "Enum", content),
            Item::Fn(_) => self.homotopy.add_mapping(node_id, "Fn", content),
            Item::Impl(_) => self.homotopy.add_mapping(node_id, "Impl", content),
            _ => self.homotopy.add_mapping(node_id, "Item", content),
        }
        
        syn::visit::visit_item(self, item);
    }
}

fn main() {
    println!("🌊 AST Homotopy Mapper: f(AST) → PrimeBox");
    println!("=========================================");
    
    // Example Rust code to analyze
    let code = r#"
        use std::collections::HashMap;
        
        fn example() -> Option<i32> {
            let mut map = HashMap::new();
            map.insert("key", 42);
            
            if let Some(value) = map.get("key") {
                println!("Found: {}", value);
                Some(*value)
            } else {
                None
            }
        }
        
        async fn async_example() -> Result<Vec<i32>, String> {
            let data = vec![1, 2, 3];
            for item in data.iter() {
                if *item > 2 {
                    return Ok(vec![*item]);
                }
            }
            Err("No items found".to_string())
        }
    "#;
    
    // Parse and analyze
    match syn::parse_file(code) {
        Ok(syntax_tree) => {
            let mut visitor = HomotopyVisitor::new();
            visitor.visit_file(&syntax_tree);
            
            println!("\n📊 HOMOTOPY CLASSIFICATION RESULTS:");
            println!("===================================");
            
            for (prime_box, count) in &visitor.homotopy.node_counts {
                let prime = *prime_box as u64;
                println!("Prime {} ({:?}): {} nodes", prime, prime_box, count);
            }
            
            println!("\n🔢 Homotopy Signature: {}", visitor.homotopy.homotopy_signature);
            
            // Generate and save homotopy map
            let homotopy_map = visitor.homotopy.generate_homotopy_map();
            std::fs::write("ast_homotopy_map.md", homotopy_map).unwrap();
            
            // Demonstrate homotopy equivalence
            println!("\n🌐 HOMOTOPY EQUIVALENCE CLASSES:");
            println!("================================");
            
            let total_nodes = visitor.homotopy.node_counts.values().sum::<u32>();
            for (prime_box, count) in &visitor.homotopy.node_counts {
                let percentage = (*count as f64 / total_nodes as f64) * 100.0;
                println!("{:?}: {:.1}% of AST", prime_box, percentage);
            }
            
            println!("\n📁 Generated: ast_homotopy_map.md");
            println!("🎉 AST successfully mapped to prime component homotopy classes!");
            
        },
        Err(e) => println!("Parse error: {}", e),
    }
}
