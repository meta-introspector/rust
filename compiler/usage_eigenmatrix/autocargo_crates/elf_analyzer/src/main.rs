use std::fs;
use std::process::Command;

fn parse_elf_headers(rlib_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Analyzing ELF headers in: {}", rlib_path);
    
    // Extract .rlib (it's an archive)
    let output = Command::new("ar")
        .args(&["t", rlib_path])
        .output()?;
    
    println!("📦 Archive contents:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    // Extract all files from archive
    let extract_output = Command::new("ar")
        .args(&["x", rlib_path])
        .output()?;
    
    if !extract_output.status.success() {
        println!("❌ Failed to extract archive");
        return Ok(());
    }
    
    // Find .o files and analyze them
    let ls_output = Command::new("ls")
        .args(&["-la", "*.o"])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    println!("📄 Object files:");
    println!("{}", String::from_utf8_lossy(&ls_output.stdout));
    
    // Use readelf to parse ELF headers
    let readelf_output = Command::new("find")
        .args(&[".", "-name", "*.o", "-exec", "readelf", "-h", "{}", ";"])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    println!("🔧 ELF Headers:");
    println!("{}", String::from_utf8_lossy(&readelf_output.stdout));
    
    // Get symbol tables
    let symbols_output = Command::new("find")
        .args(&[".", "-name", "*.o", "-exec", "readelf", "-s", "{}", ";"])
        .output()
        .unwrap_or_else(|_| std::process::Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    
    println!("📊 Symbol Tables:");
    println!("{}", String::from_utf8_lossy(&symbols_output.stdout));
    
    // Clean up extracted files
    let _ = Command::new("rm").args(&["-f", "*.o"]).output();
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <path_to_rlib>", args[0]);
        println!("Example: {} target/debug/libsplit_decls_genesis.rlib", args[0]);
        return Ok(());
    }
    
    let rlib_path = &args[1];
    
    if !std::path::Path::new(rlib_path).exists() {
        println!("❌ File not found: {}", rlib_path);
        return Ok(());
    }
    
    parse_elf_headers(rlib_path)?;
    
    Ok(())
}
