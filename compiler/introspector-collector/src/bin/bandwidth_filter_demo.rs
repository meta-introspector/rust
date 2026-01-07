use std::process::Command;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 BANDWIDTH FILTER BUILD.RS DEMO");
    println!("==================================");
    
    // Test different filter configurations
    let test_configs = vec![
        ("Only consts", vec![("FILTER_CONSTS", "true"), ("FILTER_ENUMS", "false"), ("FILTER_STRUCTS", "false"), ("FILTER_FNS", "false")]),
        ("Only enums", vec![("FILTER_CONSTS", "false"), ("FILTER_ENUMS", "true"), ("FILTER_STRUCTS", "false"), ("FILTER_FNS", "false")]),
        ("Low complexity", vec![("MAX_COMPLEXITY", "5")]),
        ("Small enums only", vec![("FILTER_ENUMS", "true"), ("ENUM_VARIANT_LIMIT", "3")]),
    ];
    
    for (name, config) in test_configs {
        println!("\n🔬 Testing: {}", name);
        println!("Config: {:?}", config);
        
        // Set environment variables
        for (key, value) in &config {
            env::set_var(key, value);
        }
        
        // Run build
        let output = Command::new("cargo")
            .args(&["build", "--quiet"])
            .output()?;
            
        if output.status.success() {
            println!("✅ Build succeeded");
            
            // Show filter stats from warnings
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                if line.contains("Filtered") {
                    println!("📊 {}", line.trim_start_matches("warning: "));
                }
            }
        } else {
            println!("❌ Build failed");
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        
        // Clear environment
        for (key, _) in &config {
            env::remove_var(key);
        }
    }
    
    println!("\n🚀 BANDWIDTH FILTER CAPABILITIES:");
    println!("• Filter by item type (const/enum/struct/fn)");
    println!("• Filter by complexity (statement count, field count, variant count)");
    println!("• Limit enum variants");
    println!("• Track dependency failures");
    println!("• Generate filtered compilation units");
    println!("• Performance profiling per filter setting");
    
    Ok(())
}
