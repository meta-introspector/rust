// export_symbol_map.rs - Export complete symbol map as JSON with dependency ordering
// Fixed to sort files by dependency count (0 dependencies first)

use split_decls_genesis::{
    symbol_resolver::{find_rust_files_in_dependency_fast, extract_all_symbols_from_file_fast, Symbol},
    dependency_extractor::get_rustc_dependencies
};
use std::collections::{HashMap, BTreeMap};
use serde_json;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

fn main() {
    println!("🗺️  Exporting complete symbol map to JSON with dependency ordering...");
    
    let rustc_deps = get_rustc_dependencies().unwrap_or_default();
    let mut file_symbols: HashMap<String, Vec<Symbol>> = HashMap::new();
    let mut file_count = 0;
    let mut total_files = 0;
    
    // Count total files first
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        total_files += rust_files.len();
    }
    println!("📊 Found {} total files to process", total_files);
    
    // Group symbols by source file
    for dep in &rustc_deps {
        let rust_files = find_rust_files_in_dependency_fast(&dep.path);
        println!("📦 {}: {} files", dep.name, rust_files.len());
        
        for file in rust_files {
            file_count += 1;
            if let Ok(symbols) = extract_all_symbols_from_file_fast(&file.to_string_lossy(), &dep.name) {
                for symbol in symbols {
                    file_symbols.entry(symbol.source_file.clone()).or_insert_with(Vec::new).push(symbol);
                }
            }
            
            if file_count % 50 == 0 {
                println!("  📊 Processed {}/{} files ({:.1}%)", file_count, total_files, (file_count as f32 / total_files as f32) * 100.0);
            }
        }
    }
    
    println!("✅ Grouped symbols from {} files", file_symbols.len());
    
    println!("📊 Calculating dependencies per file...");
    // Calculate total dependencies per file
    let mut file_deps: Vec<(String, usize)> = file_symbols.iter()
        .map(|(file, symbols)| {
            let total_deps: usize = symbols.iter().map(|s| s.dependencies.len()).sum();
            (file.clone(), total_deps)
        })
        .collect();
    
    println!("🔄 Sorting files by dependency count...");
    // Sort by dependency count (0 first, then ascending)
    file_deps.sort_by_key(|(_, deps)| *deps);
    
    println!("📊 Dependency distribution:");
    let zero_dep_files = file_deps.iter().filter(|(_, deps)| *deps == 0).count();
    println!("  - {} files with 0 dependencies", zero_dep_files);
    if let Some((_, max_deps)) = file_deps.last() {
        println!("  - Max dependencies: {}", max_deps);
    }
    
    println!("🏗️  Creating ordered symbol map...");
    // Create ordered symbol map
    let mut ordered_symbols = BTreeMap::new();
    let mut processed_files = 0;
    
    for (file, _) in &file_deps {
        if let Some(symbols) = file_symbols.get(file) {
            for symbol in symbols {
                let filename = std::path::Path::new(&symbol.source_file)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                let qualified_name = format!("{}::{}::{}", symbol.crate_name, filename, symbol.name);
                ordered_symbols.insert(qualified_name, symbol);
            }
        }
        processed_files += 1;
        if processed_files % 100 == 0 {
            println!("  📊 Processed {}/{} files for symbol ordering", processed_files, file_deps.len());
        }
    }
    
    println!("💾 Exporting to JSON...");
    // Export to JSON
    let json_output = serde_json::to_string_pretty(&ordered_symbols).expect("Failed to serialize symbols");
    std::fs::write("symbol_map.json", json_output).expect("Failed to write symbol_map.json");
    
    println!("🗜️  Compressing to .gz format...");
    // Also create compressed version
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    
    let json_output_gz = serde_json::to_string(&ordered_symbols).expect("Failed to serialize symbols for gz");
    let file = std::fs::File::create("symbol_map.json.gz").expect("Failed to create gz file");
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(json_output_gz.as_bytes()).expect("Failed to write gz data");
    encoder.finish().expect("Failed to finish gz compression");
    
    // Export summary stats
    let mut stats = HashMap::new();
    stats.insert("total_symbols", ordered_symbols.len());
    stats.insert("total_files", file_symbols.len());
    
    let mut crate_counts = HashMap::new();
    let mut type_counts = HashMap::new();
    
    for symbol in ordered_symbols.values() {
        *crate_counts.entry(symbol.crate_name.clone()).or_insert(0) += 1;
        *type_counts.entry(symbol.symbol_type.clone()).or_insert(0) += 1;
    }
    
    let summary = serde_json::json!({
        "stats": stats,
        "crate_distribution": crate_counts,
        "type_distribution": type_counts,
        "dependency_ordered_files": file_deps.iter().take(20).collect::<Vec<_>>()
    });
    
    let summary_json = serde_json::to_string_pretty(&summary).expect("Failed to serialize summary");
    std::fs::write("symbol_summary.json", summary_json).expect("Failed to write symbol_summary.json");
    
    println!("📄 Exported symbol_map.json ({} symbols)", ordered_symbols.len());
    println!("📦 Exported symbol_map.json.gz (compressed)");
    println!("📊 Exported symbol_summary.json (stats)");
    
    // Show first few files (should be 0 dependency files)
    println!("\n🔍 First files in dependency order:");
    for (file, deps) in file_deps.iter().take(5) {
        println!("  {} dependencies: {}", deps, file);
    }
}
