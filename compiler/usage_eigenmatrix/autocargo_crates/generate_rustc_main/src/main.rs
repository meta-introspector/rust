use std::fs;
use std::collections::HashMap;
use flate2::read::GzDecoder;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Generating rustcmain! macro with includes...");
    
    // Load symbol map
    let file = fs::File::open("symbol_map.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;
    let symbol_map: HashMap<String, serde_json::Value> = serde_json::from_str(&contents)?;
    
    // Filter for rustc_driver_impl::lib::main dependencies
    let target = "rustc_driver_impl::lib::main";
    let deps = resolve_dependencies(target, &symbol_map);
    
    println!("📊 Found {} dependencies for {}", deps.len(), target);
    
    // Generate the giant file
    let mut output = String::new();
    
    // Add includes for all dependencies
    output.push_str("// Auto-generated rustc main with all dependencies\n\n");
    
    for (i, dep) in deps.iter().enumerate() {
        if let Some(entry) = symbol_map.get(dep) {
            if let Some(source_file) = entry.get("source_file").and_then(|s| s.as_str()) {
                let processed_file = format!("processed_{}", source_file.replace("/", "_").replace(".rs", ".rs"));
                output.push_str(&format!("include!(\"{}\"); // {}\n", processed_file, dep));
            }
        }
    }
    
    // Add include_dep! macro definition
    output.push_str("// Define the include_dep! macro\n");
    output.push_str("macro_rules! include_dep {\n");
    output.push_str("    ($dep:expr, $file:expr) => {\n");
    output.push_str("        println!(\"Loading dependency: {}\", $dep);\n");
    output.push_str("        include!($file);\n");
    output.push_str("    };\n");
    output.push_str("}\n\n");
    
    // Generate rustcmain! macro with include-dep! calls
    output.push_str("\nmacro_rules! rustcmain {\n");
    output.push_str("    () => {\n");
    
    // Generate include-dep! calls for each dependency
    for dep in &deps {
        if let Some(entry) = symbol_map.get(dep) {
            if let Some(source_file) = entry.get("source_file").and_then(|s| s.as_str()) {
                let processed_file = format!("processed_{}", source_file.replace("/", "_").replace(".rs", ".rs"));
                output.push_str(&format!("        include_dep!(\"{}\", \"{}\");\n", dep, processed_file));
            }
        }
    }
    
    output.push_str("    };\n");
    output.push_str("}\n\n");
    
    // Add main function that uses the macro
    output.push_str("fn main() {\n");
    output.push_str("    rustcmain!();\n");
    output.push_str("}\n");
    
    // Write output
    fs::write("rustc_main_complete.rs", output)?;
    println!("✅ Generated rustc_main_complete.rs with {} includes", deps.len());
    
    Ok(())
}

fn resolve_dependencies(target: &str, symbol_map: &HashMap<String, serde_json::Value>) -> Vec<String> {
    let mut resolved = Vec::new();
    let mut visited = std::collections::HashSet::new();
    
    fn dfs(symbol: &str, map: &HashMap<String, serde_json::Value>, 
           resolved: &mut Vec<String>, visited: &mut std::collections::HashSet<String>) {
        if visited.contains(symbol) { return; }
        visited.insert(symbol.to_string());
        
        if let Some(entry) = map.get(symbol) {
            if let Some(deps) = entry.get("dependencies").and_then(|d| d.as_array()) {
                for dep in deps {
                    if let Some(dep_str) = dep.as_str() {
                        dfs(dep_str, map, resolved, visited);
                    }
                }
            }
        }
        
        resolved.push(symbol.to_string());
    }
    
    dfs(target, symbol_map, &mut resolved, &mut visited);
    resolved
}
