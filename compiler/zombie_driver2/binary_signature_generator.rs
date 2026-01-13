use std::collections::HashMap;
use std::fs;
use std::path::Path;
use goblin::elf::Elf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FunctionSignature {
    name: String,
    size: u64,
    address: u64,
    hash: u64,
    instruction_pattern: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BinarySignature {
    binary_path: String,
    binary_name: String,
    total_functions: usize,
    signatures: Vec<FunctionSignature>,
    created_at: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("🔍 BINARY SIGNATURE GENERATOR");
        println!("Usage: {} <binary_path> [output_dir]", args[0]);
        println!("       {} --compare <sig1.json> <sig2.json>", args[0]);
        println!("       {} --batch <directory>", args[0]);
        return Ok(());
    }

    let output_dir = args.get(3).map(|s| s.as_str()).unwrap_or("./signatures");
    fs::create_dir_all(output_dir)?;

    match args[1].as_str() {
        "--compare" => {
            if args.len() < 4 {
                eprintln!("Need two signature files to compare");
                return Ok(());
            }
            compare_signatures(&args[2], &args[3])?;
        }
        "--batch" => {
            if args.len() < 3 {
                eprintln!("Need directory path for batch processing");
                return Ok(());
            }
            batch_process(&args[2], output_dir)?;
        }
        _ => {
            let binary_path = &args[1];
            generate_signature(binary_path, output_dir)?;
        }
    }

    Ok(())
}

fn generate_signature(binary_path: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Generating signature for: {}", binary_path);
    
    let binary_data = fs::read(binary_path)?;
    let elf = Elf::parse(&binary_data)?;
    
    let binary_name = Path::new(binary_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    let mut signatures = Vec::new();
    
    for sym in &elf.syms {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 {
                let demangled = rustc_demangle::demangle(name).to_string();
                
                // Extract instruction pattern (first 32 bytes as hex)
                let instruction_pattern = if let Some(section) = find_text_section(&elf) {
                    let offset = sym.st_value.saturating_sub(section.sh_addr);
                    if offset < section.sh_size && offset < binary_data.len() as u64 {
                        let start = (section.sh_offset + offset) as usize;
                        let end = std::cmp::min(start + 32, binary_data.len());
                        if start < binary_data.len() {
                            format!("{:02x}", &binary_data[start..end].iter().fold(0u8, |acc, &x| acc.wrapping_add(x)))
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                
                // Create a simple hash from name + size + pattern
                let hash = calculate_function_hash(&demangled, sym.st_size, &instruction_pattern);
                
                signatures.push(FunctionSignature {
                    name: demangled,
                    size: sym.st_size,
                    address: sym.st_value,
                    hash,
                    instruction_pattern,
                });
            }
        }
    }
    
    let signature = BinarySignature {
        binary_path: binary_path.to_string(),
        binary_name: binary_name.clone(),
        total_functions: signatures.len(),
        signatures,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    let output_path = format!("{}/{}.sig.json", output_dir, binary_name);
    let json = serde_json::to_string_pretty(&signature)?;
    fs::write(&output_path, json)?;
    
    println!("✅ Signature saved: {} ({} functions)", output_path, signature.total_functions);
    
    Ok(())
}

fn find_text_section(elf: &Elf) -> Option<&goblin::elf::SectionHeader> {
    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            if name == ".text" {
                return Some(section);
            }
        }
    }
    None
}

fn calculate_function_hash(name: &str, size: u64, pattern: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    size.hash(&mut hasher);
    pattern.hash(&mut hasher);
    hasher.finish()
}

fn compare_signatures(sig1_path: &str, sig2_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Comparing signatures: {} vs {}", sig1_path, sig2_path);
    
    let sig1_data = fs::read_to_string(sig1_path)?;
    let sig2_data = fs::read_to_string(sig2_path)?;
    
    let sig1: BinarySignature = serde_json::from_str(&sig1_data)?;
    let sig2: BinarySignature = serde_json::from_str(&sig2_data)?;
    
    // Create hash maps for quick lookup
    let sig1_map: HashMap<u64, &FunctionSignature> = sig1.signatures.iter()
        .map(|s| (s.hash, s))
        .collect();
    
    let sig2_map: HashMap<u64, &FunctionSignature> = sig2.signatures.iter()
        .map(|s| (s.hash, s))
        .collect();
    
    let mut duplicates = Vec::new();
    let mut unique_to_sig1 = Vec::new();
    let mut unique_to_sig2 = Vec::new();
    
    // Find duplicates and unique functions
    for (hash, func1) in &sig1_map {
        if let Some(func2) = sig2_map.get(hash) {
            duplicates.push((func1, func2));
        } else {
            unique_to_sig1.push(func1);
        }
    }
    
    for (hash, func2) in &sig2_map {
        if !sig1_map.contains_key(hash) {
            unique_to_sig2.push(func2);
        }
    }
    
    println!("\n📊 COMPARISON RESULTS");
    println!("====================");
    println!("Binary 1: {} ({} functions)", sig1.binary_name, sig1.total_functions);
    println!("Binary 2: {} ({} functions)", sig2.binary_name, sig2.total_functions);
    println!("Duplicates: {}", duplicates.len());
    println!("Unique to {}: {}", sig1.binary_name, unique_to_sig1.len());
    println!("Unique to {}: {}", sig2.binary_name, unique_to_sig2.len());
    
    if !duplicates.is_empty() {
        println!("\n🔄 DUPLICATE FUNCTIONS:");
        for (func1, func2) in duplicates.iter().take(10) {
            println!("  {} (size: {})", func1.name, func1.size);
        }
        if duplicates.len() > 10 {
            println!("  ... and {} more", duplicates.len() - 10);
        }
    }
    
    // Calculate novelty score
    let total_functions = sig2.total_functions;
    let novel_functions = unique_to_sig2.len();
    let novelty_score = if total_functions > 0 {
        (novel_functions as f64 / total_functions as f64) * 100.0
    } else {
        0.0
    };
    
    println!("\n🎯 NOVELTY ANALYSIS");
    println!("Novelty Score: {:.2}% ({}/{} functions are unique)", 
             novelty_score, novel_functions, total_functions);
    
    Ok(())
}

fn batch_process(directory: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Batch processing directory: {}", directory);
    
    let entries = fs::read_dir(directory)?;
    let mut processed = 0;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let path_str = path.to_string_lossy();
            
            // Skip non-executable files and our own signatures
            if path_str.ends_with(".sig.json") || path_str.ends_with(".d") {
                continue;
            }
            
            // Try to process as binary
            if let Err(e) = generate_signature(&path_str, output_dir) {
                println!("⚠️  Skipped {}: {}", path_str, e);
            } else {
                processed += 1;
            }
        }
    }
    
    println!("✅ Processed {} binaries", processed);
    Ok(())
}
