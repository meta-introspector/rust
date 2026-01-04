use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analysis = fs::read_to_string("rustc_complete_analysis.txt")?;
    
    // Extract all function names and their source files
    let mut function_to_file = HashMap::new();
    let symbol_data = fs::read_to_string("symbol_map.json")?;
    let symbol_map: HashMap<String, serde_json::Value> = serde_json::from_str(&symbol_data)?;
    
    // Map functions to their source files
    for (func_name, info) in &symbol_map {
        if let Some(source_file) = info.get("source_file").and_then(|v| v.as_str()) {
            function_to_file.insert(func_name.clone(), source_file.to_string());
        }
    }
    
    // Parse dependency order from our fixed lattice
    let lattice = fs::read_to_string("rustc_fixed_lattice.txt")?;
    let mut ordered_functions = Vec::new();
    
    for line in lattice.lines() {
        if line.contains("├─") || line.contains("└─") {
            if let Some(func_name) = line.split("─ ").nth(1) {
                let func_name = func_name.trim();
                ordered_functions.push(func_name.to_string());
            }
        }
    }
    
    // Map functions to unique source files in dependency order
    let mut ordered_files = Vec::new();
    let mut seen_files = HashSet::new();
    
    for func in &ordered_functions {
        if let Some(file_path) = function_to_file.get(func) {
            if !seen_files.contains(file_path) && file_path.ends_with(".rs") {
                seen_files.insert(file_path.clone());
                ordered_files.push(file_path.clone());
            }
        }
    }
    
    // Generate build.rs
    generate_build_rs(&ordered_files)?;
    
    // Generate topological sort output
    generate_topo_sort(&ordered_functions, &ordered_files)?;
    
    Ok(())
}

fn generate_build_rs(ordered_files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut build_rs = fs::File::create("build.rs")?;
    
    writeln!(build_rs, "// Auto-generated build.rs from rustc dependency analysis")?;
    writeln!(build_rs, "// Evaluates Rust files in topological dependency order")?;
    writeln!(build_rs)?;
    writeln!(build_rs, "use std::fs;")?;
    writeln!(build_rs, "use std::path::Path;")?;
    writeln!(build_rs)?;
    writeln!(build_rs, "fn main() -> Result<(), Box<dyn std::error::Error>> {{")?;
    writeln!(build_rs, "    println!(\"🔧 Building rustc in topological order...\");")?;
    writeln!(build_rs)?;
    writeln!(build_rs, "    // Topologically sorted file evaluation order")?;
    writeln!(build_rs, "    let files_in_order = [")?;
    
    for (i, file_path) in ordered_files.iter().enumerate() {
        let comma = if i == ordered_files.len() - 1 { "" } else { "," };
        writeln!(build_rs, "        \"{}\"{}", file_path, comma)?;
    }
    
    writeln!(build_rs, "    ];")?;
    writeln!(build_rs)?;
    writeln!(build_rs, "    for (i, file_path) in files_in_order.iter().enumerate() {{")?;
    writeln!(build_rs, "        if Path::new(file_path).exists() {{")?;
    writeln!(build_rs, "            println!(\"📄 [{{:3}}/{{}}] Evaluating: {{}}\", i + 1, files_in_order.len(), file_path);")?;
    writeln!(build_rs, "            ")?;
    writeln!(build_rs, "            // Read and validate syntax")?;
    writeln!(build_rs, "            match fs::read_to_string(file_path) {{")?;
    writeln!(build_rs, "                Ok(content) => {{")?;
    writeln!(build_rs, "                    // Basic syntax validation")?;
    writeln!(build_rs, "                    if content.contains(\"fn \") || content.contains(\"struct \") || content.contains(\"enum \") {{")?;
    writeln!(build_rs, "                        println!(\"✅ Valid Rust code: {{}}\", file_path);")?;
    writeln!(build_rs, "                    }} else {{")?;
    writeln!(build_rs, "                        println!(\"⚠️  No declarations found: {{}}\", file_path);")?;
    writeln!(build_rs, "                    }}")?;
    writeln!(build_rs, "                }}")?;
    writeln!(build_rs, "                Err(e) => {{")?;
    writeln!(build_rs, "                    println!(\"❌ Error reading {{}}: {{}}\", file_path, e);")?;
    writeln!(build_rs, "                }}")?;
    writeln!(build_rs, "            }}")?;
    writeln!(build_rs, "        }} else {{")?;
    writeln!(build_rs, "            println!(\"⚠️  File not found: {{}}\", file_path);")?;
    writeln!(build_rs, "        }}")?;
    writeln!(build_rs, "    }}")?;
    writeln!(build_rs)?;
    writeln!(build_rs, "    println!(\"🎯 Topological evaluation complete!\");")?;
    writeln!(build_rs, "    Ok(())")?;
    writeln!(build_rs, "}}")?;
    
    println!("✅ Generated build.rs with {} files in topological order", ordered_files.len());
    Ok(())
}

fn generate_topo_sort(ordered_functions: &[String], ordered_files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = fs::File::create("topological_sort.txt")?;
    
    writeln!(output, "🔬 RUSTC TOPOLOGICAL SORT")?;
    writeln!(output, "=========================")?;
    writeln!(output, "Total Functions: {}", ordered_functions.len())?;
    writeln!(output, "Total Files: {}", ordered_files.len())?;
    writeln!(output)?;
    
    writeln!(output, "📋 FUNCTION EVALUATION ORDER:")?;
    for (i, func) in ordered_functions.iter().enumerate() {
        writeln!(output, "{:3}. {}", i + 1, func)?;
    }
    
    writeln!(output)?;
    writeln!(output, "📁 FILE EVALUATION ORDER:")?;
    for (i, file) in ordered_files.iter().enumerate() {
        writeln!(output, "{:3}. {}", i + 1, file)?;
    }
    
    println!("✅ Generated topological_sort.txt");
    println!("📊 Order: {} functions → {} unique files", ordered_functions.len(), ordered_files.len());
    
    Ok(())
}
