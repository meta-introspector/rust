use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analysis = fs::read_to_string("rustc_complete_analysis.txt")?;
    
    // Parse dependency graph from indentation
    let mut dependencies = HashMap::new();
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
                    
                    // Add dependencies (functions this one calls)
                    let deps = dependencies.entry(func_name.clone()).or_insert_with(Vec::new);
                    
                    // Add current stack as dependencies (functions this calls)
                    for dep in &current_stack {
                        if !deps.contains(dep) {
                            deps.push(dep.clone());
                        }
                    }
                    
                    // Push current function to stack
                    current_stack.push(func_name);
                }
            }
        }
    }
    
    // Calculate dependency levels using topological sort
    let levels = calculate_dependency_levels(&dependencies, &all_functions);
    
    // Generate corrected lattice
    generate_corrected_lattice(&levels)?;
    
    Ok(())
}

fn calculate_dependency_levels(
    dependencies: &HashMap<String, Vec<String>>, 
    all_functions: &HashSet<String>
) -> Vec<Vec<String>> {
    let mut in_degree = HashMap::new();
    let mut levels = Vec::new();
    
    // Calculate in-degrees (number of dependencies)
    for func in all_functions {
        let degree = dependencies.get(func).map_or(0, |deps| deps.len());
        in_degree.insert(func.clone(), degree);
    }
    
    // Start with functions that have no dependencies (leaves)
    let mut queue = VecDeque::new();
    for (func, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(func.clone());
        }
    }
    
    let mut current_level = 0;
    
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut current_level_functions = Vec::new();
        
        // Process all functions at current level
        for _ in 0..level_size {
            if let Some(func) = queue.pop_front() {
                current_level_functions.push(func.clone());
                
                // Find functions that depend on this one
                for (other_func, deps) in dependencies {
                    if deps.contains(&func) {
                        if let Some(degree) = in_degree.get_mut(other_func) {
                            *degree -= 1;
                            if *degree == 0 {
                                queue.push_back(other_func.clone());
                            }
                        }
                    }
                }
            }
        }
        
        if !current_level_functions.is_empty() {
            levels.push(current_level_functions);
            current_level += 1;
        }
    }
    
    levels
}

fn generate_corrected_lattice(levels: &[Vec<String>]) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = fs::File::create("rustc_corrected_lattice.txt")?;
    
    writeln!(output, "🔬 CORRECTED RUSTC DEPENDENCY LATTICE")?;
    writeln!(output, "====================================")?;
    writeln!(output, "Total Levels: {}", levels.len())?;
    writeln!(output, "Structure: Leaves → Dependencies → Main")?;
    writeln!(output)?;
    
    for (level_idx, level_functions) in levels.iter().enumerate() {
        let level_type = if level_idx == 0 {
            "🌱 LEAF LEVEL (No Dependencies)"
        } else if level_idx == levels.len() - 1 {
            "🚀 MAIN LEVEL (rustc::main::main)"
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
    
    // Generate corrected macro composition
    writeln!(output, "🏗️ CORRECTED MACRO COMPOSITION (Leaves → Main)")?;
    writeln!(output, "==============================================")?;
    writeln!(output, "macro_rules! build_rustc_correct {{")?;
    writeln!(output, "    () => {{")?;
    
    for (level_idx, level_functions) in levels.iter().enumerate() {
        writeln!(output, "        // Level {}: {} functions", level_idx, level_functions.len())?;
        for func in level_functions {
            let macro_name = func.replace("::", "_").replace(".", "_").to_lowercase();
            writeln!(output, "        {}!();", macro_name)?;
        }
        writeln!(output)?;
    }
    
    writeln!(output, "    }};")?;
    writeln!(output, "}}")?;
    
    println!("✅ Generated rustc_corrected_lattice.txt");
    println!("🔬 Corrected lattice has {} levels", levels.len());
    
    if let Some(top_level) = levels.last() {
        println!("🚀 Top level (main): {} functions", top_level.len());
        for func in top_level {
            println!("   - {}", func);
        }
    }
    
    Ok(())
}
