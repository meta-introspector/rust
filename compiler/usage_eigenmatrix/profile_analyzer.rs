// profile_analyzer.rs - Analyze all collected profile dumps
// Generate comprehensive reports across all levels

use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct LevelProfile {
    level: usize,
    total_time_ms: f64,
    compile_time_ms: f64,
    success: bool,
    binary_size: usize,
    stderr_size: usize,
    source_lines: usize,
    source_bytes: usize,
    mkrust_macros: usize,
    features_count: usize,
    functions_count: usize,
    symbols_count: usize,
}

fn main() {
    println!("📊 Profile Analyzer - Processing all dumps");
    
    let mut profiles = Vec::new();
    
    // Collect all level profiles
    for level in 0..=5 {
        if let Some(profile) = load_level_profile(level) {
            profiles.push(profile);
        }
    }
    
    if profiles.is_empty() {
        println!("No profile data found. Run mkdbuild_profiler first.");
        return;
    }
    
    // Generate comprehensive analysis
    generate_comparative_analysis(&profiles);
    generate_scaling_analysis(&profiles);
    generate_efficiency_analysis(&profiles);
    
    println!("✅ Analysis complete - reports saved to analysis_reports/");
}

fn load_level_profile(level: usize) -> Option<LevelProfile> {
    let profile_dir = format!("profile_dumps/level_{}", level);
    
    // Read profile report
    let report_path = format!("{}/profile_report.md", profile_dir);
    let report_content = fs::read_to_string(&report_path).ok()?;
    
    // Read source diagnostics
    let diag_path = format!("{}/source_diagnostics.txt", profile_dir);
    let diag_content = fs::read_to_string(&diag_path).ok()?;
    
    // Parse data
    let mut profile = LevelProfile {
        level,
        total_time_ms: 0.0,
        compile_time_ms: 0.0,
        success: false,
        binary_size: 0,
        stderr_size: 0,
        source_lines: 0,
        source_bytes: 0,
        mkrust_macros: 0,
        features_count: 0,
        functions_count: 0,
        symbols_count: 0,
    };
    
    // Parse report
    for line in report_content.lines() {
        if line.starts_with("Total Time: ") {
            if let Some(time_str) = line.split("Total Time: ").nth(1) {
                if let Some(ms_str) = time_str.split("ms").next() {
                    profile.total_time_ms = ms_str.parse().unwrap_or(0.0);
                }
            }
        }
        if line.starts_with("Compile Time: ") {
            if let Some(time_str) = line.split("Compile Time: ").nth(1) {
                if let Some(ms_str) = time_str.split("ms").next() {
                    profile.compile_time_ms = ms_str.parse().unwrap_or(0.0);
                }
            }
        }
        if line.starts_with("Success: ") {
            profile.success = line.contains("true");
        }
        if line.starts_with("Binary: ") && line.contains("bytes") {
            if let Some(size_str) = line.split("Binary: ").nth(1) {
                if let Some(num_str) = size_str.split(" bytes").next() {
                    profile.binary_size = num_str.parse().unwrap_or(0);
                }
            }
        }
        if line.starts_with("Stderr: ") && line.contains("bytes") {
            if let Some(size_str) = line.split("Stderr: ").nth(1) {
                if let Some(num_str) = size_str.split(" bytes").next() {
                    profile.stderr_size = num_str.parse().unwrap_or(0);
                }
            }
        }
    }
    
    // Parse diagnostics
    for line in diag_content.lines() {
        if line.starts_with("mkrust_macros: ") {
            profile.mkrust_macros = line.split(": ").nth(1)?.parse().unwrap_or(0);
        }
        if line.starts_with("features_count: ") {
            profile.features_count = line.split(": ").nth(1)?.parse().unwrap_or(0);
        }
        if line.starts_with("functions_count: ") {
            profile.functions_count = line.split(": ").nth(1)?.parse().unwrap_or(0);
        }
        if line.starts_with("symbols_count: ") {
            profile.symbols_count = line.split(": ").nth(1)?.parse().unwrap_or(0);
        }
        if line.starts_with("source_lines: ") {
            profile.source_lines = line.split(": ").nth(1)?.parse().unwrap_or(0);
        }
        if line.starts_with("source_bytes: ") {
            profile.source_bytes = line.split(": ").nth(1)?.parse().unwrap_or(0);
        }
    }
    
    Some(profile)
}

fn generate_comparative_analysis(profiles: &[LevelProfile]) {
    fs::create_dir_all("analysis_reports").expect("Failed to create analysis dir");
    
    let mut report = String::new();
    report.push_str("# Comparative Analysis Across Levels\n\n");
    
    report.push_str("| Level | Time(ms) | Binary(KB) | Features | Functions | Symbols | Success |\n");
    report.push_str("|-------|----------|------------|----------|-----------|---------|----------|\n");
    
    for profile in profiles {
        report.push_str(&format!(
            "| {} | {:.1} | {:.1} | {} | {} | {} | {} |\n",
            profile.level,
            profile.total_time_ms,
            profile.binary_size as f64 / 1024.0,
            profile.features_count,
            profile.functions_count,
            profile.symbols_count,
            if profile.success { "✅" } else { "❌" }
        ));
    }
    
    report.push_str("\n## Key Insights\n");
    if profiles.len() > 1 {
        let first = &profiles[0];
        let last = &profiles[profiles.len() - 1];
        
        report.push_str(&format!("- **Complexity Growth**: Level {} → {} = {}x features, {}x functions\n",
            first.level, last.level,
            last.features_count as f64 / first.features_count.max(1) as f64,
            last.functions_count as f64 / first.functions_count.max(1) as f64
        ));
        
        report.push_str(&format!("- **Binary Size Growth**: {:.1}KB → {:.1}KB = {:.1}x increase\n",
            first.binary_size as f64 / 1024.0,
            last.binary_size as f64 / 1024.0,
            last.binary_size as f64 / first.binary_size.max(1) as f64
        ));
        
        report.push_str(&format!("- **Compile Time Growth**: {:.1}ms → {:.1}ms = {:.1}x increase\n",
            first.compile_time_ms,
            last.compile_time_ms,
            last.compile_time_ms / first.compile_time_ms.max(0.1)
        ));
    }
    
    fs::write("analysis_reports/comparative_analysis.md", report)
        .expect("Failed to write comparative analysis");
}

fn generate_scaling_analysis(profiles: &[LevelProfile]) {
    let mut report = String::new();
    report.push_str("# Scaling Analysis - Growth Patterns\n\n");
    
    report.push_str("## Complexity Scaling\n");
    for profile in profiles {
        let complexity_score = profile.features_count * profile.functions_count;
        report.push_str(&format!("Level {}: Complexity Score = {} ({}×{})\n",
            profile.level, complexity_score, profile.features_count, profile.functions_count));
    }
    
    report.push_str("\n## Performance Scaling\n");
    for profile in profiles {
        let efficiency = profile.functions_count as f64 / profile.compile_time_ms.max(0.1);
        report.push_str(&format!("Level {}: Efficiency = {:.2} functions/ms\n",
            profile.level, efficiency));
    }
    
    fs::write("analysis_reports/scaling_analysis.md", report)
        .expect("Failed to write scaling analysis");
}

fn generate_efficiency_analysis(profiles: &[LevelProfile]) {
    let mut report = String::new();
    report.push_str("# Efficiency Analysis - Resource Utilization\n\n");
    
    report.push_str("## Code Density\n");
    for profile in profiles {
        let density = profile.functions_count as f64 / profile.source_lines.max(1) as f64;
        report.push_str(&format!("Level {}: {:.3} functions per source line\n",
            profile.level, density));
    }
    
    report.push_str("\n## Binary Efficiency\n");
    for profile in profiles {
        let efficiency = profile.functions_count as f64 / (profile.binary_size as f64 / 1024.0);
        report.push_str(&format!("Level {}: {:.3} functions per KB\n",
            profile.level, efficiency));
    }
    
    report.push_str("\n## Macro Efficiency\n");
    for profile in profiles {
        let macro_efficiency = profile.functions_count as f64 / profile.mkrust_macros.max(1) as f64;
        report.push_str(&format!("Level {}: {:.1} functions per mkrust! macro\n",
            profile.level, macro_efficiency));
    }
    
    fs::write("analysis_reports/efficiency_analysis.md", report)
        .expect("Failed to write efficiency analysis");
    
    println!("📈 Generated {} profile analyses", profiles.len());
}
