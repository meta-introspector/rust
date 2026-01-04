use std::process::Command;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Quick Error Pattern Extractor");
    
    // Run cargo build and capture errors
    let output = Command::new("cargo")
        .args(&["build"])
        .output()?;
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Extract unique error patterns
    let mut error_patterns = Vec::new();
    
    for line in stderr.lines() {
        if line.contains("error:") {
            error_patterns.push(line.to_string());
        }
    }
    
    // Save to file
    fs::write("current_errors.txt", error_patterns.join("\n"))?;
    
    println!("📊 Found {} unique error patterns", error_patterns.len());
    println!("💾 Saved to current_errors.txt");
    
    // Show first few errors
    for (i, error) in error_patterns.iter().take(5).enumerate() {
        println!("{}. {}", i+1, error);
    }
    
    Ok(())
}
