use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌀 FIBONACCI SPIRAL RUSTC DECOMPOSITION");
    
    let test_crates = vec![
        ".",
        "../usage_eigenmatrix", 
        "../../../syn",
        "../../../burn",
        "../../../cubecl", 
        "../../../measureme",
        "../../../rust-build",
        "../../../rust",
    ];
    
    fs::create_dir_all("fibonacci_traces")?;
    
    for fib_level in 0..=10 {
        let complexity = fibonacci(fib_level);
        let crate_count = fibonacci(fib_level + 1).min(test_crates.len());
        
        println!("\n🌀 FIBONACCI LEVEL {} - Complexity: {}, Crates: {}", 
            fib_level, complexity, crate_count);
        
        env::set_var("MAX_COMPLEXITY", complexity.to_string());
        env::set_var("FIBONACCI_LEVEL", fib_level.to_string());
        
        for i in 0..crate_count {
            let crate_path = test_crates[i];
            if !Path::new(crate_path).exists() { continue; }
            
            println!("  🔄 Building crate {} with complexity {}", crate_path, complexity);
            
            let output = Command::new("cargo")
                .args(&["build", "--release"])
                .current_dir(crate_path)
                .env("MAX_COMPLEXITY", complexity.to_string())
                .env("FIBONACCI_LEVEL", fib_level.to_string())
                .output()?;
                
            if output.status.success() {
                let crate_name = Path::new(crate_path).file_name()
                    .unwrap_or_default().to_string_lossy();
                    
                // Collect fibonacci-spiral profiles
                if let Ok(entries) = fs::read_dir(crate_path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let name = entry.file_name();
                            if let Some(name_str) = name.to_str() {
                                if name_str.starts_with("rustc_") && name_str.ends_with(".mm_profdata") {
                                    let new_name = format!("fibonacci_traces/fib_{}_{}_c{}_profile.mm_profdata", 
                                        fib_level, crate_name, complexity);
                                    let _ = fs::rename(entry.path(), &new_name);
                                    println!("    🌀 Saved: {}", new_name);
                                }
                            }
                        }
                    }
                }
                
                println!("    ✅ Fibonacci spiral level {} complete", fib_level);
            } else {
                println!("    ❌ Failed at fibonacci level {}", fib_level);
            }
        }
        
        println!("  🎯 Fibonacci {} → Complexity: {}, Built: {} crates", 
            fib_level, complexity, crate_count);
    }
    
    println!("\n🌀 FIBONACCI RUSTC DECOMPOSITION COMPLETE");
    println!("📊 Fibonacci sequence: 0,1,1,2,3,5,8,13,21,34,55...");
    println!("🚀 Each level builds the next with exponentially growing complexity!");
    Ok(())
}
