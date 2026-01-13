use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SemanticSignature {
    name: String,
    demangled_name: String,
    size: u64,
    address: u64,
    instruction_hash: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BinarySemanticProfile {
    binary_name: String,
    signatures: Vec<SemanticSignature>,
    total_functions: usize,
}

#[derive(Debug)]
struct DuplicateBlock {
    instruction_hash: u64,
    size: u64,
    binaries: Vec<String>,
    functions: Vec<String>,
    count: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 DUPLICATE INSTRUCTION BLOCK DETECTOR");
    println!("========================================");
    
    let signature_dir = "semantic_signatures";
    let entries = fs::read_dir(signature_dir)?;
    
    let mut all_blocks: HashMap<u64, DuplicateBlock> = HashMap::new();
    let mut binary_count = 0;
    
    // Process all semantic signature files
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(profile) = serde_json::from_str::<BinarySemanticProfile>(&content) {
                    binary_count += 1;
                    
                    for sig in &profile.signatures {
                        if sig.instruction_hash != 0 && sig.size > 0 {
                            let block = all_blocks.entry(sig.instruction_hash).or_insert(DuplicateBlock {
                                instruction_hash: sig.instruction_hash,
                                size: sig.size,
                                binaries: Vec::new(),
                                functions: Vec::new(),
                                count: 0,
                            });
                            
                            if !block.binaries.contains(&profile.binary_name) {
                                block.binaries.push(profile.binary_name.clone());
                            }
                            block.functions.push(sig.demangled_name.clone());
                            block.count += 1;
                        }
                    }
                }
            }
        }
    }
    
    // Find duplicates (appearing in multiple binaries)
    let mut duplicates: Vec<_> = all_blocks.values()
        .filter(|block| block.binaries.len() > 1)
        .collect();
    
    duplicates.sort_by(|a, b| b.binaries.len().cmp(&a.binaries.len()));
    
    println!("\n📊 DUPLICATE ANALYSIS RESULTS");
    println!("==============================");
    println!("Analyzed {} binaries", binary_count);
    println!("Found {} unique instruction blocks", all_blocks.len());
    println!("Found {} duplicate blocks", duplicates.len());
    
    println!("\n🔄 TOP 10 MOST DUPLICATED BLOCKS:");
    for (i, dup) in duplicates.iter().take(10).enumerate() {
        println!("{}. Hash: {:016x} | Size: {} bytes | In {} binaries | {} functions", 
                 i + 1, dup.instruction_hash, dup.size, dup.binaries.len(), dup.count);
        println!("   Binaries: {}", dup.binaries.join(", "));
        if dup.functions.len() <= 3 {
            println!("   Functions: {}", dup.functions.join(", "));
        } else {
            println!("   Functions: {} (showing first 3: {})", 
                     dup.functions.len(), 
                     dup.functions.iter().take(3).cloned().collect::<Vec<_>>().join(", "));
        }
        println!();
    }
    
    // Calculate duplication statistics
    let total_blocks = all_blocks.len();
    let duplicate_blocks = duplicates.len();
    let duplication_rate = (duplicate_blocks as f64 / total_blocks as f64) * 100.0;
    
    println!("📈 DUPLICATION STATISTICS:");
    println!("Duplication Rate: {:.1}% ({}/{} blocks)", duplication_rate, duplicate_blocks, total_blocks);
    
    // Find most duplicated binary
    let mut binary_dup_count: HashMap<String, usize> = HashMap::new();
    for dup in &duplicates {
        for binary in &dup.binaries {
            *binary_dup_count.entry(binary.clone()).or_insert(0) += 1;
        }
    }
    
    let mut sorted_binaries: Vec<_> = binary_dup_count.iter().collect();
    sorted_binaries.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎯 MOST DUPLICATED BINARIES:");
    for (binary, count) in sorted_binaries.iter().take(5) {
        println!("{}: {} duplicate blocks", binary, count);
    }
    
    // Save detailed report
    let report = serde_json::json!({
        "total_binaries": binary_count,
        "total_blocks": total_blocks,
        "duplicate_blocks": duplicate_blocks,
        "duplication_rate": duplication_rate,
        "top_duplicates": duplicates.iter().take(20).map(|d| serde_json::json!({
            "hash": format!("{:016x}", d.instruction_hash),
            "size": d.size,
            "binary_count": d.binaries.len(),
            "function_count": d.count,
            "binaries": d.binaries,
            "sample_functions": d.functions.iter().take(5).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    });
    
    fs::write("duplicate_analysis_report.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Detailed report saved to: duplicate_analysis_report.json");
    
    Ok(())
}
