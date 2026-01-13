use lib_zombie::syn_analyzer::SynAnalyzer;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path)?;

    println!("🔬 Syn AST Analysis");
    println!("==================");
    println!("📁 File: {}", file_path);

    let mut analyzer = SynAnalyzer::new();
    let analysis = analyzer.analyze_file(file_path, &content)?;

    println!("📊 Functions: {}", analysis.functions.len());
    println!("📊 Structs: {}", analysis.structs.len());
    println!("📊 Enums: {}", analysis.enums.len());
    println!("📊 Impls: {}", analysis.impls.len());
    println!("📊 Modules: {}", analysis.modules.len());

    println!("\n🔝 Top Functions:");
    for (i, func) in analysis.functions.iter().take(5).enumerate() {
        println!("   {}. {} ({} params, pub: {})", i + 1, func.name, func.params, func.is_pub);
    }

    println!("\n🔝 Top Structs:");
    for (i, s) in analysis.structs.iter().take(5).enumerate() {
        println!("   {}. {} ({} fields, pub: {})", i + 1, s.name, s.fields, s.is_pub);
    }

    let output_path = format!("{}.syn_analysis.json", file_path);
    analyzer.export_analysis(&analysis, &output_path)?;
    println!("\n📁 Analysis saved to: {}", output_path);

    Ok(())
}
