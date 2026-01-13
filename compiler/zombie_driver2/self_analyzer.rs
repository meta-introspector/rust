use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SemanticSignature {
    name: String,
    demangled_name: String,
    size: u64,
    instruction_hash: u64,
    complexity_score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BinarySemanticProfile {
    binary_name: String,
    signatures: Vec<SemanticSignature>,
    total_functions: usize,
    total_instructions: usize,
    total_nodes: usize,
    type_complexity: f64,
}

#[derive(Debug)]
struct SimilarityScore {
    binary_name: String,
    shared_functions: usize,
    shared_instruction_hashes: usize,
    complexity_similarity: f64,
    total_similarity: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 SELF-ANALYSIS: Finding Most Similar Code");
    println!("==========================================");
    
    let target_binary = "semantic_signature_generator";
    let signature_dir = "semantic_signatures";
    
    // Load our own signature
    let target_path = format!("{}/{}.semantic.json", signature_dir, target_binary);
    let target_content = fs::read_to_string(&target_path)?;
    let target_profile: BinarySemanticProfile = serde_json::from_str(&target_content)?;
    
    println!("🎯 Analyzing: {} ({} functions, {} instructions)", 
             target_profile.binary_name, 
             target_profile.total_functions,
             target_profile.total_instructions);
    
    // Create lookup maps for target
    let target_functions: HashMap<String, &SemanticSignature> = target_profile.signatures.iter()
        .map(|s| (s.demangled_name.clone(), s))
        .collect();
    
    let target_hashes: HashMap<u64, &SemanticSignature> = target_profile.signatures.iter()
        .filter(|s| s.instruction_hash != 0)
        .map(|s| (s.instruction_hash, s))
        .collect();
    
    let mut similarities = Vec::new();
    
    // Compare against all other binaries
    let entries = fs::read_dir(signature_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let filename = path.file_stem().unwrap().to_string_lossy();
            if filename == target_binary {
                continue; // Skip self
            }
            
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(profile) = serde_json::from_str::<BinarySemanticProfile>(&content) {
                    let similarity = calculate_similarity(&target_profile, &target_functions, &target_hashes, &profile);
                    similarities.push(similarity);
                }
            }
        }
    }
    
    // Sort by total similarity
    similarities.sort_by(|a, b| b.total_similarity.partial_cmp(&a.total_similarity).unwrap());
    
    println!("\n🏆 TOP 10 MOST SIMILAR BINARIES:");
    println!("=================================");
    
    for (i, sim) in similarities.iter().take(10).enumerate() {
        println!("{}. {} (similarity: {:.1}%)", 
                 i + 1, sim.binary_name, sim.total_similarity * 100.0);
        println!("   Shared functions: {} | Shared instruction blocks: {} | Complexity similarity: {:.2}",
                 sim.shared_functions, sim.shared_instruction_hashes, sim.complexity_similarity);
        println!();
    }
    
    // Analyze the most similar
    if let Some(most_similar) = similarities.first() {
        println!("🔬 DETAILED ANALYSIS OF MOST SIMILAR: {}", most_similar.binary_name);
        println!("====================================================");
        
        // Load the most similar binary for detailed comparison
        let similar_path = format!("{}/{}.semantic.json", signature_dir, most_similar.binary_name);
        let similar_content = fs::read_to_string(&similar_path)?;
        let similar_profile: BinarySemanticProfile = serde_json::from_str(&similar_content)?;
        
        // Find shared functions
        let mut shared_funcs = Vec::new();
        for sig in &similar_profile.signatures {
            if target_functions.contains_key(&sig.demangled_name) {
                shared_funcs.push(&sig.demangled_name);
            }
        }
        
        println!("Shared function examples:");
        for func in shared_funcs.iter().take(5) {
            println!("  - {}", func);
        }
        if shared_funcs.len() > 5 {
            println!("  ... and {} more", shared_funcs.len() - 5);
        }
        
        // Architecture comparison
        println!("\nArchitecture Comparison:");
        println!("  Target functions: {} | Similar functions: {}", 
                 target_profile.total_functions, similar_profile.total_functions);
        println!("  Target instructions: {} | Similar instructions: {}", 
                 target_profile.total_instructions, similar_profile.total_instructions);
        println!("  Target complexity: {:.2} | Similar complexity: {:.2}", 
                 target_profile.type_complexity, similar_profile.type_complexity);
    }
    
    // Find least similar for contrast
    if let Some(least_similar) = similarities.last() {
        println!("\n🔄 LEAST SIMILAR FOR CONTRAST: {}", least_similar.binary_name);
        println!("Similarity: {:.1}% | Shared functions: {} | Shared blocks: {}", 
                 least_similar.total_similarity * 100.0,
                 least_similar.shared_functions,
                 least_similar.shared_instruction_hashes);
    }
    
    // Save detailed report
    let report = serde_json::json!({
        "target_binary": target_binary,
        "target_stats": {
            "functions": target_profile.total_functions,
            "instructions": target_profile.total_instructions,
            "complexity": target_profile.type_complexity
        },
        "similarities": similarities.iter().take(20).map(|s| serde_json::json!({
            "binary": s.binary_name,
            "similarity_score": s.total_similarity,
            "shared_functions": s.shared_functions,
            "shared_instruction_blocks": s.shared_instruction_hashes,
            "complexity_similarity": s.complexity_similarity
        })).collect::<Vec<_>>()
    });
    
    fs::write("self_analysis_report.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Detailed self-analysis report saved to: self_analysis_report.json");
    
    Ok(())
}

fn calculate_similarity(
    target: &BinarySemanticProfile,
    target_functions: &HashMap<String, &SemanticSignature>,
    target_hashes: &HashMap<u64, &SemanticSignature>,
    other: &BinarySemanticProfile
) -> SimilarityScore {
    let mut shared_functions = 0;
    let mut shared_instruction_hashes = 0;
    
    // Count shared functions
    for sig in &other.signatures {
        if target_functions.contains_key(&sig.demangled_name) {
            shared_functions += 1;
        }
        if sig.instruction_hash != 0 && target_hashes.contains_key(&sig.instruction_hash) {
            shared_instruction_hashes += 1;
        }
    }
    
    // Calculate similarity metrics
    let function_similarity = shared_functions as f64 / target.total_functions.max(other.total_functions) as f64;
    let instruction_similarity = shared_instruction_hashes as f64 / target_hashes.len().max(1) as f64;
    let complexity_diff = (target.type_complexity - other.type_complexity).abs();
    let complexity_similarity = 1.0 - (complexity_diff / target.type_complexity.max(other.type_complexity).max(1.0));
    
    // Weighted total similarity
    let total_similarity = (function_similarity * 0.4) + (instruction_similarity * 0.4) + (complexity_similarity * 0.2);
    
    SimilarityScore {
        binary_name: other.binary_name.clone(),
        shared_functions,
        shared_instruction_hashes,
        complexity_similarity,
        total_similarity,
    }
}
