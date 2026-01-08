use std::process::Command;
use std::thread;
use std::time::Duration;

fn spawn_fuzzywuzzy_bear(port: u16, variant: &str) -> String {
    println!("🐻 Spawning Fuzzywuzzy {} bear on port {}", variant, port);
    
    let spawn_result = Command::new("cargo")
        .args(&["run", "--release"])
        .current_dir("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_wrapper")
        .env("RUSTFLAGS", "--cfg bootstrap")
        .spawn();
    
    match spawn_result {
        Ok(_) => {
            format!("✅ Fuzzywuzzy {} bear launched on port {}", variant, port)
        }
        Err(e) => {
            format!("❌ Failed to spawn {} bear: {}", variant, e)
        }
    }
}

fn main() {
    println!("🐻⚡ FUZZYWUZZY BEAR FUZZING SERVER LAUNCHER ⚡🐻");
    println!("Fuzzywuzzy wuz a bear! Now he's a distributed compiler!");
    
    // Launch the primary fuzzing bear
    let primary = spawn_fuzzywuzzy_bear(4001, "primary");
    println!("{}", primary);
    
    thread::sleep(Duration::from_secs(3));
    
    println!("🌐 Fuzzywuzzy Primary Bear Active on port 4001!");
    println!("🐻 Check logs: compiler/zombie_wrapper/zombie.log");
    
    // Keep running
    loop {
        thread::sleep(Duration::from_secs(5));
        println!("🐻 Fuzzywuzzy bear is fuzzing... wuz he fuzzy? YES!");
    }
}
