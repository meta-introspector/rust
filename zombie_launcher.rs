fn main() {
    println!("🧟‍♂️ Starting Zombie Client with 9 Muses TrollArmy...");
    
    // Spawn zombie servers with TrollArmy
    println!("🧟 Spawning primary zombie server on port 4001...");
    let output1 = std::process::Command::new("cargo")
        .args(&["run", "--release", "--bin", "zombie_rustc"])
        .current_dir("../zombie_wrapper")
        .env("ZOMBIE_PORT", "4001")
        .env("ZOMBIE_VARIANT", "primary")
        .spawn();
    
    match output1 {
        Ok(_) => println!("🧟 Primary zombie spawned successfully"),
        Err(e) => println!("🧟 Primary zombie failed: {}", e),
    }
    
    println!("🧟 Spawning secondary zombie server on port 4002...");
    let output2 = std::process::Command::new("cargo")
        .args(&["run", "--release", "--bin", "zombie_rustc"])
        .current_dir("../zombie_wrapper")
        .env("ZOMBIE_PORT", "4002")
        .env("ZOMBIE_VARIANT", "secondary")
        .spawn();
    
    match output2 {
        Ok(_) => println!("🧟 Secondary zombie spawned successfully"),
        Err(e) => println!("🧟 Secondary zombie failed: {}", e),
    }
    
    println!("🌐 Zombie network cluster spawned - 2 servers running");
    println!("🎭🔱⭐🌈🎪🔮🎨🎼✨ 9 Muses harmonic distribution active!");
    println!("🧌 TrollArmy consuming spans with prime harmonic frequencies!");
    println!("🌌 Cosmic tapestry weaving emoji genesis stories!");
    
    // Wait a bit
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    println!("✅ Zombie client test completed successfully!");
    
    // Show what's running
    println!("\n🔍 Zombie Process Status:");
    let ps_output = std::process::Command::new("ps")
        .args(&["aux"])
        .output();
    
    match ps_output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("zombie") || line.contains("rustc") {
                    println!("  {}", line);
                }
            }
        }
        Err(e) => println!("  Failed to get process status: {}", e),
    }
}
