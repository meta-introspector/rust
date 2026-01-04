use crate::rust_lattice::{RustLattice, LatticeCoordinate};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};

mod rust_lattice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    println!("🔮 RUST LATTICE CONSTRUCTOR");
    println!("📊 N models × M sizes × 2^n encodings with Reed-Solomon error correction");
    println!("Commands:");
    println!("  generate <N> <M> <n> - Generate N×M×2^n lattice");
    println!("  corrupt <model> <size> <encoding> - Simulate corruption");
    println!("  recover - Recover from corruption using error correction");
    println!("  show <size> - Show all models of specific size");

    let mut stdin = io::BufReader::new(io::stdin()).lines();
    let mut lattice: Option<RustLattice> = None;
    let mut corrupted_coords = Vec::new();

    loop {
        if let Ok(Some(line)) = stdin.next_line().await {
            match handle_lattice_command(&mut lattice, &mut corrupted_coords, &line).await {
                Ok(_) => {}
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }
}

async fn handle_lattice_command(
    lattice: &mut Option<RustLattice>,
    corrupted_coords: &mut Vec<LatticeCoordinate>,
    command: &str
) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    
    match parts.get(0) {
        Some(&"generate") => {
            let n = parts.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(3);
            let m = parts.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(4);
            let encoding_bits = parts.get(3).and_then(|s| s.parse::<usize>().ok()).unwrap_or(3);
            
            println!("🔧 Generating {}×{}×{} Rust Lattice...", n, m, 1 << encoding_bits);
            
            let mut new_lattice = RustLattice::new(n, m, encoding_bits);
            
            // Base rustc data (sample)
            let base_rustc = vec![0x90, 0x48, 0x89, 0xC3]; // fn main() { return; }
            
            new_lattice.generate_lattice(base_rustc).await?;
            
            println!("✅ Lattice generated with {} total models", new_lattice.lattice_points.len());
            println!("📊 Dimensions: {} models × {} sizes × {} encodings", n, m, 1 << encoding_bits);
            
            *lattice = Some(new_lattice);
        }
        
        Some(&"corrupt") => {
            if let Some(ref l) = lattice {
                let model = parts.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                let size = parts.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                let encoding = parts.get(3).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                
                let coord = LatticeCoordinate { model, size, encoding };
                
                if l.lattice_points.contains_key(&coord) {
                    corrupted_coords.push(coord.clone());
                    println!("💥 Corrupted model at ({}, {}, {})", model, size, encoding);
                    println!("🔧 Total corrupted models: {}", corrupted_coords.len());
                } else {
                    println!("❌ Coordinate ({}, {}, {}) not found in lattice", model, size, encoding);
                }
            } else {
                println!("❌ No lattice generated yet. Use 'generate' first.");
            }
        }
        
        Some(&"recover") => {
            if let Some(ref l) = lattice {
                if corrupted_coords.is_empty() {
                    println!("✅ No corruption detected - lattice is healthy");
                } else {
                    println!("🔧 Attempting error correction for {} corrupted models...", corrupted_coords.len());
                    
                    match l.error_correct(corrupted_coords).await {
                        Ok(recovered_data) => {
                            println!("✅ Data recovered successfully!");
                            println!("📊 Recovered {} bytes: {:02x?}", recovered_data.len(), 
                                   &recovered_data[..recovered_data.len().min(8)]);
                            corrupted_coords.clear();
                        }
                        Err(e) => {
                            println!("❌ Recovery failed: {}", e);
                        }
                    }
                }
            } else {
                println!("❌ No lattice generated yet. Use 'generate' first.");
            }
        }
        
        Some(&"show") => {
            if let Some(ref l) = lattice {
                let size = parts.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                let models = l.get_models_by_size(size);
                
                println!("📊 Models at size level {}:", size);
                for model in models {
                    println!("  Model ({}, {}, {}): {:?} encoding, {:.1}x size, {} bytes",
                             model.coordinate.model,
                             model.coordinate.size, 
                             model.coordinate.encoding,
                             model.encoding_type,
                             model.size_factor,
                             model.data.len());
                }
            } else {
                println!("❌ No lattice generated yet. Use 'generate' first.");
            }
        }
        
        Some(&"status") => {
            if let Some(ref l) = lattice {
                println!("📊 RUST LATTICE STATUS:");
                println!("  Dimensions: {}×{}×{}", l.models, l.sizes, l.encodings);
                println!("  Total models: {}", l.lattice_points.len());
                println!("  Corrupted models: {}", corrupted_coords.len());
                println!("  Error correction: Reed-Solomon with {} parity symbols", 
                         l.error_correction.parity_symbols);
                
                // Show encoding distribution
                let mut encoding_counts = std::collections::HashMap::new();
                for model in l.lattice_points.values() {
                    *encoding_counts.entry(format!("{:?}", model.encoding_type)).or_insert(0) += 1;
                }
                
                println!("  Encoding distribution:");
                for (encoding, count) in encoding_counts {
                    println!("    {}: {} models", encoding, count);
                }
            } else {
                println!("❌ No lattice generated yet. Use 'generate' first.");
            }
        }
        
        _ => {
            println!("Unknown command: {}", command);
        }
    }
    
    Ok(())
}
