use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the complete analysis
    let analysis = fs::read_to_string("rustc_complete_analysis.txt")?;
    
    // Parse unique functions from analysis
    let mut functions = HashMap::new();
    let mut call_counts = HashMap::new();
    
    for line in analysis.lines() {
        if line.contains("📍") && line.contains(":") {
            // Extract function name from lines like "📍 core::int_log10::i128: 1stmts[call_u128]"
            if let Some(func_part) = line.split("📍").nth(1) {
                if let Some(func_name) = func_part.split(":").next() {
                    let func_name = func_name.trim();
                    functions.insert(func_name.to_string(), true);
                }
            }
        }
        
        // Count function calls from summary
        if line.contains("x ") && !line.contains("📊") && !line.contains("🔥") {
            if let Some(count_part) = line.split("x ").next() {
                if let Ok(count) = count_part.trim().parse::<u32>() {
                    if let Some(func_name) = line.split("x ").nth(1) {
                        let func_name = func_name.trim();
                        call_counts.insert(func_name.to_string(), count);
                    }
                }
            }
        }
    }
    
    println!("🔧 Generating function macros for {} unique functions", functions.len());
    
    // Generate macro file
    let mut macro_file = fs::File::create("rustc_function_macros.rs")?;
    
    writeln!(macro_file, "//! Auto-generated function macros for rustc reconstruction")?;
    writeln!(macro_file, "//! Generated from complete rustc::main::main analysis")?;
    writeln!(macro_file, "//! Total functions: {}", functions.len())?;
    writeln!(macro_file)?;
    
    // Generate individual function macros
    for (func_name, _) in &functions {
        let macro_name = generate_macro_name(func_name);
        let call_count = call_counts.get(func_name).unwrap_or(&1);
        
        writeln!(macro_file, "/// Function: {} (called {}x)", func_name, call_count)?;
        writeln!(macro_file, "macro_rules! {} {{", macro_name)?;
        writeln!(macro_file, "    () => {{")?;
        writeln!(macro_file, "        // Placeholder for: {}", func_name)?;
        writeln!(macro_file, "        compile_error!(\"Function '{}' not implemented\");", func_name)?;
        writeln!(macro_file, "    }};")?;
        writeln!(macro_file, "    ($impl:item) => {{")?;
        writeln!(macro_file, "        $impl")?;
        writeln!(macro_file, "    }};")?;
        writeln!(macro_file, "}}")?;
        writeln!(macro_file)?;
    }
    
    // Generate composition macro
    writeln!(macro_file, "/// Compose complete rustc from all function macros")?;
    writeln!(macro_file, "macro_rules! compose_rustc {{")?;
    writeln!(macro_file, "    () => {{")?;
    
    for (func_name, _) in &functions {
        let macro_name = generate_macro_name(func_name);
        writeln!(macro_file, "        {}!();", macro_name)?;
    }
    
    writeln!(macro_file, "    }};")?;
    writeln!(macro_file, "}}")?;
    
    // Generate usage example
    writeln!(macro_file)?;
    writeln!(macro_file, "/// Example usage:")?;
    writeln!(macro_file, "/// ```")?;
    writeln!(macro_file, "/// // Implement specific function")?;
    writeln!(macro_file, "/// core_int_log10_i128! {{")?;
    writeln!(macro_file, "///     fn core_int_log10_i128() -> i32 {{")?;
    writeln!(macro_file, "///         // implementation")?;
    writeln!(macro_file, "///     }}")?;
    writeln!(macro_file, "/// }}")?;
    writeln!(macro_file, "///")?;
    writeln!(macro_file, "/// // Compose entire rustc")?;
    writeln!(macro_file, "/// compose_rustc!();")?;
    writeln!(macro_file, "/// ```")?;
    
    println!("✅ Generated rustc_function_macros.rs with {} function macros", functions.len());
    
    Ok(())
}

fn generate_macro_name(func_name: &str) -> String {
    func_name
        .replace("::", "_")
        .replace(".", "_")
        .replace("-", "_")
        .replace(" ", "_")
        .replace("{", "")
        .replace("}", "")
        .replace("(", "")
        .replace(")", "")
        .replace("[", "")
        .replace("]", "")
        .replace(",", "_")
        .replace("___", "_")
        .replace("__", "_")
        .trim_matches('_')
        .to_lowercase()
}
