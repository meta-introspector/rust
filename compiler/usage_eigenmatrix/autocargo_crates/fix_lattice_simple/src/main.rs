use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analysis = fs::read_to_string("rustc_complete_analysis.txt")?;
    
    // Parse the actual call structure from the analysis
    let mut call_graph = HashMap::new();
    let mut all_functions = HashSet::new();
    let mut current_stack: Vec<String> = Vec::new();
    
    for line in analysis.lines() {
        if line.contains("📍") {
            let indent_level = line.chars().take_while(|&c| c == ' ').count() / 2;
            
            if let Some(func_part) = line.split("📍").nth(1) {
                if let Some(func_name) = func_part.split(":").next() {
                    let func_name = func_name.trim().to_string();
                    all_functions.insert(func_name.clone());
                    
                    // Adjust stack to current level
                    current_stack.truncate(indent_level);
                    
                    // Record what this function calls (its dependencies)
                    let calls = call_graph.entry(func_name.clone()).or_insert_with(Vec::new);
                    
                    // Add current stack as what this function calls
                    for called_func in &current_stack {
                        if !calls.contains(called_func) {
                            calls.push(called_func.clone());
                        }
                    }
                    
                    current_stack.push(func_name);
                }
            }
        }
    }
    
    // Calculate dependency levels - functions with no calls are level 0
    let mut levels = Vec::new();
    let mut assigned_levels = HashMap::new();
    
    // Find functions with no dependencies (leaves)
    let mut level_0 = Vec::new();
    for func in &all_functions {
        if call_graph.get(func).map_or(true, |calls| calls.is_empty()) {
            level_0.push(func.clone());
            assigned_levels.insert(func.clone(), 0);
        }
    }
    levels.push(level_0);
    
    // Iteratively assign levels
    let mut changed = true;
    while changed {
        changed = false;
        let mut next_level: Vec<String> = Vec::new();
        
        for func in &all_functions {
            if assigned_levels.contains_key(func) {
                continue; // Already assigned
            }
            
            // Check if all dependencies are assigned
            if let Some(calls) = call_graph.get(func) {
                let mut max_dep_level = None;
                let mut all_deps_assigned = true;
                
                for dep in calls {
                    if let Some(&dep_level) = assigned_levels.get(dep) {
                        max_dep_level = Some(max_dep_level.unwrap_or(0).max(dep_level));
                    } else {
                        all_deps_assigned = false;
                        break;
                    }
                }
                
                if all_deps_assigned {
                    let func_level = max_dep_level.unwrap_or(0) + 1;
                    assigned_levels.insert(func.clone(), func_level);
                    
                    // Ensure we have enough levels
                    while levels.len() <= func_level {
                        levels.push(Vec::new());
                    }
                    levels[func_level].push(func.clone());
                    changed = true;
                }
            }
        }
    }
    
    // Handle any remaining unassigned functions (cycles)
    let mut unassigned = Vec::new();
    for func in &all_functions {
        if !assigned_levels.contains_key(func) {
            unassigned.push(func.clone());
        }
    }
    
    if !unassigned.is_empty() {
        println!("⚠️  Found {} functions in cycles - assigning to top level", unassigned.len());
        levels.push(unassigned);
    }
    
    // Generate the corrected lattice
    generate_fixed_lattice(&levels)?;
    
    Ok(())
}

fn generate_fixed_lattice(levels: &[Vec<String>]) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = fs::File::create("rustc_fixed_lattice.txt")?;
    
    writeln!(output, "🔬 FIXED RUSTC DEPENDENCY LATTICE")?;
    writeln!(output, "=================================")?;
    writeln!(output, "Total Levels: {}", levels.len())?;
    writeln!(output, "Structure: Leaves (Level 0) → Main (Level {})", levels.len() - 1)?;
    writeln!(output)?;
    
    for (level_idx, level_functions) in levels.iter().enumerate() {
        if level_functions.is_empty() { continue; }
        
        let level_type = if level_idx == 0 {
            "🌱 LEAF LEVEL (No Dependencies)"
        } else if level_idx == levels.len() - 1 {
            "🚀 MAIN LEVEL (Entry Point)"
        } else {
            "📊 INTERMEDIATE LEVEL"
        };
        
        writeln!(output, "LEVEL {} - {} ({} functions)", level_idx, level_type, level_functions.len())?;
        writeln!(output, "├─ Dependency Depth: {}", level_idx)?;
        writeln!(output, "└─ Functions:")?;
        
        for (idx, func) in level_functions.iter().enumerate() {
            let prefix = if idx == level_functions.len() - 1 { "   └─" } else { "   ├─" };
            writeln!(output, "{} {}", prefix, func)?;
        }
        writeln!(output)?;
    }
    
    println!("✅ Generated rustc_fixed_lattice.txt");
    println!("🔬 Fixed lattice has {} levels", levels.len());
    
    // Show top level (should contain rustc::main::main)
    if let Some(top_level) = levels.last() {
        println!("🚀 Top level (main): {} functions", top_level.len());
        for func in top_level {
            println!("   - {}", func);
            if func.contains("rustc") && func.contains("main") {
                println!("     ⭐ FOUND MAIN ENTRY POINT!");
            }
        }
    }
    
    // Show bottom level (leaves)
    if let Some(bottom_level) = levels.first() {
        println!("🌱 Bottom level (leaves): {} functions", bottom_level.len());
        for func in bottom_level.iter().take(5) {
            println!("   - {}", func);
        }
        if bottom_level.len() > 5 {
            println!("   ... and {} more", bottom_level.len() - 5);
        }
    }
    
    Ok(())
}
