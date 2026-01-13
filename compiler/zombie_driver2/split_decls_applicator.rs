use std::collections::HashMap;
use std::fs;
use syn::{parse_file, visit::Visit, ItemFn, ItemStruct, ItemEnum, ItemTrait, ItemImpl};
use serde_json::json;

#[derive(Debug, Clone)]
struct SplitLayer {
    layer_name: String,
    layer_type: LayerType,
    items: Vec<CodeItem>,
    dependencies: Vec<String>,
    io_signature: String,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum LayerType {
    Interface,    // Traits and public APIs
    Logic,        // Core business logic
    Data,         // Structs and data types
    IO,           // Input/output operations
    Error,        // Error handling
    Utils,        // Utility functions
}

#[derive(Debug, Clone)]
struct CodeItem {
    name: String,
    item_type: String,
    source_code: String,
    complexity: usize,
    dependencies: Vec<String>,
}

struct SplitDeclsVisitor {
    layers: HashMap<LayerType, Vec<CodeItem>>,
    current_file: String,
}

impl<'ast> Visit<'ast> for SplitDeclsVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let function_name = node.sig.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        
        // Classify function into layer
        let layer_type = classify_function_layer(&function_name, &source_code);
        
        let item = CodeItem {
            name: function_name,
            item_type: "function".to_string(),
            source_code: source_code.clone(),
            complexity: calculate_complexity_from_code(&source_code),
            dependencies: extract_function_dependencies(&source_code),
        };
        
        self.layers.entry(layer_type).or_default().push(item);
    }
    
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let struct_name = node.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        
        let item = CodeItem {
            name: struct_name,
            item_type: "struct".to_string(),
            source_code: source_code.clone(),
            complexity: 1,
            dependencies: extract_struct_dependencies(&source_code),
        };
        
        self.layers.entry(LayerType::Data).or_default().push(item);
    }
    
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        let enum_name = node.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        
        let item = CodeItem {
            name: enum_name,
            item_type: "enum".to_string(),
            source_code,
            complexity: node.variants.len(),
            dependencies: Vec::new(),
        };
        
        self.layers.entry(LayerType::Data).or_default().push(item);
    }
    
    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        let trait_name = node.ident.to_string();
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        
        let item = CodeItem {
            name: trait_name,
            item_type: "trait".to_string(),
            source_code,
            complexity: node.items.len(),
            dependencies: Vec::new(),
        };
        
        self.layers.entry(LayerType::Interface).or_default().push(item);
    }
    
    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let impl_name = if let syn::Type::Path(type_path) = &*node.self_ty {
            type_path.path.segments.last().unwrap().ident.to_string()
        } else {
            "Unknown".to_string()
        };
        
        let source_code = quote::ToTokens::to_token_stream(node).to_string();
        
        let item = CodeItem {
            name: format!("impl_{}", impl_name),
            item_type: "impl".to_string(),
            source_code,
            complexity: node.items.len(),
            dependencies: Vec::new(),
        };
        
        self.layers.entry(LayerType::Logic).or_default().push(item);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("✂️  SPLIT-DECLS APPLICATOR");
    println!("==========================");
    
    // Get personal repositories from previous analysis
    let personal_repos = get_target_repositories()?;
    println!("📁 Applying split-decls to {} repositories", personal_repos.len());
    
    let mut all_split_layers: HashMap<String, Vec<SplitLayer>> = HashMap::new();
    
    // Process each repository
    for (i, repo_path) in personal_repos.iter().enumerate() {
        println!("   Processing repo {}/{}: {}", i + 1, personal_repos.len(), repo_path);
        
        let rust_files = find_rust_files_in_repo(repo_path)?;
        let mut repo_layers: HashMap<LayerType, Vec<CodeItem>> = HashMap::new();
        
        // Process each Rust file in the repository
        for rust_file in &rust_files {
            if let Ok(content) = fs::read_to_string(rust_file) {
                if let Ok(syntax_tree) = parse_file(&content) {
                    let mut visitor = SplitDeclsVisitor {
                        layers: HashMap::new(),
                        current_file: rust_file.clone(),
                    };
                    
                    visitor.visit_file(&syntax_tree);
                    
                    // Merge layers from this file
                    for (layer_type, items) in visitor.layers {
                        repo_layers.entry(layer_type).or_default().extend(items);
                    }
                }
            }
        }
        
        // Convert to SplitLayer format
        let mut split_layers = Vec::new();
        for (layer_type, items) in repo_layers {
            if !items.is_empty() {
                let layer = SplitLayer {
                    layer_name: format!("{:?}", layer_type),
                    layer_type: layer_type.clone(),
                    items,
                    dependencies: Vec::new(),
                    io_signature: generate_io_signature(&layer_type),
                };
                split_layers.push(layer);
            }
        }
        
        all_split_layers.insert(repo_path.clone(), split_layers);
        
        // Generate split-decls files for this repository
        generate_split_decls_files(repo_path, &all_split_layers[repo_path])?;
    }
    
    // Generate summary report
    println!("\n📊 SPLIT-DECLS ANALYSIS:");
    println!("=========================");
    
    let mut total_layers = 0;
    let mut layer_type_counts: HashMap<String, usize> = HashMap::new();
    
    for (repo, layers) in &all_split_layers {
        total_layers += layers.len();
        println!("📂 {}: {} layers", repo, layers.len());
        
        for layer in layers {
            *layer_type_counts.entry(layer.layer_name.clone()).or_insert(0) += 1;
            println!("   {} ({}): {} items", 
                     layer.layer_name, 
                     layer.io_signature,
                     layer.items.len());
        }
    }
    
    println!("\n🏗️  LAYER TYPE DISTRIBUTION:");
    let mut sorted_types: Vec<_> = layer_type_counts.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));
    
    for (layer_type, count) in &sorted_types {
        println!("   {}: {} layers", layer_type, count);
    }
    
    // Generate comprehensive report
    let report = json!({
        "analysis_type": "split_decls_application",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "repositories_processed": personal_repos.len(),
            "total_layers_created": total_layers,
            "layer_types": sorted_types.iter().map(|(t, c)| json!({
                "type": t,
                "count": c
            })).collect::<Vec<_>>()
        },
        "repositories": all_split_layers.iter().map(|(repo, layers)| json!({
            "repository": repo,
            "layer_count": layers.len(),
            "layers": layers.iter().map(|layer| json!({
                "name": layer.layer_name,
                "type": format!("{:?}", layer.layer_type),
                "item_count": layer.items.len(),
                "io_signature": layer.io_signature,
                "sample_items": layer.items.iter().take(3).map(|item| json!({
                    "name": item.name,
                    "type": item.item_type,
                    "complexity": item.complexity
                })).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    });
    
    fs::write("split_decls_application_report.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Split-decls application report saved to: split_decls_application_report.json");
    
    Ok(())
}

fn get_target_repositories() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Focus on key repositories for split-decls application
    let target_repos = vec![
        "./nix/vendor/rust/cargo2nix/submodules/split-decls-rs",
        "./nix/vendor/rust/cargo2nix/submodules/split-decls-genesis",
        ".", // Current zombie driver
    ];
    
    Ok(target_repos.into_iter().map(|s| s.to_string()).collect())
}

fn find_rust_files_in_repo(repo_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rust_files = Vec::new();
    let full_path = if repo_path.starts_with("./") {
        format!("/mnt/data1/{}", &repo_path[2..])
    } else if repo_path == "." {
        "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_driver2".to_string()
    } else {
        format!("/mnt/data1/{}", repo_path)
    };
    
    if let Ok(entries) = fs::read_dir(&full_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                    rust_files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    
    // Limit to prevent overwhelming output
    rust_files.truncate(20);
    Ok(rust_files)
}

fn classify_function_layer(function_name: &str, source_code: &str) -> LayerType {
    // Classify based on function name and content patterns
    if function_name.contains("error") || function_name.contains("Error") || source_code.contains("Result<") {
        LayerType::Error
    } else if function_name.contains("read") || function_name.contains("write") || function_name.contains("parse") {
        LayerType::IO
    } else if function_name.starts_with("is_") || function_name.starts_with("has_") || function_name.contains("util") {
        LayerType::Utils
    } else if source_code.contains("impl ") || source_code.contains("trait ") {
        LayerType::Interface
    } else {
        LayerType::Logic
    }
}

fn generate_io_signature(layer_type: &LayerType) -> String {
    match layer_type {
        LayerType::Interface => "trait → impl".to_string(),
        LayerType::Logic => "data → result".to_string(),
        LayerType::Data => "() → struct".to_string(),
        LayerType::IO => "input → output".to_string(),
        LayerType::Error => "error → result".to_string(),
        LayerType::Utils => "value → value".to_string(),
    }
}

fn calculate_complexity_from_code(code: &str) -> usize {
    let mut complexity = 1;
    complexity += code.matches("if ").count();
    complexity += code.matches("match ").count();
    complexity += code.matches("for ").count();
    complexity += code.matches("while ").count();
    complexity
}

fn extract_function_dependencies(code: &str) -> Vec<String> {
    let mut deps = Vec::new();
    if code.contains("std::") { deps.push("std".to_string()); }
    if code.contains("serde") { deps.push("serde".to_string()); }
    if code.contains("syn::") { deps.push("syn".to_string()); }
    if code.contains("quote::") { deps.push("quote".to_string()); }
    deps
}

fn extract_struct_dependencies(code: &str) -> Vec<String> {
    let mut deps = Vec::new();
    if code.contains("Vec<") { deps.push("Vec".to_string()); }
    if code.contains("HashMap<") { deps.push("HashMap".to_string()); }
    if code.contains("String") { deps.push("String".to_string()); }
    deps
}

fn generate_split_decls_files(repo_path: &str, layers: &[SplitLayer]) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = format!("split_decls_output/{}", repo_path.replace("./", "").replace("/", "_"));
    fs::create_dir_all(&output_dir)?;
    
    for layer in layers {
        let layer_file = format!("{}/{}_layer.rs", output_dir, layer.layer_name.to_lowercase());
        let mut layer_content = format!(
            "// Split-Decls Layer: {}\n// IO Signature: {}\n// Generated from: {}\n\n",
            layer.layer_name, layer.io_signature, repo_path
        );
        
        for item in &layer.items {
            layer_content.push_str(&format!("// {}: {}\n", item.item_type, item.name));
            layer_content.push_str(&item.source_code);
            layer_content.push_str("\n\n");
        }
        
        fs::write(&layer_file, layer_content)?;
    }
    
    println!("   ✂️  Generated {} split-decls layers in {}", layers.len(), output_dir);
    Ok(())
}
