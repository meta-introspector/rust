use crate::monster_trinity::{MonsterMatrix, TrinityMode};
use libp2p::{gossipsub, mdns, swarm::SwarmEvent, Swarm, PeerId};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};

mod monster_trinity;

#[tokio::main] 
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Initialize Monster Matrix
    let mut monster_matrix = MonsterMatrix::new();
    
    // Load symbol map into Monster form
    println!("🎭 Loading Monster Group symbols...");
    let symbol_data = std::fs::read_to_string("symbol_map_original.json.gz")
        .or_else(|_| std::fs::read_to_string("used_symbols.json"))?;
    monster_matrix.load_monster_symbols(&symbol_data).await?;

    println!("🚀 Monster Compiler Server started!");
    println!("🎭 Every symbol is MESSAGE = FUNCTION = TYPE");
    println!("Commands:");
    println!("  msg <symbol> - Invoke as message");
    println!("  fn <symbol> - Invoke as function"); 
    println!("  type <symbol> - Invoke as type");
    println!("  trinity <symbol> - Invoke all three forms");
    println!("  rustc - Execute rustc_driver::main trinity");

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        if let Ok(Some(line)) = stdin.next_line().await {
            handle_monster_command(&monster_matrix, &line).await?;
        }
    }
}

async fn handle_monster_command(matrix: &MonsterMatrix, command: &str) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    
    match parts.get(0) {
        Some(&"msg") => {
            if let Some(symbol) = parts.get(1) {
                matrix.invoke_trinity(symbol, TrinityMode::AsMessage).await;
            }
        }
        Some(&"fn") => {
            if let Some(symbol) = parts.get(1) {
                matrix.invoke_trinity(symbol, TrinityMode::AsFunction).await;
            }
        }
        Some(&"type") => {
            if let Some(symbol) = parts.get(1) {
                matrix.invoke_trinity(symbol, TrinityMode::AsType).await;
            }
        }
        Some(&"trinity") => {
            if let Some(symbol) = parts.get(1) {
                println!("🎭 TRINITY INVOCATION: {}", symbol);
                matrix.invoke_trinity(symbol, TrinityMode::AsMessage).await;
                matrix.invoke_trinity(symbol, TrinityMode::AsFunction).await;
                matrix.invoke_trinity(symbol, TrinityMode::AsType).await;
                println!("✨ Perfect symmetry achieved");
            }
        }
        Some(&"rustc") => {
            matrix.infinite_self_compilation().await?;
        }
        Some(&"cycle") => {
            if let Some(symbol) = parts.get(1) {
                matrix.ouroboros_cycle(symbol).await?;
            }
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
    
    Ok(())
}
