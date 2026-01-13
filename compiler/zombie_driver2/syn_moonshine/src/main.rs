use self_analysis::SelfAnalyzer;

mod self_analysis;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌙 MOONSHINE: SELF-TRANSFORMING ANALYSIS");
    println!("========================================");
    println!("Program analyzing itself from compile-time → runtime → shift");
    println!();

    // The program analyzes itself through all phases
    let report = SelfAnalyzer::analyze_self()?;

    // Output the complete analysis
    println!("\n📊 FINAL SELF-ANALYSIS REPORT");
    println!("==============================");

    println!("\n🌳 COMPILE-TIME VIEW:");
    println!("   Our AST patterns: {}", report.compile_time_view.our_ast.len());
    println!("   Syn AST patterns: {}", report.compile_time_view.syn_ast.len());
    println!(
        "   Mathematical signatures: {}",
        report.compile_time_view.mathematical_signatures.len()
    );

    println!("\n🔍 RUNTIME VIEW:");
    println!("   Our binary symbols: {}", report.runtime_view.our_symbols.len());
    println!("   Syn binary symbols: {}", report.runtime_view.syn_symbols.len());
    println!("   Binary signatures: {}", report.runtime_view.binary_signatures.len());

    println!("\n🔗 JOINED ANALYSIS:");
    println!("   Matched patterns: {}", report.joined_analysis.matched_patterns.len());
    println!("   Divergences: {}", report.joined_analysis.divergences.len());
    println!("   Resonance frequency: {:.3}", report.joined_analysis.resonance_frequency);

    println!("\n🌀 SHIFTED PERSPECTIVE:");
    println!(
        "   Compile→Runtime: {}",
        report.shifted_perspective.compile_time_becomes_runtime.len()
    );
    println!(
        "   Runtime→Compile: {}",
        report.shifted_perspective.runtime_becomes_compile_time.len()
    );
    println!(
        "   Transformation matrix: {}x{}",
        report.shifted_perspective.transformation_matrix.len(),
        report.shifted_perspective.transformation_matrix.get(0).map(|r| r.len()).unwrap_or(0)
    );

    // Show some examples
    println!("\n🎯 SAMPLE MATCHED PATTERNS:");
    for (i, (ast, binary)) in report.joined_analysis.matched_patterns.iter().take(5).enumerate() {
        println!("   {}: {} ↔ {}", i + 1, ast, binary);
    }

    println!("\n🔄 SAMPLE TRANSFORMATIONS:");
    for (i, transform) in
        report.shifted_perspective.compile_time_becomes_runtime.iter().take(3).enumerate()
    {
        println!("   {}: {}", i + 1, transform);
    }

    // Save the complete report
    let report_json = serde_json::to_string_pretty(&report)?;
    std::fs::write("moonshine_self_analysis_report.json", report_json)?;

    println!("\n💾 Complete analysis saved to: moonshine_self_analysis_report.json");
    println!("\n🌙 MOONSHINE SELF-ANALYSIS COMPLETE");
    println!("   The program has successfully analyzed itself from both");
    println!("   compile-time and runtime perspectives, joined them,");
    println!("   and shifted between the two viewpoints.");

    Ok(())
}
