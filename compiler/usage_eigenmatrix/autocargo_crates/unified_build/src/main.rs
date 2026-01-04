// unified_build.rs - Complete replacement for old build.rs
// Combines: runbuild.rs + process_rustc_files.rs + export_symbol_map.rs + run_build.rs transformations
// This is the single expensive operation that generates all code

use split_decls_genesis::{
    symbol_resolver::{find_rust_files_in_dependency_fast, extract_all_symbols_from_file_fast, Symbol},
    dependency_extractor::get_rustc_dependencies,
    build_lib::*,
    transformations::*,
    transformation_tracker::TransformationTracker,
};
use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use serde_json;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Unified Build: Complete rustc processing pipeline");
    
    // Check cache first
    let cache_path = "build_cache.json";
    if Path::new(cache_path).exists() {
        if let Ok(cache_content) = fs::read_to_string(cache_path) {
            if let Ok(cache_data) = serde_json::from_str::<serde_json::Value>(&cache_content) {
                if let Some(timestamp) = cache_data.get("timestamp").and_then(|t| t.as_u64()) {
                    let cache_age = SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs() - timestamp;
                    
                    if cache_age < 3600 {
                        println!("💾 Found recent cache ({} seconds old), skipping processing", cache_age);
                        return Ok(());
                    }
                }
            }
        }
    }
    
    println!("🔧 Step 1: Processing rustc files with transformations...");
    process_rustc_files()?;
    
    println!("🗺️  Step 2: Exporting symbol map with dependencies...");
    export_symbol_map()?;
    
    println!("💾 Step 3: Creating build cache...");
    create_build_cache()?;
    
    println!("✅ Unified build complete!");
    Ok(())
}

fn process_rustc_files() -> Result<(), Box<dyn std::error::Error>> {
    let rustc_deps = get_rustc_dependencies().unwrap_or_default();
    let mut processed_count = 0;
    let mut total_files = 0;
    let mut tracker = TransformationTracker::new();
    
    // Count total files
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        total_files += rust_files.len();
    }
    
    println!("📊 Found {} total files to process", total_files);
    
    // Process each file with transformations
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        println!("📦 Processing {}: {} files", dep.name, rust_files.len());
        
        for file in rust_files {
            if let Ok(content) = fs::read_to_string(&file) {
                // Apply transformations with tracking
                let transformed = apply_transformations_with_tracking(&content, &file.to_string_lossy(), &mut tracker)?;
                
                // Write transformed file to submodules/
                let output_path = get_output_path(&file, &dep.name)?;
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&output_path, transformed)?;
                
                processed_count += 1;
                if processed_count % 100 == 0 {
                    println!("  📊 Processed {}/{} files ({:.1}%)", 
                        processed_count, total_files, 
                        (processed_count as f32 / total_files as f32) * 100.0);
                }
            }
        }
    }
    
    // Save audit log
    tracker.save_audit_log(Path::new("transformation_audit.log"))?;
    println!("{}", tracker.get_summary());
    println!("✅ Processed {} files with transformations", processed_count);
    Ok(())
}

fn apply_transformations_with_tracking(content: &str, file_path: &str, tracker: &mut TransformationTracker) -> Result<String, Box<dyn std::error::Error>> {
    let mut transformed = content.to_string();
    
    // Apply systematic transformations in order with tracking
    
    // 1. Strip malformed doc attributes first
    transformed = tracker.track("DOC_STRIP", "Strip malformed doc attributes", file_path, &transformed, strip_malformed_doc_attributes);
    
    // 2. Add bootstrap features for lib.rs files
    if file_path.ends_with("/lib.rs") {
        transformed = tracker.track("BOOTSTRAP_FEATURES", "Add compiler bootstrap features", file_path, &transformed, add_bootstrap_features);
    }
    
    // 3. Fix jobserver imports
    transformed = tracker.track("JOBSERVER_FIX", "Fix jobserver imports", file_path, &transformed, fix_jobserver_imports);
    
    // 4. Add missing type definitions
    transformed = tracker.track("TYPE_DEFS", "Add missing type definitions", file_path, &transformed, |c| add_missing_type_definitions(c, file_path));
    
    // 5. Add required imports
    transformed = tracker.track("IMPORTS", "Add required imports", file_path, &transformed, |c| add_required_imports(c, file_path));
    
    // 6. Fix attribute spacing: # [attr] → #[attr]
    transformed = tracker.track("ATTR_SPACING", "Fix attribute spacing", file_path, &transformed, |c| c.replace("# [", "#["));
    
    // 7. Handle newtype_index! macros
    transformed = tracker.track("NEWTYPE_INDEX", "Expand newtype_index macros", file_path, &transformed, |c| expand_newtype_index_macros(c).unwrap_or_else(|_| c.to_string()));
    
    // 8. Add source tracking comment
    let mut result = format!("// @source(file=\"{}\", generator=\"unified_build.rs\")\n", file_path);
    result.push_str(&transformed);
    
    Ok(result)
}

fn apply_transformations(content: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut transformed = content.to_string();
    
    // 1. Fix attribute spacing: # [attr] → #[attr]
    transformed = transformed.replace("# [", "#[");
    
    // 2. Handle newtype_index! macros
    transformed = expand_newtype_index_macros(&transformed)?;
    
    // 3. Add source tracking comment
    let mut result = format!("// SRC: {}\n// GENERATED BY: unified_build.rs (split-decls-genesis)\n\n", file_path);
    result.push_str(&transformed);
    
    Ok(result)
}

fn expand_newtype_index_macros(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    use regex::Regex;
    
    // Match rustc_index::newtype_index! { ... } or newtype_index! { ... }
    let re = Regex::new(r"(?:rustc_index::)?newtype_index!\s*\{([^{}]*(?:\{[^{}]*\}[^{}]*)*)\}")?;
    
    let mut result = content.to_string();
    
    for cap in re.captures_iter(content) {
        let full_match = cap.get(0).unwrap().as_str();
        let macro_body = cap.get(1).unwrap().as_str();
        
        // Extract struct name and attributes
        if let Some((struct_name, attributes)) = parse_newtype_index_body(macro_body) {
            let expanded = generate_newtype_index_expansion(&struct_name, &attributes);
            result = result.replace(full_match, &expanded);
        }
    }
    
    Ok(result)
}

fn parse_newtype_index_body(body: &str) -> Option<(String, Vec<String>)> {
    let mut struct_name = None;
    let mut attributes = Vec::new();
    
    for line in body.lines() {
        let line = line.trim();
        if line.starts_with("#[") {
            attributes.push(line.to_string());
        } else if line.starts_with("pub struct ") || line.starts_with("struct ") {
            if let Some(name) = line.split_whitespace().nth(if line.starts_with("pub") { 2 } else { 1 }) {
                struct_name = Some(name.trim_end_matches(" {").trim_end_matches("{").to_string());
            }
        }
    }
    
    struct_name.map(|name| (name, attributes))
}

fn generate_newtype_index_expansion(struct_name: &str, attributes: &[String]) -> String {
    let mut result = String::new();
    
    // Add attributes
    for attr in attributes {
        result.push_str(&format!("{}\n", attr));
    }
    
    // Generate the struct
    result.push_str(&format!("#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]\n"));
    result.push_str(&format!("pub struct {}(pub u32);\n\n", struct_name));
    
    // Implement Idx trait
    result.push_str(&format!("impl rustc_index::Idx for {} {{\n", struct_name));
    result.push_str("    fn new(idx: usize) -> Self {\n");
    result.push_str(&format!("        {}(idx as u32)\n", struct_name));
    result.push_str("    }\n");
    result.push_str("    \n");
    result.push_str("    fn index(self) -> usize {\n");
    result.push_str("        self.0 as usize\n");
    result.push_str("    }\n");
    result.push_str("}\n\n");
    
    // Add common implementations
    result.push_str(&format!("impl {} {{\n", struct_name));
    result.push_str("    pub const ZERO: Self = Self(0);\n");
    result.push_str("}\n\n");
    
    result.push_str(&format!("impl std::fmt::Debug for {} {{\n", struct_name));
    result.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    result.push_str(&format!("        write!(f, \"{}({{}})\", self.0)\n", struct_name));
    result.push_str("    }\n");
    result.push_str("}\n");
    
    result
}
    let mut result = content.to_string();
    
    // Add feature flags for rustc parsing
    if !result.contains("#![feature(try_blocks)]") {
        result = format!("#![feature(try_blocks)]\n{}", result);
    }
    
    // Apply transformations from run_build.rs
    result = add_prelude(&result);
    result = fix_file_paths(&result, file_path);
    result = fix_env_vars(&result);
    result = fix_attribute_spacing(&result);
    result = remove_crate_attrs(&result);  // Now selective - keeps doc attributes
    
    Ok(result)
}

fn export_symbol_map() -> Result<(), Box<dyn std::error::Error>> {
    let rustc_deps = get_rustc_dependencies().unwrap_or_default();
    let mut file_symbols: HashMap<String, Vec<Symbol>> = HashMap::new();
    let mut all_symbols: HashMap<String, Symbol> = HashMap::new();
    let mut symbol_usage_counts: HashMap<String, usize> = HashMap::new();
    let mut file_count = 0;
    let mut total_files = 0;
    
    // Count total files
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        total_files += rust_files.len();
    }
    
    println!("🔍 Step 2a: Extracting symbols from {} files...", total_files);
    
    // First pass: Extract all symbols
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        
        for file in rust_files {
            file_count += 1;
            if let Ok(symbols) = extract_all_symbols_from_file_fast(&file.to_string_lossy(), &dep.name) {
                for symbol in symbols {
                    let symbol_key = format!("{}::{}", symbol.crate_name, symbol.name);
                    all_symbols.insert(symbol_key.clone(), symbol.clone());
                    symbol_usage_counts.insert(symbol_key, 0); // Initialize usage count
                    file_symbols.entry(symbol.source_file.clone()).or_insert_with(Vec::new).push(symbol);
                }
            }
            
            if file_count % 200 == 0 {
                println!("  📊 Extracted symbols from {}/{} files ({:.1}%)", 
                    file_count, total_files, 
                    (file_count as f32 / total_files as f32) * 100.0);
            }
        }
    }
    
    println!("🔍 Step 2b: Counting symbol usage across {} files (parallel)...", total_files);
    
    // Collect all files for parallel processing
    let mut all_files = Vec::new();
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        all_files.extend(rust_files);
    }
    
    // Parallel usage counting
    let symbol_usage_counts = Arc::new(Mutex::new(symbol_usage_counts));
    let processed_count = Arc::new(Mutex::new(0usize));
    
    all_files.par_iter().for_each(|file| {
        if let Ok(content) = fs::read_to_string(file) {
            let mut local_counts = HashMap::new();
            
            // Count all symbols in this file
            for (symbol_key, _) in &all_symbols {
                if let Some(symbol_name) = symbol_key.split("::").last() {
                    let count = content.matches(symbol_name).count();
                    if count > 0 {
                        local_counts.insert(symbol_key.clone(), count);
                    }
                }
            }
            
            // Merge local counts into global counts
            if !local_counts.is_empty() {
                let mut global_counts = symbol_usage_counts.lock().unwrap();
                for (symbol_key, count) in local_counts {
                    *global_counts.get_mut(&symbol_key).unwrap() += count;
                }
            }
        }
        
        // Progress reporting
        let mut count = processed_count.lock().unwrap();
        *count += 1;
        if *count % 200 == 0 {
            println!("  📊 Analyzed usage in {}/{} files ({:.1}%)", 
                *count, total_files, 
                (*count as f32 / total_files as f32) * 100.0);
        }
    });
    
    let symbol_usage_counts = Arc::try_unwrap(symbol_usage_counts).unwrap().into_inner().unwrap();
    
    // Build final symbol map with usage data
    let mut symbol_map: BTreeMap<String, serde_json::Value> = BTreeMap::new();
    let mut dependency_counts: Vec<(String, usize)> = Vec::new();
    
    for (file_path, symbols) in &file_symbols {
        let mut total_deps = 0;
        for symbol in symbols {
            total_deps += symbol.dependencies.len();
            
            let symbol_key = format!("{}::{}", symbol.crate_name, symbol.name);
            let usage_count = symbol_usage_counts.get(&symbol_key).unwrap_or(&0);
            
            let symbol_data = serde_json::json!({
                "name": symbol.name,
                "symbol_type": symbol.symbol_type,
                "source_file": symbol.source_file,
                "crate_name": symbol.crate_name,
                "dependencies": symbol.dependencies,
                "usage_count": usage_count
            });
            
            symbol_map.insert(symbol_key, symbol_data);
        }
        dependency_counts.push((file_path.clone(), total_deps));
    }
    
    // Sort by dependency count (0 dependencies first)
    dependency_counts.sort_by_key(|(_, count)| *count);
    
    // Write compressed symbol map
    let output_file = fs::File::create("symbol_map_original.json.gz")?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let json_output = serde_json::to_string_pretty(&symbol_map)?;
    encoder.write_all(json_output.as_bytes())?;
    encoder.finish()?;
    
    // Calculate usage statistics
    let total_usage: usize = symbol_usage_counts.values().sum();
    let used_symbols = symbol_usage_counts.values().filter(|&&count| count > 0).count();
    
    println!("✅ Exported {} symbols to symbol_map_original.json.gz", symbol_map.len());
    println!("📊 Files with 0 dependencies: {}", 
        dependency_counts.iter().filter(|(_, count)| *count == 0).count());
    println!("📊 Symbols with usage: {}/{} ({:.1}%)", 
        used_symbols, symbol_map.len(),
        (used_symbols as f32 / symbol_map.len() as f32) * 100.0);
    println!("📊 Total symbol usages found: {}", total_usage);
    
    Ok(())
}

fn create_build_cache() -> Result<(), Box<dyn std::error::Error>> {
    let cache_data = serde_json::json!({
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        "version": "unified_build_v1",
        "status": "complete"
    });
    
    fs::write("build_cache.json", serde_json::to_string_pretty(&cache_data)?)?;
    Ok(())
}

fn get_output_path(input_path: &Path, crate_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Convert input path to output path in submodules/
    let relative_path = input_path.strip_prefix("../rust/")?;
    Ok(PathBuf::from("submodules/rust").join(relative_path))
}

// Transformation functions (from run_build.rs)
fn add_prelude(content: &str) -> String {
    format!("// Generated by unified_build.rs\n\n{}", content)
}

fn fix_file_paths(content: &str, _file_path: &str) -> String {
    content.replace("../", "")
}

fn fix_env_vars(content: &str) -> String {
    content.replace("env!(", "option_env!(")
}

fn fix_attribute_spacing(content: &str) -> String {
    content.replace("# [", "#[")
}

fn remove_crate_attrs(content: &str) -> String {
    content.lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            // Keep doc attributes but remove stability and other problematic attributes
            !trimmed.starts_with("#![") || 
            trimmed.starts_with("#![doc(") ||
            trimmed.starts_with("#![allow(") ||
            trimmed.starts_with("#![warn(") ||
            trimmed.starts_with("#![deny(")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
