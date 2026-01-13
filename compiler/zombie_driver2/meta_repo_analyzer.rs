use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 META-INTROSPECTOR REPO ANALYZER");
    println!("===================================");
    
    // Read repo list
    let repos_content = fs::read_to_string("/mnt/data1/meta-introspector/repos.txt")?;
    let repos: Vec<&str> = repos_content.lines().collect();
    
    println!("📁 Found {} repositories to analyze", repos.len());
    
    for repo_path in &repos {
        let expanded_path = if repo_path.starts_with("~/") {
            repo_path.replace("~", "/home/mdupont")
        } else {
            repo_path.to_string()
        };
        
        println!("\n🔬 Analyzing: {}", expanded_path);
        
        if Path::new(&expanded_path).exists() {
            // Run our comprehensive analysis on each repo
            analyze_repo(&expanded_path)?;
        } else {
            println!("⚠️  Path not found: {}", expanded_path);
        }
    }
    
    Ok(())
}

fn analyze_repo(repo_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Find Rust files in the repo
    let rust_files = find_rust_files(repo_path)?;
    println!("   📄 Found {} Rust files", rust_files.len());
    
    if rust_files.is_empty() {
        return Ok(());
    }
    
    // Run our analysis tools on this repo
    let repo_name = Path::new(repo_path).file_name().unwrap().to_string_lossy();
    
    // 1. Basic Block Analysis
    println!("   🧬 Running basic block analysis...");
    run_basic_block_analysis(repo_path, &rust_files)?;
    
    // 2. IO Matrix Analysis  
    println!("   🔄 Running IO matrix analysis...");
    run_io_matrix_analysis(repo_path, &rust_files)?;
    
    // 3. Unknown Type Classification
    println!("   🔍 Running unknown type classification...");
    run_unknown_type_analysis(repo_path, &rust_files)?;
    
    // 4. Look for split-decls patterns
    if repo_name.contains("split-decls") {
        println!("   ✂️  Analyzing split-decls patterns...");
        analyze_split_decls_patterns(repo_path, &rust_files)?;
    }
    
    println!("   ✅ Analysis complete for {}", repo_name);
    Ok(())
}

fn find_rust_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    
    if !Path::new(dir).exists() {
        return Ok(files);
    }
    
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            files.push(path.to_string_lossy().to_string());
        } else if path.is_dir() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
            if let Ok(mut subfiles) = find_rust_files(&path.to_string_lossy()) {
                files.append(&mut subfiles);
            }
        }
    }
    
    Ok(files)
}

fn run_basic_block_analysis(repo_path: &str, rust_files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    // Simplified basic block analysis for each repo
    let mut total_functions = 0;
    let mut total_chars = 0;
    
    for file_path in rust_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            total_chars += content.len();
            
            if let Ok(syntax_tree) = syn::parse_file(&content) {
                for item in &syntax_tree.items {
                    if let syn::Item::Fn(_) = item {
                        total_functions += 1;
                    }
                }
            }
        }
    }
    
    println!("     Functions: {}, Characters: {}", total_functions, total_chars);
    Ok(())
}

fn run_io_matrix_analysis(repo_path: &str, rust_files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    
    for file_path in rust_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(syntax_tree) = syn::parse_file(&content) {
                for item in &syntax_tree.items {
                    if let syn::Item::Fn(func) = item {
                        // Count parameter types
                        for input in &func.sig.inputs {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let type_str = type_to_string(&pat_type.ty);
                                *type_counts.entry(type_str).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    let unique_types = type_counts.len();
    println!("     Unique types: {}", unique_types);
    Ok(())
}

fn run_unknown_type_analysis(repo_path: &str, rust_files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut unknown_count = 0;
    
    for file_path in rust_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(syntax_tree) = syn::parse_file(&content) {
                for item in &syntax_tree.items {
                    if let syn::Item::Fn(func) = item {
                        for input in &func.sig.inputs {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let type_str = type_to_string(&pat_type.ty);
                                if type_str.contains("Unknown") {
                                    unknown_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("     Unknown types: {}", unknown_count);
    Ok(())
}

fn analyze_split_decls_patterns(repo_path: &str, rust_files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut split_patterns = Vec::new();
    
    for file_path in rust_files {
        if let Ok(content) = fs::read_to_string(file_path) {
            // Look for split-decls specific patterns
            if content.contains("split_decl") || content.contains("SplitDecl") {
                split_patterns.push(file_path.clone());
            }
            
            // Look for syn/quote usage
            if content.contains("syn::") && content.contains("quote::") {
                println!("     📝 Found syn/quote usage in: {}", 
                        Path::new(file_path).file_name().unwrap().to_string_lossy());
            }
        }
    }
    
    println!("     Split-decls patterns: {}", split_patterns.len());
    Ok(())
}

fn type_to_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => {
            type_path.path.segments.iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::")
        }
        syn::Type::Reference(type_ref) => {
            format!("&{}", type_to_string(&type_ref.elem))
        }
        syn::Type::Tuple(type_tuple) => {
            if type_tuple.elems.is_empty() {
                "()".to_string()
            } else {
                let elem_types: Vec<String> = type_tuple.elems.iter()
                    .map(type_to_string)
                    .collect();
                format!("({})", elem_types.join(", "))
            }
        }
        _ => "Unknown".to_string(),
    }
}
