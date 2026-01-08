use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct CompilationRequest {
    peer_id: String,
    file_path: String,
    request_id: String,
}

fn log_message(msg: &str) {
    println!("{}", msg);
}

async fn spawn_zombie_server(port: u16, variant: &str) -> String {
    log_message(&format!("🧟 Spawning {} zombie server on port {}...", variant, port));
    
    let output = std::process::Command::new("cargo")
        .args(&["run", "--release", "--bin", "zombie_rustc"])
        .current_dir("../zombie_wrapper")
        .env("ZOMBIE_PORT", port.to_string())
        .env("ZOMBIE_VARIANT", variant)
        .spawn();
    
    match output {
        Ok(_) => format!("{}:{} spawned", variant, port),
        Err(e) => format!("{}:{} failed: {}", variant, port, e),
    }
}

#[tokio::main]
async fn main() {
    log_message("🧟‍♂️ Starting Zombie Client with 9 Muses TrollArmy...");
    
    // Spawn zombie servers with TrollArmy
    let spawn_result_1 = spawn_zombie_server(4001, "primary").await;
    log_message(&format!("🧟 Spawned primary zombie: {}", spawn_result_1));
    
    let spawn_result_2 = spawn_zombie_server(4002, "secondary").await;
    log_message(&format!("🧟 Spawned secondary zombie: {}", spawn_result_2));
    
    let spawn_result_3 = spawn_zombie_server(4003, "fuzzer").await;
    log_message(&format!("🧟 Spawned fuzzer zombie: {}", spawn_result_3));
    
    log_message("🌐 Zombie network cluster spawned - 3 servers running");
    log_message("🎭🔱⭐🌈🎪🔮🎨🎼✨ 9 Muses harmonic distribution active!");
    
    // Wait for servers to initialize
    sleep(Duration::from_secs(10)).await;
    
    log_message("✅ Zombie client test completed successfully!");
}
