use std::collections::HashSet;
use std::fs;
use std::path::Path;
use anyhow::Result;
use syn::{parse_file, Item, UseTree, Path as SynPath};

/// Custom rustc driver for automatic stub generation
pub struct StubGenerator {
    missing_imports: HashSet<String>,
    generated_stubs: HashSet<String>,
}

impl StubGenerator {
    pub fn new() -> Self {
        Self {
            missing_imports: HashSet::new(),
            generated_stubs: HashSet::new(),
        }
    }

    /// Process a single Rust file and collect missing imports
    pub fn process_file(&mut self, file_path: &Path) -> Result<()> {
        let content = fs::read_to_string(file_path)?;
        let syntax_tree = parse_file(&content)?;

        for item in syntax_tree.items {
            self.process_item(&item);
        }

        Ok(())
    }

    /// Process a single AST item
    fn process_item(&mut self, item: &Item) {
        match item {
            Item::Use(use_item) => {
                self.process_use_tree(&use_item.tree);
            }
            Item::Mod(mod_item) => {
                if let Some((_, items)) = &mod_item.content {
                    for item in items {
                        self.process_item(item);
                    }
                }
            }
            _ => {}
        }
    }

    /// Process use statements to find imports
    fn process_use_tree(&mut self, tree: &UseTree) {
        match tree {
            UseTree::Path(path) => {
                let path_str = path.ident.to_string();
                if path_str.starts_with("crate::") {
                    self.missing_imports.insert(path_str);
                }
                self.process_use_tree(&path.tree);
            }
            UseTree::Name(name) => {
                // Terminal import
            }
            UseTree::Rename(rename) => {
                // Renamed import
            }
            UseTree::Glob(_) => {
                // Glob import
            }
            UseTree::Group(group) => {
                for tree in &group.items {
                    self.process_use_tree(tree);
                }
            }
        }
    }

    /// Convert syn::Path to string
    fn path_to_string(&self, path: &SynPath) -> String {
        path.segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
    }

    /// Generate stub modules for all missing imports
    pub fn generate_stubs(&mut self) -> String {
        let mut stubs: Vec<String> = Vec::new();
        let mut modules = std::collections::BTreeMap::new();

        for import in &self.missing_imports {
            if import.starts_with("crate::") {
                let path = &import[7..]; // Remove "crate::"
                let parts: Vec<&str> = path.split("::").collect();
                
                // Build module hierarchy
                for i in 0..parts.len() {
                    let module_path = parts[..=i].join("::");
                    let part = parts[i];
                    
                    if i == parts.len() - 1 {
                        // Last part - could be struct or module
                        if part.chars().next().unwrap().is_uppercase() {
                            // Likely a struct/type
                            let parent_path = if i > 0 {
                                parts[..i].join("::")
                            } else {
                                String::new()
                            };
                            modules.entry(parent_path).or_insert_with(Vec::new)
                                .push(format!("pub struct {};", part));
                        } else {
                            // Module
                            modules.entry(module_path).or_insert_with(Vec::new);
                        }
                    } else {
                        // Intermediate module
                        modules.entry(module_path).or_insert_with(Vec::new);
                    }
                }
            }
        }

        // Generate nested module structure
        self.generate_nested_modules(&modules, "", 0)
    }

    fn generate_nested_modules(
        &self,
        modules: &std::collections::BTreeMap<String, Vec<String>>,
        prefix: &str,
        depth: usize,
    ) -> String {
        let mut result = String::new();
        let indent = "    ".repeat(depth);

        for (path, items) in modules {
            if path.starts_with(prefix) && path.matches("::").count() == depth {
                let module_name = if depth == 0 {
                    path.clone()
                } else {
                    path.split("::").last().unwrap().to_string()
                };

                result.push_str(&format!("{}pub mod {} {{\n", indent, module_name));
                
                // Add items for this module
                for item in items {
                    result.push_str(&format!("{}    {}\n", indent, item));
                }

                // Add nested modules
                let nested_prefix = if prefix.is_empty() {
                    path.clone()
                } else {
                    format!("{}::{}", prefix, module_name)
                };
                result.push_str(&self.generate_nested_modules(modules, &nested_prefix, depth + 1));
                
                result.push_str(&format!("{}}}\n", indent));
            }
        }

        result
    }

    /// Process all files in a directory recursively
    pub fn process_directory(&mut self, dir: &Path) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.process_directory(&path)?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Err(e) = self.process_file(&path) {
                    eprintln!("Warning: Failed to process {}: {}", path.display(), e);
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut generator = StubGenerator::new();
    
    // Process the src directory
    let src_dir = Path::new("src");
    if src_dir.exists() {
        generator.process_directory(src_dir)?;
    }

    println!("Found {} missing imports", generator.missing_imports.len());
    
    // Generate stubs
    let stubs = generator.generate_stubs();
    
    println!("// Generated stub modules:");
    println!("{}", stubs);
    
    // Optionally write to a file
    fs::write("generated_stubs.rs", &stubs)?;
    println!("Stubs written to generated_stubs.rs");
    
    Ok(())
}
