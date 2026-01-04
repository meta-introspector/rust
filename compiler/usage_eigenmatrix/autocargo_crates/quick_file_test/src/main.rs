use std::process::Command;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Quick File Compiler - Testing individual processed files");
    
    let src_dir = "src";
    let mut total_files = 0;
    let mut compiled_files = 0;
    let mut error_files = 0;
    
    // Get all processed files
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "rs") 
            && path.file_name().unwrap().to_str().unwrap().starts_with("processed_") {
            
            total_files += 1;
            let file_name = path.file_name().unwrap().to_str().unwrap();
            
            print!("Testing {:<60} ... ", file_name);
            
            // Try to compile just this file
            let output = Command::new("rustc")
                .args(&[
                    "--crate-type", "lib",
                    "--allow", "warnings",
                    "--cfg", "feature=\"no_core\"",
                    path.to_str().unwrap()
                ])
                .output()?;
            
            if output.status.success() {
                println!("✅ OK");
                compiled_files += 1;
            } else {
                println!("❌ FAIL");
                error_files += 1;
                
                // Show first error
                let stderr = String::from_utf8_lossy(&output.stderr);
                if let Some(first_error) = stderr.lines().find(|line| line.contains("error:")) {
                    println!("    {}", first_error);
                }
            }
        }
    }
    
    println!("\n📊 Summary:");
    println!("   Total files: {}", total_files);
    println!("   Compiled:    {} ({}%)", compiled_files, (compiled_files * 100) / total_files.max(1));
    println!("   Errors:      {} ({}%)", error_files, (error_files * 100) / total_files.max(1));
    
    Ok(())
}
