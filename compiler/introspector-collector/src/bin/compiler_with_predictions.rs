use std::process::Command;
use std::env;

fn main() {
    println!("🚀 Enhanced Rust Compiler with Predictive Analysis");
    
    // Run predictive analysis first
    println!("🔮 Running predictive analysis...");
    let prediction_result = Command::new("./target/debug/predictive_analyzer")
        .output();
    
    match prediction_result {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                eprintln!("Prediction warnings: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("⚠️  Could not run predictive analysis: {}", e);
        }
    }
    
    // Get original rustc arguments
    let args: Vec<String> = env::args().skip(1).collect();
    
    // Run the actual compilation with usage collection
    println!("🔧 Starting compilation with usage collection...");
    let mut cmd = Command::new("./target/debug/working_usage_collector");
    cmd.args(&args);
    
    // Set environment for usage collection
    cmd.env("USAGE_OUTPUT_DIR", "./test_usage_data");
    
    let result = cmd.status();
    
    match result {
        Ok(status) => {
            if status.success() {
                println!("✅ Compilation completed successfully");
            } else {
                println!("❌ Compilation failed with status: {}", status);
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to run compiler: {}", e);
            std::process::exit(1);
        }
    }
}
