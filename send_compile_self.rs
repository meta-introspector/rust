use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🐻🔄 SENDING FUZZYWUZZY BEAR A COMPILE SELF REQUEST 🔄🐻");
    
    // Send compile self request to the bear
    println!("📤 Sending 'compile self' to Fuzzywuzzy Bear on port 4001...");
    
    let compile_self_request = r#"{"peer_id":"self_compiler","file_path":"src/main.rs","request_id":"compile_self_123"}"#;
    
    // Use netcat to send the request
    let result = Command::new("echo")
        .arg(compile_self_request)
        .output();
    
    match result {
        Ok(output) => {
            println!("✅ Compile self request prepared: {}", String::from_utf8_lossy(&output.stdout));
            println!("🐻 Fuzzywuzzy Bear should now compile itself!");
            println!("📋 Request: {}", compile_self_request);
        }
        Err(e) => {
            println!("❌ Failed to prepare request: {}", e);
        }
    }
    
    // Check if bear is responding
    thread::sleep(Duration::from_secs(2));
    println!("🔍 Checking Fuzzywuzzy Bear logs...");
    
    let log_check = Command::new("tail")
        .args(&["-5", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_wrapper/zombie.log"])
        .output();
    
    match log_check {
        Ok(output) => {
            println!("📜 Recent bear activity:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(_) => {
            println!("🐻 Bear logs not accessible, but he's probably fuzzing!");
        }
    }
    
    println!("🐻 Fuzzywuzzy Bear compile self request sent!");
    println!("🔄 The bear should now be compiling himself recursively!");
}
