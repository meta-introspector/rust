use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lattice = fs::read_to_string("rustc_lattice_structure.txt")?;
    
    // Parse layers and their functions
    let mut layers = HashMap::new();
    let mut current_level = None;
    
    for line in lattice.lines() {
        if line.starts_with("LEVEL ") {
            if let Some(level_str) = line.split("LEVEL ").nth(1) {
                if let Some(level_num) = level_str.split(" ").next() {
                    current_level = level_num.parse::<usize>().ok();
                }
            }
        } else if line.contains("├─") || line.contains("└─") {
            if let Some(level) = current_level {
                if let Some(func_name) = line.split("─ ").nth(1) {
                    let func_name = func_name.trim().to_string();
                    layers.entry(level).or_insert_with(Vec::new).push(func_name);
                }
            }
        }
    }
    
    // Generate parameterized layer macros
    let mut output = fs::File::create("rustc_layer_macros.rs")?;
    
    writeln!(output, "//! Parameterized Layer Macros for Rustc Crystal")?;
    writeln!(output, "//! Each layer takes its dependencies as macro arguments")?;
    writeln!(output)?;
    
    for level in 0..=41 {
        if let Some(functions) = layers.get(&level) {
            writeln!(output, "/// Layer {} - {} functions", level, functions.len())?;
            writeln!(output, "/// Dependencies: Layers 0..{}", level)?;
            
            // Generate macro with dependency parameters
            let deps = if level == 0 { 
                String::new() 
            } else { 
                (0..level).map(|i| format!("$layer_{}:expr", i)).collect::<Vec<_>>().join(", ")
            };
            
            writeln!(output, "macro_rules! rustc_layer_{} {{", level)?;
            
            if level == 0 {
                writeln!(output, "    () => {{")?;
            } else {
                writeln!(output, "    ({}) => {{", deps)?;
            }
            
            writeln!(output, "        // Layer {} implementation", level)?;
            for func in functions {
                let macro_name = func.replace("::", "_").replace(".", "_").to_lowercase();
                writeln!(output, "        {}!();", macro_name)?;
            }
            
            if level > 0 {
                writeln!(output, "        // Use dependencies:")?;
                for dep_level in 0..level {
                    writeln!(output, "        // $layer_{}", dep_level)?;
                }
            }
            
            writeln!(output, "    }};")?;
            writeln!(output, "}}")?;
            writeln!(output)?;
        }
    }
    
    // Generate composition macro
    writeln!(output, "/// Compose entire rustc crystal with layer dependencies")?;
    writeln!(output, "macro_rules! compose_rustc_crystal {{")?;
    writeln!(output, "    () => {{")?;
    
    for level in 0..=41 {
        if layers.contains_key(&level) {
            if level == 0 {
                writeln!(output, "        let layer_0 = rustc_layer_0!();")?;
            } else {
                let deps = (0..level).map(|i| format!("layer_{}", i)).collect::<Vec<_>>().join(", ");
                writeln!(output, "        let layer_{} = rustc_layer_{}!({});", level, level, deps)?;
            }
        }
    }
    
    writeln!(output, "        layer_41 // Return top layer (main)")?;
    writeln!(output, "    }};")?;
    writeln!(output, "}}")?;
    
    // Generate usage example
    writeln!(output)?;
    writeln!(output, "/// Example usage:")?;
    writeln!(output, "/// ```")?;
    writeln!(output, "/// // Build layer by layer with dependencies")?;
    writeln!(output, "/// let foundation = rustc_layer_0!();")?;
    writeln!(output, "/// let core = rustc_layer_1!(foundation);")?;
    writeln!(output, "/// let build = rustc_layer_2!(foundation, core);")?;
    writeln!(output, "/// // ...")?;
    writeln!(output, "/// let complete_rustc = compose_rustc_crystal!();")?;
    writeln!(output, "/// ```")?;
    
    println!("✅ Generated rustc_layer_macros.rs");
    println!("🔮 Created {} parameterized layer macros", layers.len());
    
    Ok(())
}
