use std::process::Command;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        return;
    }
    
    let source = &args[1];
    let mut iteration = 0;
    let max_iterations = 5;
    
    println!("🔄 Self-Improving Compiler Driver");
    println!("==================================");
    
    loop {
        iteration += 1;
        println!("Iteration {}: Analyzing {}", iteration, source);
        
        // Run embedded advisor on source
        let output = Command::new("cargo")
            .args(&["run", "--bin", "embedded_graph_advisor", source])
            .output()
            .expect("Failed to run advisor");
        
        let suggestions = String::from_utf8_lossy(&output.stdout);
        let suggestion_count = suggestions.matches("🎯 Line").count();
        
        if suggestion_count == 0 {
            println!("✅ No more suggestions - converged!");
            break;
        }
        
        println!("  Found {} suggestions, applying fixes...", suggestion_count);
        
        // Apply simple fixes
        if let Ok(content) = fs::read_to_string(source) {
            let mut fixed = content
                .replace(".clone()", "")  // Remove unnecessary clones
                .replace(".unwrap()", "?"); // Replace unwrap with ?
            
            // Backup and write
            let backup = format!("{}.iter{}", source, iteration);
            fs::write(&backup, &content).ok();
            fs::write(source, &fixed).ok();
            
            println!("  ✅ Applied fixes, backup saved to {}", backup);
        }
        
        if iteration >= max_iterations {
            println!("⚠️  Max iterations reached");
            break;
        }
    }
    
    // Final compilation test
    println!("\n🧪 Testing final compilation...");
    let result = Command::new("rustc")
        .args(&["--check", source])
        .status()
        .expect("Failed to test compilation");
    
    if result.success() {
        println!("🎉 Self-improvement complete - code compiles successfully!");
    } else {
        println!("❌ Compilation issues remain");
    }
}
