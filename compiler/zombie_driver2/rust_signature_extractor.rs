use std::collections::HashMap;
use std::fs;
use syn::{parse_file, visit::Visit, Type, FnArg, ReturnType};
use serde_json::json;

#[derive(Debug, Clone)]
struct RustSignature {
    file_path: String,
    function_name: String,
    input_types: Vec<String>,
    output_types: Vec<String>,
    complexity_score: usize,
    line_count: usize,
    char_frequency: HashMap<char, usize>,
    dependencies: Vec<String>,
    repository: String,
}

#[derive(Debug, Default)]
struct IOMatrixEntry {
    signatures: Vec<RustSignature>,
    frequency: usize,
    avg_complexity: f64,
    repositories: Vec<String>,
}

struct SignatureExtractor {
    signatures: Vec<RustSignature>,
    current_file: String,
    current_repo: String,
}

impl<'ast> Visit<'ast> for SignatureExtractor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let function_name = node.sig.ident.to_string();
        
        // Extract input types
        let mut input_types = Vec::new();
        for input in &node.sig.inputs {
            match input {
                FnArg::Typed(pat_type) => {
                    input_types.push(type_to_string(&pat_type.ty));
                }
                FnArg::Receiver(_) => {
                    input_types.push("self".to_string());
                }
            }
        }
        
        // Extract output types
        let mut output_types = Vec::new();
        match &node.sig.output {
            ReturnType::Default => output_types.push("()".to_string()),
            ReturnType::Type(_, ty) => output_types.push(type_to_string(ty)),
        }
        
        // Calculate metrics
        let function_code = quote::ToTokens::to_token_stream(node).to_string();
        let line_count = function_code.lines().count();
        let complexity_score = calculate_complexity(&function_code);
        let char_frequency = calculate_char_frequency(&function_code);
        let dependencies = extract_dependencies(&function_code);
        
        let signature = RustSignature {
            file_path: self.current_file.clone(),
            function_name,
            input_types,
            output_types,
            complexity_score,
            line_count,
            char_frequency,
            dependencies,
            repository: self.current_repo.clone(),
        };
        
        self.signatures.push(signature);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 RUST SIGNATURE EXTRACTOR & IO MATRIX ANALYZER");
    println!("=================================================");
    
    // Get personal repositories from previous analysis
    let personal_repos = get_personal_rust_repos()?;
    println!("📁 Found {} personal Rust repositories", personal_repos.len());
    
    let mut all_signatures = Vec::new();
    let mut io_matrix: HashMap<String, IOMatrixEntry> = HashMap::new();
    
    // Process each personal repository
    for (i, repo_path) in personal_repos.iter().enumerate() {
        if i % 10 == 0 {
            println!("   Processing repo {}/{}: {}", i + 1, personal_repos.len(), repo_path);
        }
        
        let rust_files = find_rust_files_in_repo(repo_path)?;
        
        for rust_file in &rust_files {
            if let Ok(content) = fs::read_to_string(rust_file) {
                if let Ok(syntax_tree) = parse_file(&content) {
                    let mut extractor = SignatureExtractor {
                        signatures: Vec::new(),
                        current_file: rust_file.clone(),
                        current_repo: repo_path.clone(),
                    };
                    
                    extractor.visit_file(&syntax_tree);
                    
                    // Add signatures to collection
                    for signature in extractor.signatures {
                        // Create IO matrix entries
                        for input_type in &signature.input_types {
                            for output_type in &signature.output_types {
                                let io_key = format!("{} → {}", input_type, output_type);
                                let entry = io_matrix.entry(io_key).or_default();
                                
                                entry.signatures.push(signature.clone());
                                entry.frequency += 1;
                                entry.repositories.push(repo_path.clone());
                            }
                        }
                        
                        all_signatures.push(signature);
                    }
                }
            }
        }
    }
    
    // Calculate averages for IO matrix
    for entry in io_matrix.values_mut() {
        if !entry.signatures.is_empty() {
            entry.avg_complexity = entry.signatures.iter()
                .map(|s| s.complexity_score as f64)
                .sum::<f64>() / entry.signatures.len() as f64;
            
            // Remove duplicates from repositories
            entry.repositories.sort();
            entry.repositories.dedup();
        }
    }
    
    // Generate analysis report
    println!("\n📊 SIGNATURE ANALYSIS RESULTS:");
    println!("==============================");
    println!("Total signatures extracted: {}", all_signatures.len());
    println!("Unique IO patterns: {}", io_matrix.len());
    
    // Top IO patterns
    println!("\n🔄 TOP IO PATTERNS:");
    let mut sorted_patterns: Vec<_> = io_matrix.iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.frequency.cmp(&a.1.frequency));
    
    for (i, (pattern, entry)) in sorted_patterns.iter().take(15).enumerate() {
        println!("{}. {} (freq: {}, avg complexity: {:.1}, repos: {})", 
                 i + 1, pattern, entry.frequency, entry.avg_complexity, entry.repositories.len());
    }
    
    // Analyze by repository
    println!("\n📂 SIGNATURES BY REPOSITORY:");
    let mut repo_signatures: HashMap<String, Vec<&RustSignature>> = HashMap::new();
    for signature in &all_signatures {
        repo_signatures.entry(signature.repository.clone())
            .or_default()
            .push(signature);
    }
    
    let mut sorted_repos: Vec<_> = repo_signatures.iter().collect();
    sorted_repos.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (i, (repo, signatures)) in sorted_repos.iter().take(10).enumerate() {
        let avg_complexity = signatures.iter()
            .map(|s| s.complexity_score as f64)
            .sum::<f64>() / signatures.len() as f64;
        
        println!("{}. {} ({} signatures, avg complexity: {:.1})", 
                 i + 1, repo, signatures.len(), avg_complexity);
    }
    
    // Focus on split-decls repositories
    println!("\n✂️  SPLIT-DECLS SIGNATURE ANALYSIS:");
    let split_decls_signatures: Vec<_> = all_signatures.iter()
        .filter(|s| s.repository.contains("split") && s.repository.contains("decl"))
        .collect();
    
    println!("Split-decls signatures: {}", split_decls_signatures.len());
    
    if !split_decls_signatures.is_empty() {
        let mut split_decls_patterns: HashMap<String, usize> = HashMap::new();
        for signature in &split_decls_signatures {
            for input_type in &signature.input_types {
                for output_type in &signature.output_types {
                    let pattern = format!("{} → {}", input_type, output_type);
                    *split_decls_patterns.entry(pattern).or_insert(0) += 1;
                }
            }
        }
        
        let mut sorted_split_patterns: Vec<_> = split_decls_patterns.iter().collect();
        sorted_split_patterns.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("Top split-decls patterns:");
        for (pattern, count) in sorted_split_patterns.iter().take(10) {
            println!("   {}: {}", pattern, count);
        }
    }
    
    // Generate comprehensive JSON report
    let report = json!({
        "analysis_type": "rust_signature_io_matrix",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "total_signatures": all_signatures.len(),
            "unique_io_patterns": io_matrix.len(),
            "repositories_analyzed": personal_repos.len(),
            "split_decls_signatures": split_decls_signatures.len()
        },
        "top_io_patterns": sorted_patterns.iter().take(20).map(|(pattern, entry)| json!({
            "pattern": pattern,
            "frequency": entry.frequency,
            "avg_complexity": entry.avg_complexity,
            "repositories": entry.repositories.len(),
            "sample_functions": entry.signatures.iter().take(3).map(|s| json!({
                "function": s.function_name,
                "file": s.file_path,
                "complexity": s.complexity_score
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "repository_analysis": sorted_repos.iter().take(15).map(|(repo, signatures)| json!({
            "repository": repo,
            "signature_count": signatures.len(),
            "avg_complexity": signatures.iter().map(|s| s.complexity_score as f64).sum::<f64>() / signatures.len() as f64,
            "unique_functions": signatures.len()
        })).collect::<Vec<_>>(),
        "split_decls_analysis": {
            "signature_count": split_decls_signatures.len(),
            "repositories": split_decls_signatures.iter().map(|s| s.repository.clone()).collect::<std::collections::HashSet<_>>().len()
        }
    });
    
    fs::write("rust_signature_io_matrix.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Rust signature & IO matrix analysis saved to: rust_signature_io_matrix.json");
    
    Ok(())
}

fn get_personal_rust_repos() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Get repositories that contain Rust code from our personal repo list
    let personal_indicators = vec![
        "split-decls", "zombie", "introspector", "rust", "cargo"
    ];
    
    let mut rust_repos = Vec::new();
    
    // Check recent repositories for Rust content
    let recent_paths = vec![
        "./nix/vendor/rust/cargo2nix/submodules/split-decls-rs",
        "./nix/vendor/rust/cargo2nix/submodules/split-decls-genesis", 
        "./nix/vendor/rust/cargo2nix/submodules/rust-build",
        "./nix/time/2025/08/07/rust-analyser-hf-dataset",
        "./nix/time/2025/08/07/hf-dataset-validator-rust",
        "./time2/time",
        ".",  // Current directory
    ];
    
    for path in recent_paths {
        if std::path::Path::new(&format!("/mnt/data1/{}", path.trim_start_matches("./"))).exists() {
            rust_repos.push(path.to_string());
        }
    }
    
    Ok(rust_repos)
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
                } else if path.is_dir() && !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                    if let Ok(mut subfiles) = find_rust_files_in_repo(&path.to_string_lossy()) {
                        rust_files.append(&mut subfiles);
                    }
                }
            }
        }
    }
    
    Ok(rust_files)
}

fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            type_path.path.segments.iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::")
        }
        Type::Reference(type_ref) => {
            format!("&{}", type_to_string(&type_ref.elem))
        }
        Type::Tuple(type_tuple) => {
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

fn calculate_complexity(code: &str) -> usize {
    let mut complexity = 1; // Base complexity
    
    // Count control flow statements
    complexity += code.matches("if ").count();
    complexity += code.matches("match ").count();
    complexity += code.matches("for ").count();
    complexity += code.matches("while ").count();
    complexity += code.matches("loop ").count();
    
    complexity
}

fn calculate_char_frequency(text: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}

fn extract_dependencies(code: &str) -> Vec<String> {
    let mut deps = Vec::new();
    
    // Look for common Rust dependencies
    let patterns = ["std::", "serde", "tokio", "syn::", "quote::", "HashMap", "Vec", "Result"];
    
    for pattern in &patterns {
        if code.contains(pattern) {
            deps.push(pattern.to_string());
        }
    }
    
    deps.sort();
    deps.dedup();
    deps
}
