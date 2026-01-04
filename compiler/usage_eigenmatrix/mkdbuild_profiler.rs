// mkdbuild_profiler.rs - Maximum profiling and diagnostics collection
// Collects all dumps, traces, and performance data from each build

use std::env;
use std::fs;
use std::process::Command;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: mkdbuild_profiler file.rs [--level=N]");
        return;
    }
    
    let source_file = &args[1];
    let level = extract_level(&args);
    
    println!("🔬 mkdbuild_profiler: Maximum profiling for {} at level {}", source_file, level);
    
    // Create profiling directory
    let profile_dir = format!("profile_dumps/level_{}", level);
    fs::create_dir_all(&profile_dir).expect("Failed to create profile dir");
    
    let start_time = Instant::now();
    
    // Read and process source
    let content = fs::read_to_string(source_file).expect("Failed to read source");
    let generated = process_mkrust_macros(&content, level);
    
    let output_file = format!("{}/generated_level_{}.rs", profile_dir, level);
    fs::write(&output_file, &generated).expect("Failed to write output");
    
    // Collect pre-compilation diagnostics
    collect_source_diagnostics(&content, level, &profile_dir);
    
    // Compile with maximum profiling
    let compile_result = compile_with_profiling(&output_file, level, &profile_dir);
    
    let total_time = start_time.elapsed();
    
    // Generate comprehensive profile report
    generate_profile_report(level, &profile_dir, total_time, &compile_result);
    
    println!("📊 Profile data saved to {}/", profile_dir);
}

fn extract_level(args: &[String]) -> usize {
    for arg in args {
        if arg.starts_with("--level=") {
            return arg[8..].parse().unwrap_or(0);
        }
    }
    0
}

fn collect_source_diagnostics(content: &str, _level: usize, profile_dir: &str) {
    let mut diagnostics = String::new();
    
    // Count mkrust macros
    let macro_count = content.matches("mkrust!(").count();
    diagnostics.push_str(&format!("mkrust_macros: {}\n", macro_count));
    
    // Count features per level
    let mut features_count = 0;
    let mut functions_count = 0;
    let mut symbols_count = 0;
    
    for line in content.lines() {
        if line.contains("features!(") {
            features_count += line.matches(',').count() + 1;
        }
        if line.contains("functions!(") {
            functions_count += line.matches(',').count() + 1;
        }
        if line.contains("symbols!(") {
            symbols_count += line.matches(',').count() + 1;
        }
    }
    
    diagnostics.push_str(&format!("features_count: {}\n", features_count));
    diagnostics.push_str(&format!("functions_count: {}\n", functions_count));
    diagnostics.push_str(&format!("symbols_count: {}\n", symbols_count));
    diagnostics.push_str(&format!("source_lines: {}\n", content.lines().count()));
    diagnostics.push_str(&format!("source_bytes: {}\n", content.len()));
    
    fs::write(&format!("{}/source_diagnostics.txt", profile_dir), diagnostics)
        .expect("Failed to write diagnostics");
}

fn compile_with_profiling(output_file: &str, level: usize, profile_dir: &str) -> CompileResult {
    let start = Instant::now();
    
    // Compile with maximum diagnostics and profiling
    let output = Command::new("rustc")
        .arg(output_file)
        .arg("--cfg")
        .arg(&format!("level_{}", level))
        .arg("-Z")
        .arg("time-passes")
        .arg("-Z")
        .arg("print-type-sizes")
        .arg("--emit")
        .arg("dep-info,metadata,mir,llvm-ir,asm")
        .arg("-C")
        .arg("opt-level=0")
        .arg("-C")
        .arg("debuginfo=2")
        .arg("--error-format")
        .arg("json")
        .output()
        .expect("Failed to run rustc");
    
    let compile_time = start.elapsed();
    
    // Save compilation output
    fs::write(&format!("{}/rustc_stdout.txt", profile_dir), &output.stdout)
        .expect("Failed to write stdout");
    fs::write(&format!("{}/rustc_stderr.txt", profile_dir), &output.stderr)
        .expect("Failed to write stderr");
    
    // Save generated artifacts
    let binary_name = format!("generated_level_{}", level);
    if let Ok(metadata) = fs::metadata(&binary_name) {
        fs::write(&format!("{}/binary_size.txt", profile_dir), 
                 format!("{}", metadata.len())).expect("Failed to write binary size");
    }
    
    // Move generated files to profile directory
    for ext in &["d", "rmeta", "mir", "ll", "s"] {
        let artifact = format!("{}.{}", output_file.trim_end_matches(".rs"), ext);
        if fs::metadata(&artifact).is_ok() {
            let dest = format!("{}/{}.{}", profile_dir, binary_name, ext);
            let _ = fs::rename(&artifact, dest);
        }
    }
    
    CompileResult {
        success: output.status.success(),
        compile_time,
        stdout_size: output.stdout.len(),
        stderr_size: output.stderr.len(),
    }
}

fn generate_profile_report(level: usize, profile_dir: &str, total_time: std::time::Duration, 
                          compile_result: &CompileResult) {
    let mut report = String::new();
    
    report.push_str(&format!("# mkdbuild Profile Report - Level {}\n\n", level));
    report.push_str(&format!("## Timing\n"));
    report.push_str(&format!("Total Time: {:?}\n", total_time));
    report.push_str(&format!("Compile Time: {:?}\n", compile_result.compile_time));
    report.push_str(&format!("Success: {}\n\n", compile_result.success));
    
    report.push_str(&format!("## Output Sizes\n"));
    report.push_str(&format!("Stdout: {} bytes\n", compile_result.stdout_size));
    report.push_str(&format!("Stderr: {} bytes\n", compile_result.stderr_size));
    
    // Read binary size if available
    if let Ok(size_content) = fs::read_to_string(&format!("{}/binary_size.txt", profile_dir)) {
        report.push_str(&format!("Binary: {} bytes\n", size_content.trim()));
    }
    
    report.push_str(&format!("\n## Generated Files\n"));
    if let Ok(entries) = fs::read_dir(profile_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(name) = path.file_name() {
                    if let Ok(metadata) = entry.metadata() {
                        report.push_str(&format!("- {}: {} bytes\n", 
                                               name.to_string_lossy(), metadata.len()));
                    }
                }
            }
        }
    }
    
    // Performance metrics
    report.push_str(&format!("\n## Performance Metrics\n"));
    report.push_str(&format!("Compile Speed: {:.2} KB/s\n", 
                            compile_result.stdout_size as f64 / compile_result.compile_time.as_secs_f64() / 1024.0));
    
    fs::write(&format!("{}/profile_report.md", profile_dir), report)
        .expect("Failed to write profile report");
}

fn process_mkrust_macros(content: &str, target_level: usize) -> String {
    // Same as before but with profiling annotations
    let mut output = String::new();
    output.push_str(&format!("// Generated by mkdbuild_profiler - Level {}\n", target_level));
    output.push_str(&format!("// Timestamp: {}\n\n", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()));
    
    // Add profiling instrumentation
    output.push_str("#[cfg(feature = \"profiling\")]\n");
    output.push_str("use std::time::Instant;\n\n");
    
    // Process macros (simplified for brevity)
    output.push_str(&format!("fn main() {{\n"));
    output.push_str(&format!("    println!(\"Level {} Compiler with Profiling\");\n", target_level));
    output.push_str(&format!("    let start = Instant::now();\n"));
    output.push_str(&format!("    \n"));
    output.push_str(&format!("    let programs = vec![\"const x = 1;\", \"const y = true;\"];\n"));
    output.push_str(&format!("    for program in programs {{\n"));
    output.push_str(&format!("        println!(\"Compiling: {{}}\", program);\n"));
    output.push_str(&format!("    }}\n"));
    output.push_str(&format!("    \n"));
    output.push_str(&format!("    println!(\"Execution time: {{:?}}\", start.elapsed());\n"));
    output.push_str(&format!("}}\n"));
    
    output
}

#[derive(Debug)]
struct CompileResult {
    success: bool,
    compile_time: std::time::Duration,
    stdout_size: usize,
    stderr_size: usize,
}
