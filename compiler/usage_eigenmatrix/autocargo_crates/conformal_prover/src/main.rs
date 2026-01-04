use crate::conformal_proof::ConformalProver;
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};

mod conformal_proof;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    println!("🏹 CONFORMAL FIELD THEORY PROVER");
    println!("📊 Proving: Simple Models ≅ Complex Models (Arrow Preservation)");
    println!("🔄 Phase Transition: Old Rust → New Rust with Conformal Invariance");
    println!();
    
    let mut prover = ConformalProver::new();
    
    println!("Commands:");
    println!("  extract - Extract local ASTs from our repo");
    println!("  map - Map to rustc ASTs");  
    println!("  emoji - Generate emoji mappings");
    println!("  prove - Prove arrow preservation (MAIN PROOF)");
    println!("  status - Show proof status");
    println!("  auto - Run complete proof automatically");

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        if let Ok(Some(line)) = stdin.next_line().await {
            match handle_proof_command(&mut prover, &line).await {
                Ok(_) => {}
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }
}

async fn handle_proof_command(prover: &mut ConformalProver, command: &str) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    
    match parts.get(0) {
        Some(&"extract") => {
            prover.extract_local_asts().await?;
        }
        
        Some(&"map") => {
            prover.map_to_rustc_asts().await?;
        }
        
        Some(&"emoji") => {
            prover.generate_emoji_mappings().await?;
        }
        
        Some(&"prove") => {
            println!("🏹 STARTING CONFORMAL FIELD THEORY PROOF");
            println!("📊 Hypothesis: Our simple models preserve all arrows to complex rustc");
            println!();
            
            let proven = prover.prove_arrow_preservation().await?;
            
            if proven {
                println!();
                println!("🎉 PROOF COMPLETE: CONFORMAL FIELD THEORY VERIFIED");
                println!("✅ Simple ≅ Complex: Arrow preservation confirmed");
                println!("🔄 Phase transition preserves all fundamental relationships");
                println!("📊 No arrows bent - conformal mapping successful");
            } else {
                println!();
                println!("❌ PROOF FAILED: Conformal mapping broken");
                println!("🔧 Some arrows were bent during transformation");
            }
        }
        
        Some(&"status") => {
            println!("📊 CONFORMAL PROOF STATUS:");
            println!("  Local ASTs: {}", prover.proof.local_asts.len());
            println!("  Rustc ASTs: {}", prover.proof.rust_asts.len());
            println!("  Emoji mappings: {}", prover.proof.emoji_mappings.len());
            println!("  8D coordinates: {}", prover.proof.space_8d.len());
            println!("  Simple arrows: {}", prover.proof.arrow_preservation.simple_arrows.len());
            println!("  Complex arrows: {}", prover.proof.arrow_preservation.complex_arrows.len());
            println!("  Arrow mappings: {}", prover.proof.arrow_preservation.arrow_mappings.len());
            println!("  Conformal invariants: {}", prover.proof.arrow_preservation.conformal_invariants.len());
            
            if !prover.proof.arrow_preservation.conformal_invariants.is_empty() {
                let preserved = prover.proof.arrow_preservation.conformal_invariants
                    .iter()
                    .filter(|inv| inv.preserved)
                    .count();
                let total = prover.proof.arrow_preservation.conformal_invariants.len();
                println!("  Preservation ratio: {}/{} ({:.1}%)", 
                         preserved, total, (preserved as f64 / total as f64) * 100.0);
            }
        }
        
        Some(&"auto") => {
            println!("🚀 RUNNING COMPLETE CONFORMAL PROOF");
            println!();
            
            println!("Step 1: Extracting local ASTs...");
            prover.extract_local_asts().await?;
            
            println!("Step 2: Mapping to rustc ASTs...");
            prover.map_to_rustc_asts().await?;
            
            println!("Step 3: Generating emoji mappings...");
            prover.generate_emoji_mappings().await?;
            
            println!("Step 4: Proving arrow preservation...");
            let proven = prover.prove_arrow_preservation().await?;
            
            println!();
            if proven {
                println!("🎉 COMPLETE PROOF SUCCESSFUL");
                println!("✅ Conformal Field Theory: Simple ≅ Complex");
                println!("🔄 Phase transition verified with arrow preservation");
                
                // Show some example mappings
                println!();
                println!("📊 Example Conformal Mappings:");
                for (i, (simple, complex)) in prover.proof.arrow_preservation.arrow_mappings.iter().take(5).enumerate() {
                    let emoji = prover.proof.emoji_mappings.get(simple).unwrap_or(&"❓".to_string());
                    let coords = prover.proof.space_8d.get(simple).unwrap_or(&[0.0; 8]);
                    println!("  {}: {} {} → {} (8D: [{:.2}, {:.2}, {:.2}, ...])", 
                             i+1, simple, emoji, complex, coords[0], coords[1], coords[2]);
                }
            } else {
                println!("❌ PROOF FAILED - Conformal mapping incomplete");
            }
        }
        
        _ => {
            println!("Unknown command: {}", command);
        }
    }
    
    Ok(())
}
