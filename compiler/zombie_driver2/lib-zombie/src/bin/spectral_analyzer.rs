use lib_zombie::ModuleSpectralProfiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Module Spectral Analysis - Flag Effect Profiler");
    println!("=================================================");
    
    let mut profiler = ModuleSpectralProfiler::new();
    
    // Profile all module/flag combinations
    profiler.profile_all_combinations()?;
    
    // Export spectral analysis
    profiler.export_analysis("spectral_analysis.json")?;
    
    println!("✅ Spectral analysis complete!");
    println!("📊 Results show frequency domain effects of compilation flags on:");
    println!("   - Enum variant processing patterns");
    println!("   - Function call frequencies");
    println!("   - Dominant compilation frequencies per flag");
    
    Ok(())
}
