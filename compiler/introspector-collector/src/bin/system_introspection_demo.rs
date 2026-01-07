use introspector_collector::{check_system};
use introspector_collector::system_introspection::*;

fn main() {
    println!("🔍 SYSTEM INTROSPECTION");
    println!("🖥️  Checking real system resources vs massive lifeform requirements");
    println!("📊 Using free -g, /proc/cpuinfo, nvidia-smi, df");
    
    // Check system resources
    let introspector = check_system!();
    
    // Show detected resources
    println!("\n🖥️  DETECTED SYSTEM RESOURCES:");
    println!("💾 RAM: {}GB total, {}GB available", 
        introspector.detected_resources.total_ram_gb,
        introspector.detected_resources.available_ram_gb);
    println!("🔥 CPU: {} cores", introspector.detected_resources.cpu_cores);
    println!("🧠 CPU Model: {}", introspector.detected_resources.cpu_model);
    println!("🎮 GPU: {}GB RAM", introspector.detected_resources.gpu_ram_gb);
    println!("🎮 GPU Model: {}", introspector.detected_resources.gpu_model);
    println!("💿 Disk: {}GB available", introspector.detected_resources.disk_available_gb);
    
    // Show compatibility report
    println!("\n📋 COMPATIBILITY REPORT:");
    let report = introspector.compatibility_report();
    println!("{}", report);
    
    // Test specific lifeform complexities
    println!("\n🧪 LIFEFORM COMPATIBILITY TESTS:");
    for bits in [3, 5, 8, 10] {
        let can_run = check_system!(can_run, bits);
        let (ram_util, cpu_util, gpu_util) = check_system!(utilization, bits);
        
        let status = if can_run { "✅" } else { "❌" };
        println!("  {}-bit lifeform: {} (RAM: {:.1}%, CPU: {:.1}%, GPU: {:.1}%)", 
            bits, status, ram_util, cpu_util, gpu_util);
    }
    
    // Show monitoring commands
    println!("\n📊 SYSTEM MONITORING COMMANDS:");
    let commands = introspector.monitoring_commands();
    for (i, cmd) in commands.iter().enumerate() {
        println!("  {}: {}", i + 1, cmd);
    }
    
    // Real-time system check
    println!("\n⚡ REAL-TIME SYSTEM CHECK:");
    
    // Execute free -h
    if let Ok(output) = std::process::Command::new("free").arg("-h").output() {
        println!("💾 Memory Status:");
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().take(3) {
            println!("    {}", line);
        }
    }
    
    // Execute nproc
    if let Ok(output) = std::process::Command::new("nproc").output() {
        let cores = String::from_utf8_lossy(&output.stdout).trim();
        println!("🔥 CPU Cores: {}", cores);
    }
    
    // Execute nvidia-smi (if available)
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .arg("--query-gpu=name,memory.total,memory.used")
        .arg("--format=csv,noheader")
        .output() {
        println!("🎮 GPU Status:");
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            println!("    {}", line);
        }
    } else {
        println!("🎮 GPU Status: nvidia-smi not available");
    }
    
    // Execute df for disk space
    if let Ok(output) = std::process::Command::new("df").arg("-h").arg("/").output() {
        println!("💿 Disk Status:");
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().take(2) {
            println!("    {}", line);
        }
    }
    
    // Reality check
    println!("\n🎯 REALITY CHECK:");
    let max_runnable = introspector.can_run_lifeforms.iter()
        .filter(|(_, can_run)| *can_run)
        .map(|(bits, _)| *bits)
        .max();
    
    match max_runnable {
        Some(max_bits) => {
            println!("✅ Your system can run up to {}-bit lifeforms", max_bits);
            println!("🎮 That's {} possible states per lifeform", 1 << max_bits);
            println!("💡 You have a decent system for massive Game of Life!");
        },
        None => {
            println!("❌ Your system cannot run any massive lifeforms");
            println!("💡 You need datacenter-class hardware:");
            println!("   • Minimum 90GB RAM for 3-bit lifeforms");
            println!("   • Minimum 60 CPU cores");
            println!("   • Minimum 36GB GPU RAM");
            println!("   • Consider AWS p4d.24xlarge instances");
        }
    }
    
    // Show upgrade path
    println!("\n🚀 UPGRADE PATH:");
    if introspector.detected_resources.available_ram_gb < 90 {
        println!("  💾 RAM: Need {}GB more (current: {}GB)", 
            90 - introspector.detected_resources.available_ram_gb,
            introspector.detected_resources.available_ram_gb);
    }
    if introspector.detected_resources.cpu_cores < 60 {
        println!("  🔥 CPU: Need {} more cores (current: {})", 
            60 - introspector.detected_resources.cpu_cores,
            introspector.detected_resources.cpu_cores);
    }
    if introspector.detected_resources.gpu_ram_gb < 36 {
        println!("  🎮 GPU: Need {}GB more GPU RAM (current: {}GB)", 
            36 - introspector.detected_resources.gpu_ram_gb,
            introspector.detected_resources.gpu_ram_gb);
    }
    
    // Cost analysis
    println!("\n💰 COST ANALYSIS:");
    println!("  AWS p4d.24xlarge (for 10-bit lifeforms):");
    println!("    • 1,152GB RAM, 96 vCPUs, 8×40GB A100 GPUs");
    println!("    • Cost: ~$32/hour = $23,000/month");
    println!("    • Can run {} 10-bit lifeforms simultaneously", 
        std::cmp::min(1152 / 300, 96 / 200));
    
    println!("\n✨ SYSTEM INTROSPECTION COMPLETE!");
    println!("🔍 Real system resources detected and analyzed");
    println!("📊 Compatibility with massive lifeforms assessed");
    println!("💡 Upgrade recommendations provided");
    println!("🎯 Reality check: Most systems need serious upgrades!");
    
    // Save introspection results
    std::fs::create_dir_all("src/generated/system_introspection").ok();
    
    std::fs::write("src/generated/system_introspection/compatibility_report.txt", report)
        .expect("Failed to write compatibility report");
    
    let system_summary = format!(
        "SYSTEM INTROSPECTION SUMMARY\n\
         \n\
         DETECTED RESOURCES:\n\
         RAM: {}GB total, {}GB available\n\
         CPU: {} cores ({})\n\
         GPU: {}GB ({})\n\
         Disk: {}GB available\n\
         \n\
         MAXIMUM LIFEFORM COMPLEXITY: {:?}\n\
         \n\
         MONITORING COMMANDS:\n\
         {}\n\
         \n\
         Generated by system introspection at runtime.",
        introspector.detected_resources.total_ram_gb,
        introspector.detected_resources.available_ram_gb,
        introspector.detected_resources.cpu_cores,
        introspector.detected_resources.cpu_model,
        introspector.detected_resources.gpu_ram_gb,
        introspector.detected_resources.gpu_model,
        introspector.detected_resources.disk_available_gb,
        max_runnable,
        commands.join("\n")
    );
    
    std::fs::write("src/generated/system_introspection/system_summary.txt", system_summary)
        .expect("Failed to write system summary");
    
    println!("💾 System introspection results saved!");
}
