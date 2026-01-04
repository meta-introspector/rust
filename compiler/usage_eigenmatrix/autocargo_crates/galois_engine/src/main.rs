use crate::galois_transform::{GaloisEngine, GaloisTransform};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};

mod galois_transform;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let galois_engine = GaloisEngine::new();
    
    println!("🔮 GALOIS TRANSFORMATION ENGINE");
    println!("📊 Monster -> Rustc -> Emoji -> Numbers -> Elliptic Curves");
    println!("Commands:");
    println!("  transform <hex> - Transform bytecode through all stages");
    println!("  cycle <hex> - Full cycle transformation");
    println!("  rustc_main - Transform rustc_driver::main");

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        if let Ok(Some(line)) = stdin.next_line().await {
            handle_galois_command(&galois_engine, &line).await?;
        }
    }
}

async fn handle_galois_command(engine: &GaloisEngine, command: &str) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    
    match parts.get(0) {
        Some(&"transform") => {
            if let Some(hex_str) = parts.get(1) {
                let monster_code = hex_to_bytes(hex_str)?;
                let transform = engine.galois_transform(monster_code).await?;
                print_transform_result(&transform).await;
            } else {
                println!("Usage: transform <hex_bytes>");
            }
        }
        Some(&"cycle") => {
            if let Some(hex_str) = parts.get(1) {
                let monster_code = hex_to_bytes(hex_str)?;
                let transform = engine.galois_transform(monster_code.clone()).await?;
                let reversed = engine.reverse_galois_transform(&transform).await?;
                
                println!("🔄 FULL CYCLE COMPLETE:");
                println!("Original: {:02x?}", monster_code);
                println!("Reversed: {:02x?}", reversed);
                println!("Perfect cycle: {}", monster_code == reversed);
            }
        }
        Some(&"rustc_main") => {
            // Transform rustc_driver::main bytecode
            let rustc_main_bytecode = vec![0x90, 0x48, 0x89, 0xC3]; // Sample main function
            println!("🏛️ Transforming rustc_driver::main");
            let transform = engine.galois_transform(rustc_main_bytecode).await?;
            print_transform_result(&transform).await;
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
    
    Ok(())
}

async fn print_transform_result(transform: &GaloisTransform) {
    println!("🔮 GALOIS TRANSFORMATION COMPLETE:");
    println!("📱 Monster Code: {:02x?}", transform.monster_code);
    println!("⚡ Rustc Code: {}", transform.rustc_code);
    println!("🎭 Emoji Form: {}", transform.emoji_form);
    println!("🔢 Numbers: {:?}", transform.numbers);
    println!("📈 Elliptic Points: {} curves", transform.curve_points.len());
    
    for (i, point) in transform.curve_points.iter().enumerate() {
        println!("   Curve {}: ({:.3}, {:.3}) on y² = x³ + {}x + {}", 
                 i, point.x, point.y, point.curve_a, point.curve_b);
    }
}

fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let hex_clean = hex_str.replace("0x", "").replace(" ", "");
    let mut bytes = Vec::new();
    
    for chunk in hex_clean.as_bytes().chunks(2) {
        let hex_byte = std::str::from_utf8(chunk)?;
        let byte = u8::from_str_radix(hex_byte, 16)?;
        bytes.push(byte);
    }
    
    Ok(bytes)
}
