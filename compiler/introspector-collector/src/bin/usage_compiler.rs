use std::env;
use std::fs;
use std::collections::HashMap;
use introspector_collector::libusagedata::load_crate_usage;
use introspector_collector::usage_code_map::compile_usage_to_executable;

fn get_mycelial_data_path() -> Result<String, Box<dyn std::error::Error>> {
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let parsed: toml::Value = toml::from_str(&cargo_toml)?;
    
    if let Some(metadata) = parsed.get("package").and_then(|p| p.get("metadata")) {
        if let Some(path) = metadata.get("mycelial_data_path").and_then(|p| p.as_str()) {
            return Ok(format!("{}/crate_usage_data", path));
        }
    }
    
    Err("mycelial_data_path not found in Cargo.toml metadata".into())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <output_file.rs>", args[0]);
        std::process::exit(1);
    }
    
    let output_file = &args[1];
    
    // Get data path from Cargo.toml metadata
    let data_path = match get_mycelial_data_path() {
        Ok(path) => {
            println!("📁 Data path from Cargo.toml: {}", path);
            path
        },
        Err(e) => {
            eprintln!("❌ Failed to read mycelial data path from Cargo.toml: {}", e);
            std::process::exit(1);
        }
    };
    
    // Load syn usage data from mycelial network
    println!("🔍 Looking for syn data in: {}", data_path);
    let crate_usage_data = match load_crate_usage("syn", &data_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("❌ Failed to load usage data: {}", e);
            std::process::exit(1);
        }
    };
    
    // Extract usage data from all crates
    let mut all_usage_data = Vec::new();
    for crate_data in &crate_usage_data {
        all_usage_data.extend(crate_data.usages.iter().cloned());
    }
    
    println!("📊 Loaded {} usage patterns from {} crates", all_usage_data.len(), crate_usage_data.len());
    
    // Compile usage patterns to executable code
    let generated_code = compile_usage_to_executable(&all_usage_data);
    
    // Write generated code to file
    match fs::write(output_file, generated_code) {
        Ok(_) => println!("✅ Generated executable code written to {}", output_file),
        Err(e) => {
            eprintln!("❌ Failed to write output file: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("🚀 Usage-to-code compilation complete!");
    println!("   Generated from {} usage patterns from {} crates", all_usage_data.len(), crate_usage_data.len());
    println!("   Output: {}", output_file);
}
