use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use serde_json::Value;
use syn::{parse_file, visit::Visit, Item};

#[derive(Debug)]
struct DependencyResolver {
    symbol_map: HashMap<String, Value>,
    processed_files: HashMap<String, String>,
    resolved_order: Vec<String>,
    included_crates: HashSet<String>,
}

impl DependencyResolver {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("🔄 Loading symbol map and processed files...");
        
        // Load symbol map
        let symbol_map_content = fs::read_to_string("symbol_map.json")?;
        let symbol_map: HashMap<String, Value> = serde_json::from_str(&symbol_map_content)?;
        
        // Load all processed files
        let mut processed_files = HashMap::new();
        for entry in fs::read_dir("submodules/rust/compiler")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let crate_name = entry.file_name().to_string_lossy().to_string();
                let lib_path = entry.path().join("src/lib.rs");
                if lib_path.exists() {
                    if let Ok(content) = fs::read_to_string(&lib_path) {
                        processed_files.insert(crate_name, content);
                    }
                }
            }
        }
        
        println!("✅ Loaded {} symbols and {} processed files", symbol_map.len(), processed_files.len());
        
        Ok(Self {
            symbol_map,
            processed_files,
            resolved_order: Vec::new(),
            included_crates: HashSet::new(),
        })
    }
    
    pub fn resolve_complete_dependency_tree(&mut self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("🎯 Resolving complete dependency tree for: {}", target);
        
        // Step 1: Find all dependencies recursively
        let all_deps = self.find_all_dependencies(target)?;
        println!("📊 Found {} total dependencies", all_deps.len());
        
        // Step 2: Topologically sort dependencies
        self.resolved_order = self.topological_sort(&all_deps)?;
        println!("🔄 Sorted {} dependencies in correct order", self.resolved_order.len());
        
        // Step 3: Generate complete code with all dependencies
        let complete_code = self.generate_complete_code(target)?;
        
        Ok(complete_code)
    }
    
    fn find_all_dependencies(&self, target: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
        let mut all_deps = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(target.to_string());
        
        while let Some(current) = queue.pop_front() {
            if all_deps.contains(&current) {
                continue;
            }
            
            all_deps.insert(current.clone());
            
            // Get dependencies from symbol map
            if let Some(entry) = self.symbol_map.get(&current) {
                if let Some(deps) = entry.get("dependencies").and_then(|d| d.as_array()) {
                    for dep in deps {
                        if let Some(dep_str) = dep.as_str() {
                            queue.push_back(dep_str.to_string());
                        }
                    }
                }
            }
            
            // Also analyze AST for additional dependencies
            if let Some(source_file) = self.get_source_file(&current) {
                if let Ok(ast_deps) = self.extract_ast_dependencies(&source_file) {
                    for dep in ast_deps {
                        queue.push_back(dep);
                    }
                }
            }
        }
        
        Ok(all_deps)
    }
    
    fn extract_ast_dependencies(&self, source_file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut visitor = DependencyVisitor::new();
        
        if let Ok(ast) = parse_file(source_file) {
            visitor.visit_file(&ast);
        }
        
        Ok(visitor.dependencies)
    }
    
    fn topological_sort(&self, deps: &HashSet<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        
        for dep in deps {
            if !visited.contains(dep) {
                self.dfs_sort(dep, &mut visited, &mut temp_visited, &mut sorted)?;
            }
        }
        
        sorted.reverse(); // Reverse for correct dependency order
        Ok(sorted)
    }
    
    fn dfs_sort(&self, node: &str, visited: &mut HashSet<String>, temp_visited: &mut HashSet<String>, sorted: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if temp_visited.contains(node) {
            return Ok(()); // Cycle detected, skip
        }
        if visited.contains(node) {
            return Ok(());
        }
        
        temp_visited.insert(node.to_string());
        
        // Visit dependencies first
        if let Some(entry) = self.symbol_map.get(node) {
            if let Some(deps) = entry.get("dependencies").and_then(|d| d.as_array()) {
                for dep in deps {
                    if let Some(dep_str) = dep.as_str() {
                        self.dfs_sort(dep_str, visited, temp_visited, sorted)?;
                    }
                }
            }
        }
        
        temp_visited.remove(node);
        visited.insert(node.to_string());
        sorted.push(node.to_string());
        
        Ok(())
    }
    
    fn generate_complete_code(&mut self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();
        
        // Add header
        code.push_str("#![recursion_limit = \"512\"]\n");
        code.push_str("#![allow(internal_features)]\n");
        code.push_str("#![allow(unused)]\n");
        code.push_str("#![feature(rustc_private)]\n\n");
        
        // Add all required extern crates
        let required_crates = self.find_required_crates()?;
        for crate_name in &required_crates {
            code.push_str(&format!("extern crate {};\n", crate_name));
            self.included_crates.insert(crate_name.clone());
        }
        code.push_str("\n");
        
        // Include wrap_types with resolver macros
        code.push_str("// Custom macro to include processed rustc files\n");
        code.push_str("macro_rules! include_rustc {\n");
        code.push_str("    ($crate_name:ident, $file:ident) => {\n");
        code.push_str("        include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/submodules/rust/compiler/\", stringify!($crate_name), \"/src/\", stringify!($file), \".rs\"));\n");
        code.push_str("    };\n");
        code.push_str("    ($crate_name:ident) => {\n");
        code.push_str("        include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/submodules/rust/compiler/\", stringify!($crate_name), \"/src/lib.rs\"));\n");
        code.push_str("    };\n");
        code.push_str("}\n\n");
        
        // Include all dependencies in correct order
        for dep in &self.resolved_order {
            if let Some(crate_name) = self.get_crate_name(dep) {
                code.push_str(&format!("// === {} ===\n", dep));
                code.push_str(&format!("include_rustc!({});\n\n", crate_name));
            }
        }
        
        // Add target code
        code.push_str(&format!("// === TARGET: {} ===\n", target));
        if let Some(target_code) = self.get_target_code(target) {
            code.push_str(&target_code);
        }
        
        Ok(code)
    }
    
    fn find_required_crates(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut crates = HashSet::new();
        
        // Standard rustc crates
        let rustc_crates = vec![
            "rustc_driver", "rustc_driver_impl", "rustc_session", "rustc_middle",
            "rustc_ast", "rustc_hir", "rustc_data_structures", "rustc_span",
            "rustc_errors", "rustc_interface", "rustc_codegen_ssa", "rustc_target",
            "rustc_metadata", "rustc_parse", "rustc_expand", "rustc_builtin_macros",
            "rustc_passes", "rustc_mir_build", "rustc_mir_transform", "rustc_mir_dataflow",
            "rustc_const_eval", "rustc_hir_analysis", "rustc_hir_typeck", "rustc_traits",
            "rustc_trait_selection", "rustc_infer", "rustc_borrowck", "rustc_privacy",
            "rustc_resolve", "rustc_lint", "rustc_serialize", "rustc_index", "rustc_macros"
        ];
        
        for crate_name in rustc_crates {
            crates.insert(crate_name.to_string());
        }
        
        Ok(crates.into_iter().collect())
    }
    
    fn get_source_file(&self, symbol: &str) -> Option<String> {
        self.symbol_map.get(symbol)
            .and_then(|entry| entry.get("source_file"))
            .and_then(|s| s.as_str())
            .and_then(|path| fs::read_to_string(format!("submodules/{}", path)).ok())
    }
    
    fn get_crate_name(&self, symbol: &str) -> Option<&str> {
        self.symbol_map.get(symbol)
            .and_then(|entry| entry.get("crate_name"))
            .and_then(|s| s.as_str())
    }
    
    fn get_target_code(&self, target: &str) -> Option<String> {
        // Generate the final target code
        Some(format!("fn main() {{\n    {}();\n}}", target))
    }
}

struct DependencyVisitor {
    dependencies: Vec<String>,
}

impl DependencyVisitor {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for DependencyVisitor {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        // Extract use statements as dependencies
        if let syn::UseTree::Path(path) = &node.tree {
            let path_str = path.ident.to_string();
            if !self.dependencies.contains(&path_str) {
                self.dependencies.push(path_str);
            }
        }
        syn::visit::visit_item_use(self, node);
    }
    
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        // Extract function calls as potential dependencies
        syn::visit::visit_item_fn(self, node);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <target_symbol>", args[0]);
        std::process::exit(1);
    }
    
    let target = &args[1];
    let mut resolver = DependencyResolver::new()?;
    
    println!("🚀 Starting complete dependency resolution...");
    let complete_code = resolver.resolve_complete_dependency_tree(target)?;
    
    // Write the complete resolved code
    fs::write("src/resolved_complete.rs", &complete_code)?;
    println!("✅ Generated complete resolved code: src/resolved_complete.rs");
    println!("📊 Included {} crates and {} dependencies", 
             resolver.included_crates.len(), 
             resolver.resolved_order.len());
    
    // Test compilation
    println!("🔧 Testing compilation...");
    let output = std::process::Command::new("cargo")
        .args(&["check", "--lib"])
        .output()?;
    
    if output.status.success() {
        println!("✅ Compilation successful!");
    } else {
        println!("❌ Compilation failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}
