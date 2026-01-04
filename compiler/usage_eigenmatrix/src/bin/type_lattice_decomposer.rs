use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<()> {
    println!("🎯 TYPE-DRIVEN LATTICE DECOMPOSITION");
    println!("═══════════════════════════════════");
    
    let mut decomposer = TypeLatticeDecomposer::new();
    
    // Extract all types from our analysis
    decomposer.extract_type_universe()?;
    
    // Split compiler into type-based lattice nodes
    decomposer.decompose_by_types()?;
    
    // Generate type-specific compiler components
    decomposer.generate_typed_components()?;
    
    Ok(())
}

struct TypeLatticeDecomposer {
    // Type universe extracted from rustc
    domain_types: HashSet<String>,
    range_types: HashSet<String>, 
    body_types: HashSet<String>,
    
    // Original compiler graph
    compiler_graph: HashMap<String, Vec<String>>,
    
    // Type-based lattice decomposition
    domain_lattice: HashMap<String, Vec<String>>,  // domain_type -> functions
    range_lattice: HashMap<String, Vec<String>>,   // range_type -> functions
    body_lattice: HashMap<String, Vec<String>>,    // body_type -> functions
    
    // Generated components
    typed_components: HashMap<String, TypedComponent>,
}

#[derive(Debug, Clone)]
struct TypedComponent {
    name: String,
    domain_type: String,
    range_type: String,
    body_types: Vec<String>,
    lattice_position: LatticePosition,
    generated_code: String,
}

#[derive(Debug, Clone)]
struct LatticePosition {
    domain_coord: usize,
    range_coord: usize,
    body_coord: usize,
}

impl TypeLatticeDecomposer {
    fn new() -> Self {
        Self {
            domain_types: HashSet::new(),
            range_types: HashSet::new(),
            body_types: HashSet::new(),
            compiler_graph: HashMap::new(),
            domain_lattice: HashMap::new(),
            range_lattice: HashMap::new(),
            body_lattice: HashMap::new(),
            typed_components: HashMap::new(),
        }
    }
    
    fn extract_type_universe(&mut self) -> Result<()> {
        println!("🔍 Extracting type universe from rustc analysis...");
        
        // Load our equivalence analysis
        if let Ok(equiv_data) = fs::read_to_string("rustc_equivalence_analysis.json") {
            let data: serde_json::Value = serde_json::from_str(&equiv_data)?;
            
            // Extract types from largest classes
            if let Some(largest_classes) = data["largest_classes"].as_array() {
                for class in largest_classes {
                    if let Some(signature) = class["signature"].as_str() {
                        self.parse_signature_types(signature);
                    }
                }
            }
        }
        
        // Load compiler graph
        self.load_compiler_graph()?;
        
        // Extract additional types from function names
        self.extract_types_from_function_names();
        
        println!("  Type universe extracted:");
        println!("    Domain types: {}", self.domain_types.len());
        println!("    Range types: {}", self.range_types.len());
        println!("    Body types: {}", self.body_types.len());
        
        // Show sample types
        println!("  Sample domain types: {:?}", 
            self.domain_types.iter().take(5).collect::<Vec<_>>());
        println!("  Sample range types: {:?}", 
            self.range_types.iter().take(5).collect::<Vec<_>>());
        
        Ok(())
    }
    
    fn parse_signature_types(&mut self, signature: &str) {
        // Parse signatures like "DefId → Generic (Medium, Utility)"
        if let Some(arrow_pos) = signature.find(" → ") {
            let domain = signature[..arrow_pos].trim();
            let rest = &signature[arrow_pos + 3..];
            
            if let Some(paren_pos) = rest.find(" (") {
                let range = rest[..paren_pos].trim();
                
                self.domain_types.insert(domain.to_string());
                self.range_types.insert(range.to_string());
                
                // Extract body types from parentheses
                if let Some(start) = rest.find('(') {
                    if let Some(end) = rest.find(')') {
                        let body_part = &rest[start+1..end];
                        for body_type in body_part.split(',') {
                            self.body_types.insert(body_type.trim().to_string());
                        }
                    }
                }
            }
        }
    }
    
    fn load_compiler_graph(&mut self) -> Result<()> {
        // Load from our self-constructed compiler
        if let Ok(compiler_data) = fs::read_to_string("self_constructed_rustc.json") {
            let data: serde_json::Value = serde_json::from_str(&compiler_data)?;
            
            if let Some(components) = data["components"].as_array() {
                for component in components {
                    if let Some(name) = component["name"].as_str() {
                        if let Some(comp_type) = component["type"].as_str() {
                            self.compiler_graph.entry(name.to_string())
                                .or_insert_with(Vec::new)
                                .push(comp_type.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn extract_types_from_function_names(&mut self) {
        for func_name in self.compiler_graph.keys() {
            // Extract types from function names
            if func_name.contains("DefId") {
                self.domain_types.insert("DefId".to_string());
            }
            if func_name.contains("Span") {
                self.domain_types.insert("Span".to_string());
            }
            if func_name.contains("Ty") || func_name.contains("Type") {
                self.domain_types.insert("Type".to_string());
                self.range_types.insert("Type".to_string());
            }
            if func_name.contains("Expr") {
                self.domain_types.insert("Expression".to_string());
            }
            if func_name.contains("bool") || func_name.contains("is_") {
                self.range_types.insert("bool".to_string());
            }
            if func_name.contains("String") || func_name.contains("str") {
                self.range_types.insert("String".to_string());
            }
            if func_name.contains("Vec") {
                self.range_types.insert("Vec".to_string());
            }
            if func_name.contains("Option") {
                self.range_types.insert("Option".to_string());
            }
            if func_name.contains("Result") {
                self.range_types.insert("Result".to_string());
            }
            
            // Body types from function behavior
            if func_name.contains("parse") {
                self.body_types.insert("Parser".to_string());
            }
            if func_name.contains("analyze") {
                self.body_types.insert("Analyzer".to_string());
            }
            if func_name.contains("transform") {
                self.body_types.insert("Transformer".to_string());
            }
            if func_name.contains("codegen") {
                self.body_types.insert("Generator".to_string());
            }
        }
    }
    
    fn decompose_by_types(&mut self) -> Result<()> {
        println!("\n🧩 Decomposing compiler into type-based lattice...");
        
        // Create lattice dimensions
        let domain_types: Vec<String> = self.domain_types.iter().cloned().collect();
        let range_types: Vec<String> = self.range_types.iter().cloned().collect();
        let body_types: Vec<String> = self.body_types.iter().cloned().collect();
        
        println!("  Lattice dimensions:");
        println!("    Domain: {} types", domain_types.len());
        println!("    Range: {} types", range_types.len());
        println!("    Body: {} types", body_types.len());
        println!("    Total lattice points: {}", 
            domain_types.len() * range_types.len() * body_types.len());
        
        // Classify each function into lattice positions
        for func_name in self.compiler_graph.keys() {
            let domain_type = self.infer_domain_type(func_name);
            let range_type = self.infer_range_type(func_name);
            let body_type = self.infer_body_type(func_name);
            
            // Add to domain lattice
            self.domain_lattice.entry(domain_type.clone())
                .or_insert_with(Vec::new)
                .push(func_name.clone());
            
            // Add to range lattice
            self.range_lattice.entry(range_type.clone())
                .or_insert_with(Vec::new)
                .push(func_name.clone());
            
            // Add to body lattice
            self.body_lattice.entry(body_type.clone())
                .or_insert_with(Vec::new)
                .push(func_name.clone());
        }
        
        println!("  Functions classified into lattice:");
        println!("    Domain lattice nodes: {}", self.domain_lattice.len());
        println!("    Range lattice nodes: {}", self.range_lattice.len());
        println!("    Body lattice nodes: {}", self.body_lattice.len());
        
        Ok(())
    }
    
    fn infer_domain_type(&self, func_name: &str) -> String {
        if func_name.contains("DefId") { "DefId".to_string() }
        else if func_name.contains("Span") { "Span".to_string() }
        else if func_name.contains("Type") || func_name.contains("Ty") { "Type".to_string() }
        else if func_name.contains("Expr") { "Expression".to_string() }
        else if func_name.contains("String") || func_name.contains("str") { "String".to_string() }
        else { "Generic".to_string() }
    }
    
    fn infer_range_type(&self, func_name: &str) -> String {
        if func_name.contains("bool") || func_name.contains("is_") { "bool".to_string() }
        else if func_name.contains("Option") { "Option".to_string() }
        else if func_name.contains("Result") { "Result".to_string() }
        else if func_name.contains("Vec") { "Vec".to_string() }
        else if func_name.contains("String") { "String".to_string() }
        else { "Generic".to_string() }
    }
    
    fn infer_body_type(&self, func_name: &str) -> String {
        if func_name.contains("parse") { "Parser".to_string() }
        else if func_name.contains("analyze") { "Analyzer".to_string() }
        else if func_name.contains("transform") { "Transformer".to_string() }
        else if func_name.contains("codegen") { "Generator".to_string() }
        else if func_name.contains("check") { "Validator".to_string() }
        else { "Utility".to_string() }
    }
    
    fn generate_typed_components(&mut self) -> Result<()> {
        println!("\n⚡ Generating typed compiler components...");
        
        let domain_types: Vec<String> = self.domain_types.iter().cloned().collect();
        let range_types: Vec<String> = self.range_types.iter().cloned().collect();
        let body_types: Vec<String> = self.body_types.iter().cloned().collect();
        
        let mut component_count = 0;
        
        // Generate component for each lattice point
        for (d, domain_type) in domain_types.iter().enumerate() {
            for (r, range_type) in range_types.iter().enumerate() {
                for (b, body_type) in body_types.iter().enumerate() {
                    
                    // Check if this lattice point has functions
                    let domain_funcs = self.domain_lattice.get(domain_type).cloned().unwrap_or_default();
                    let range_funcs = self.range_lattice.get(range_type).cloned().unwrap_or_default();
                    let body_funcs = self.body_lattice.get(body_type).cloned().unwrap_or_default();
                    
                    // Find intersection of functions
                    let intersection = self.find_function_intersection(&domain_funcs, &range_funcs, &body_funcs);
                    
                    if !intersection.is_empty() {
                        let component_name = format!("{}_{}_{}_{}", domain_type, range_type, body_type, component_count);
                        
                        let component = TypedComponent {
                            name: component_name.clone(),
                            domain_type: domain_type.clone(),
                            range_type: range_type.clone(),
                            body_types: vec![body_type.clone()],
                            lattice_position: LatticePosition {
                                domain_coord: d,
                                range_coord: r,
                                body_coord: b,
                            },
                            generated_code: self.generate_component_code(domain_type, range_type, body_type, &intersection),
                        };
                        
                        self.typed_components.insert(component_name, component);
                        component_count += 1;
                    }
                }
            }
        }
        
        println!("  Generated {} typed components", self.typed_components.len());
        
        // Save typed lattice
        self.save_typed_lattice()?;
        
        Ok(())
    }
    
    fn find_function_intersection(&self, domain_funcs: &[String], range_funcs: &[String], body_funcs: &[String]) -> Vec<String> {
        let domain_set: HashSet<_> = domain_funcs.iter().collect();
        let range_set: HashSet<_> = range_funcs.iter().collect();
        let body_set: HashSet<_> = body_funcs.iter().collect();
        
        domain_set.intersection(&range_set)
            .filter(|f| body_set.contains(*f))
            .map(|f| f.to_string())
            .collect()
    }
    
    fn generate_component_code(&self, domain_type: &str, range_type: &str, body_type: &str, functions: &[String]) -> String {
        format!(r#"
// Typed Component: {} → {} (Body: {})
// Generated from {} functions
struct {}{}{}Component {{
    functions: Vec<String>,
}}

impl {}{}{}Component {{
    fn new() -> Self {{
        Self {{
            functions: vec![{}],
        }}
    }}
    
    fn process(&self, input: {}) -> {} {{
        // {} implementation
        todo!("Generated from lattice position")
    }}
}}
"#, 
            domain_type, range_type, body_type, functions.len(),
            domain_type, range_type, body_type,
            domain_type, range_type, body_type,
            functions.iter().map(|f| format!("\"{}\"", f)).collect::<Vec<_>>().join(", "),
            self.rust_type_name(domain_type),
            self.rust_type_name(range_type),
            body_type
        )
    }
    
    fn rust_type_name(&self, type_name: &str) -> &str {
        match type_name {
            "DefId" => "DefId",
            "Type" => "Type",
            "Expression" => "Expr",
            "String" => "String",
            "bool" => "bool",
            "Option" => "Option<T>",
            "Result" => "Result<T, E>",
            "Vec" => "Vec<T>",
            _ => "&str"
        }
    }
    
    fn save_typed_lattice(&self) -> Result<()> {
        let lattice_data = serde_json::json!({
            "metadata": {
                "description": "Type-driven lattice decomposition of rustc",
                "domain_types": self.domain_types.len(),
                "range_types": self.range_types.len(),
                "body_types": self.body_types.len(),
                "total_components": self.typed_components.len()
            },
            "type_universe": {
                "domains": self.domain_types.iter().collect::<Vec<_>>(),
                "ranges": self.range_types.iter().collect::<Vec<_>>(),
                "bodies": self.body_types.iter().collect::<Vec<_>>()
            },
            "lattice_structure": {
                "domain_lattice": self.domain_lattice.iter().map(|(k, v)| (k, v.len())).collect::<HashMap<_, _>>(),
                "range_lattice": self.range_lattice.iter().map(|(k, v)| (k, v.len())).collect::<HashMap<_, _>>(),
                "body_lattice": self.body_lattice.iter().map(|(k, v)| (k, v.len())).collect::<HashMap<_, _>>()
            },
            "components": self.typed_components.values().map(|c| {
                serde_json::json!({
                    "name": c.name,
                    "domain": c.domain_type,
                    "range": c.range_type,
                    "body": c.body_types,
                    "position": format!("({}, {}, {})", 
                        c.lattice_position.domain_coord,
                        c.lattice_position.range_coord,
                        c.lattice_position.body_coord)
                })
            }).collect::<Vec<_>>()
        });
        
        fs::write("typed_lattice_decomposition.json", serde_json::to_string_pretty(&lattice_data)?)?;
        
        // Generate Rust code file
        let mut rust_code = String::new();
        rust_code.push_str("// TYPE-DRIVEN LATTICE COMPILER COMPONENTS\n");
        rust_code.push_str("// Generated from rustc type analysis\n\n");
        
        for component in self.typed_components.values() {
            rust_code.push_str(&component.generated_code);
            rust_code.push('\n');
        }
        
        fs::write("typed_lattice_compiler.rs", rust_code)?;
        
        println!("💾 Typed lattice saved:");
        println!("  Metadata: typed_lattice_decomposition.json");
        println!("  Code: typed_lattice_compiler.rs");
        
        Ok(())
    }
}
