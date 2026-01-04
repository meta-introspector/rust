use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<()> {
    println!("🔢 CONSTANT-ONLY SUBGRAPH EXTRACTOR & COMPOSER");
    println!("═══════════════════════════════════════════════");
    
    let mut extractor = ConstantSubgraphExtractor::new();
    
    // Extract only constant-related paths from rustc
    extractor.extract_constant_subgraph()?;
    
    // Compose minimal compiler from lattice parts
    extractor.compose_minimal_compiler()?;
    
    Ok(())
}

struct ConstantSubgraphExtractor {
    full_graph: HashMap<String, Vec<String>>,
    constant_subgraph: HashMap<String, Vec<String>>,
    lattice_parts: HashMap<String, LatticePart>,
    minimal_compiler: Vec<String>,
}

#[derive(Debug, Clone)]
struct LatticePart {
    name: String,
    part_type: LatticeType,
    dependencies: Vec<String>,
    rust_code: String,
}

#[derive(Debug, Clone)]
enum LatticeType {
    ConstantParser,
    ConstantEvaluator, 
    ConstantPrinter,
    ConstantValidator,
    MinimalDriver,
}

impl ConstantSubgraphExtractor {
    fn new() -> Self {
        Self {
            full_graph: HashMap::new(),
            constant_subgraph: HashMap::new(),
            lattice_parts: HashMap::new(),
            minimal_compiler: Vec::new(),
        }
    }
    
    fn extract_constant_subgraph(&mut self) -> Result<()> {
        println!("🔍 Extracting constant-only subgraph from rustc...");
        
        // Load full rustc graph
        self.load_full_graph()?;
        
        // Find constant-related entry points
        let constant_roots = self.find_constant_roots();
        println!("  Found {} constant entry points", constant_roots.len());
        
        // Extract subgraph using BFS from constant roots
        self.extract_subgraph_from_roots(&constant_roots);
        
        println!("  Subgraph extracted:");
        println!("    Full graph: {} nodes", self.full_graph.len());
        println!("    Constant subgraph: {} nodes", self.constant_subgraph.len());
        println!("    Reduction: {:.1}%", 
            (self.constant_subgraph.len() as f64 / self.full_graph.len() as f64) * 100.0);
        
        Ok(())
    }
    
    fn load_full_graph(&mut self) -> Result<()> {
        // Load from usage data
        let usage_dir = "../../usage_data";
        let mut file_count = 0;
        
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: serde_json::Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage_obj in usages {
                        let user_id = usage_obj["user_def_id"].as_str().unwrap_or("unknown");
                        let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                        
                        self.full_graph.entry(user_id.to_string())
                            .or_insert_with(Vec::new)
                            .push(used_id.to_string());
                    }
                }
                
                file_count += 1;
                if file_count >= 200 {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn find_constant_roots(&self) -> Vec<String> {
        let mut roots = Vec::new();
        
        for node in self.full_graph.keys() {
            if self.is_constant_related(node) {
                roots.push(node.clone());
            }
        }
        
        roots
    }
    
    fn is_constant_related(&self, node: &str) -> bool {
        let node_lower = node.to_lowercase();
        
        // Constant-specific patterns
        node_lower.contains("const") ||
        node_lower.contains("literal") ||
        node_lower.contains("static") ||
        node.starts_with("\"") ||
        node.chars().all(|c| c.is_ascii_digit()) ||
        node == "true" || node == "false" ||
        
        // Essential parsing for constants
        (node_lower.contains("parse") && (
            node_lower.contains("literal") ||
            node_lower.contains("const") ||
            node_lower.contains("number") ||
            node_lower.contains("string")
        )) ||
        
        // Essential evaluation for constants
        (node_lower.contains("eval") && node_lower.contains("const")) ||
        
        // Essential printing
        (node_lower.contains("fmt") || node_lower.contains("print")) ||
        
        // Core types needed for constants
        node_lower.contains("i32") || node_lower.contains("str") || 
        node_lower.contains("bool") || node_lower.contains("f64")
    }
    
    fn extract_subgraph_from_roots(&mut self, roots: &[String]) {
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        // Start BFS from all roots
        for root in roots {
            queue.push_back(root.clone());
        }
        
        while let Some(current) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            
            visited.insert(current.clone());
            
            if let Some(connections) = self.full_graph.get(&current) {
                // Only include connections that are also constant-related
                let filtered_connections: Vec<String> = connections.iter()
                    .filter(|conn| self.is_constant_related(conn))
                    .cloned()
                    .collect();
                
                if !filtered_connections.is_empty() {
                    self.constant_subgraph.insert(current.clone(), filtered_connections.clone());
                    
                    // Add connections to queue
                    for conn in &filtered_connections {
                        if !visited.contains(conn) {
                            queue.push_back(conn.clone());
                        }
                    }
                }
            }
        }
    }
    
    fn compose_minimal_compiler(&mut self) -> Result<()> {
        println!("\n🏗️ Composing minimal compiler from lattice parts...");
        
        // Create lattice parts from subgraph
        self.create_lattice_parts();
        
        // Compose minimal compiler
        self.assemble_minimal_compiler();
        
        // Generate final compiler code
        self.generate_compiler_code()?;
        
        Ok(())
    }
    
    fn create_lattice_parts(&mut self) {
        println!("🔧 Creating lattice parts from subgraph...");
        
        for (node, connections) in &self.constant_subgraph {
            let part_type = self.classify_lattice_part(node);
            let rust_code = self.generate_rust_code_for_node(node, &part_type);
            
            let lattice_part = LatticePart {
                name: node.clone(),
                part_type,
                dependencies: connections.clone(),
                rust_code,
            };
            
            self.lattice_parts.insert(node.clone(), lattice_part);
        }
        
        println!("  Created {} lattice parts", self.lattice_parts.len());
        
        // Count by type
        let mut type_counts = HashMap::new();
        for part in self.lattice_parts.values() {
            *type_counts.entry(format!("{:?}", part.part_type)).or_insert(0) += 1;
        }
        
        println!("  Lattice part distribution:");
        for (part_type, count) in &type_counts {
            println!("    {}: {} parts", part_type, count);
        }
    }
    
    fn classify_lattice_part(&self, node: &str) -> LatticeType {
        let node_lower = node.to_lowercase();
        
        if node_lower.contains("parse") && (node_lower.contains("const") || node_lower.contains("literal")) {
            LatticeType::ConstantParser
        } else if node_lower.contains("eval") && node_lower.contains("const") {
            LatticeType::ConstantEvaluator
        } else if node_lower.contains("fmt") || node_lower.contains("print") {
            LatticeType::ConstantPrinter
        } else if node_lower.contains("check") || node_lower.contains("valid") {
            LatticeType::ConstantValidator
        } else if node_lower.contains("main") || node_lower.contains("driver") {
            LatticeType::MinimalDriver
        } else {
            LatticeType::ConstantParser // Default
        }
    }
    
    fn generate_rust_code_for_node(&self, node: &str, part_type: &LatticeType) -> String {
        match part_type {
            LatticeType::ConstantParser => {
                format!(r#"
// Generated from: {}
fn parse_constant(input: &str) -> Option<ConstantValue> {{
    if let Ok(i) = input.parse::<i64>() {{
        Some(ConstantValue::Integer(i))
    }} else if input == "true" || input == "false" {{
        Some(ConstantValue::Boolean(input == "true"))
    }} else if input.starts_with('"') && input.ends_with('"') {{
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    }} else {{
        None
    }}
}}
"#, node)
            },
            
            LatticeType::ConstantEvaluator => {
                format!(r#"
// Generated from: {}
fn evaluate_constant(value: &ConstantValue) -> String {{
    match value {{
        ConstantValue::Integer(i) => i.to_string(),
        ConstantValue::String(s) => s.clone(),
        ConstantValue::Boolean(b) => b.to_string(),
        ConstantValue::Float(f) => f.to_string(),
    }}
}}
"#, node)
            },
            
            LatticeType::ConstantPrinter => {
                format!(r#"
// Generated from: {}
fn print_constant(value: &ConstantValue) {{
    match value {{
        ConstantValue::Integer(i) => println!("{{}}", i),
        ConstantValue::String(s) => println!("{{}}", s),
        ConstantValue::Boolean(b) => println!("{{}}", b),
        ConstantValue::Float(f) => println!("{{}}", f),
    }}
}}
"#, node)
            },
            
            LatticeType::ConstantValidator => {
                format!(r#"
// Generated from: {}
fn validate_constant(input: &str) -> bool {{
    input.parse::<i64>().is_ok() ||
    input == "true" || input == "false" ||
    (input.starts_with('"') && input.ends_with('"'))
}}
"#, node)
            },
            
            LatticeType::MinimalDriver => {
                format!(r#"
// Generated from: {}
fn main() {{
    let input = std::env::args().nth(1).unwrap_or_default();
    if let Some(constant) = parse_constant(&input) {{
        print_constant(&constant);
    }}
}}
"#, node)
            },
        }
    }
    
    fn assemble_minimal_compiler(&mut self) {
        println!("🔗 Assembling minimal compiler from lattice parts...");
        
        // Order parts by dependencies (topological sort)
        let ordered_parts = self.topological_sort_parts();
        
        self.minimal_compiler = ordered_parts;
        
        println!("  Assembled {} components in dependency order", self.minimal_compiler.len());
    }
    
    fn topological_sort_parts(&self) -> Vec<String> {
        // Simple topological sort
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        
        // Add parts with no dependencies first
        for (name, part) in &self.lattice_parts {
            if part.dependencies.is_empty() {
                result.push(name.clone());
                visited.insert(name.clone());
            }
        }
        
        // Add remaining parts
        for (name, _) in &self.lattice_parts {
            if !visited.contains(name) {
                result.push(name.clone());
            }
        }
        
        result
    }
    
    fn generate_compiler_code(&self) -> Result<()> {
        println!("📝 Generating final minimal compiler code...");
        
        let mut compiler_code = String::new();
        
        // Add header
        compiler_code.push_str(r#"
// MINIMAL CONSTANT-ONLY COMPILER
// Generated from rustc subgraph extraction
// Lattice composition of constant-handling parts only

#[derive(Debug, Clone)]
enum ConstantValue {
    Integer(i64),
    String(String),
    Boolean(bool),
    Float(f64),
}

"#);
        
        // Add lattice parts in order
        for part_name in &self.minimal_compiler {
            if let Some(part) = self.lattice_parts.get(part_name) {
                compiler_code.push_str(&part.rust_code);
                compiler_code.push('\n');
            }
        }
        
        // Save compiler
        fs::write("minimal_constant_compiler.rs", compiler_code)?;
        
        println!("✅ Minimal compiler generated!");
        println!("  Output: minimal_constant_compiler.rs");
        println!("  Size reduction: {} → {} components", 
            self.full_graph.len(), self.lattice_parts.len());
        
        // Save composition metadata
        let metadata = serde_json::json!({
            "original_graph_size": self.full_graph.len(),
            "constant_subgraph_size": self.constant_subgraph.len(),
            "lattice_parts": self.lattice_parts.len(),
            "reduction_ratio": (self.constant_subgraph.len() as f64 / self.full_graph.len() as f64) * 100.0,
            "composition_method": "subgraph_extraction_and_lattice_assembly"
        });
        
        fs::write("minimal_compiler_metadata.json", serde_json::to_string_pretty(&metadata)?)?;
        
        Ok(())
    }
}
