use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analysis = fs::read_to_string("rustc_complete_analysis.txt")?;
    
    // Parse function hierarchy from indentation
    let mut lattice_levels: Vec<Vec<String>> = Vec::new();
    let mut current_level = 0;
    let mut functions_by_level = HashMap::new();
    
    for line in analysis.lines() {
        if line.contains("📍") {
            // Count indentation to determine lattice level
            let indent_level = line.chars().take_while(|&c| c == ' ').count() / 2;
            
            if let Some(func_part) = line.split("📍").nth(1) {
                if let Some(func_name) = func_part.split(":").next() {
                    let func_name = func_name.trim().to_string();
                    
                    // Ensure we have enough levels
                    while lattice_levels.len() <= indent_level {
                        lattice_levels.push(Vec::new());
                    }
                    
                    lattice_levels[indent_level].push(func_name.clone());
                    functions_by_level.insert(func_name, indent_level);
                }
            }
        }
    }
    
    // Generate lattice structure
    let mut output = fs::File::create("rustc_lattice_structure.txt")?;
    
    writeln!(output, "🔬 RUSTC DEPENDENCY LATTICE - CRYSTAL STRUCTURE")?;
    writeln!(output, "==============================================")?;
    writeln!(output, "Lattice Depth: {} levels", lattice_levels.len())?;
    writeln!(output, "Structure: Tree → DAG → Lattice → Crystal")?;
    writeln!(output)?;
    
    for (level, functions) in lattice_levels.iter().enumerate() {
        if functions.is_empty() { continue; }
        
        let level_type = match level {
            0 => "🌱 LEAF LEVEL (Standalone - No Dependencies)",
            1 => "🔧 CORE LEVEL (Basic Operations)", 
            2 => "🏗️ BUILD LEVEL (Construction)",
            3 => "🎯 LOGIC LEVEL (Business Logic)",
            4 => "🚀 MAIN LEVEL (Entry Points)",
            _ => "📊 COMPOSITE LEVEL"
        };
        
        writeln!(output, "LEVEL {} - {} ({} functions)", level, level_type, functions.len())?;
        writeln!(output, "├─ Lattice Position: {}/{}", level, lattice_levels.len() - 1)?;
        writeln!(output, "├─ Crystal Layer: {}", level)?;
        writeln!(output, "└─ Functions:")?;
        
        for (idx, func) in functions.iter().enumerate() {
            let prefix = if idx == functions.len() - 1 { "   └─" } else { "   ├─" };
            writeln!(output, "{} {}", prefix, func)?;
        }
        writeln!(output)?;
    }
    
    // Generate build order (bottom-up)
    writeln!(output, "🏗️ CRYSTAL CONSTRUCTION ORDER (Bottom → Top)")?;
    writeln!(output, "===========================================")?;
    
    for (level, functions) in lattice_levels.iter().enumerate() {
        if functions.is_empty() { continue; }
        
        writeln!(output, "// Crystal Layer {}: Build {} functions", level, functions.len())?;
        for func in functions {
            let macro_name = func.replace("::", "_").replace(".", "_").to_lowercase();
            writeln!(output, "{}!(); // Level {}", macro_name, level)?;
        }
        writeln!(output)?;
    }
    
    // Generate macro lattice
    writeln!(output, "🔮 MACRO LATTICE COMPOSITION")?;
    writeln!(output, "============================")?;
    writeln!(output, "macro_rules! build_rustc_lattice {{")?;
    writeln!(output, "    () => {{")?;
    
    for (level, functions) in lattice_levels.iter().enumerate() {
        if functions.is_empty() { continue; }
        writeln!(output, "        // Lattice Level {}", level)?;
        for func in functions {
            let macro_name = func.replace("::", "_").replace(".", "_").to_lowercase();
            writeln!(output, "        {}!();", macro_name)?;
        }
        writeln!(output)?;
    }
    
    writeln!(output, "    }};")?;
    writeln!(output, "}}")?;
    
    println!("✅ Generated rustc_lattice_structure.txt");
    println!("🔬 Crystal has {} levels", lattice_levels.len());
    
    for (level, functions) in lattice_levels.iter().enumerate() {
        if !functions.is_empty() {
            println!("   Level {}: {} functions", level, functions.len());
        }
    }
    
    Ok(())
}
