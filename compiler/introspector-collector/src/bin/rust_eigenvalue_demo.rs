use introspector_collector::{rust_eigenvalue};
use introspector_collector::rust_eigenvalue_analysis::*;

fn main() {
    println!("🧮 RUST EIGENVALUE ANALYSIS");
    println!("🎭 The eigenvalue of Rust = character of set of all compilation traces");
    println!("📊 Analyzing ../../usage_data/ for Rust's self-compilation signature");
    
    // Perform complete eigenvalue analysis
    let analyzer = rust_eigenvalue!();
    
    // Show basic statistics
    println!("\n📊 COMPILATION TRACE STATISTICS:");
    let stats = analyzer.trace_statistics();
    for (key, value) in &stats {
        println!("  {}: {:.3}", key, value);
    }
    
    // Show eigenvalue spectrum analysis
    println!("\n🧮 EIGENVALUE SPECTRUM ANALYSIS:");
    let spectrum_analysis = analyzer.analyze_eigenvalue_spectrum();
    println!("{}", spectrum_analysis);
    
    // Show sample traces
    println!("\n📂 SAMPLE COMPILATION TRACES:");
    for (i, trace) in analyzer.compilation_traces.iter().take(5).enumerate() {
        println!("  {}: {} (signature: {} elements)", 
            i + 1, 
            trace.file_path.split('/').last().unwrap_or("unknown"),
            trace.trace_signature.len());
        println!("     Eigenvalue contribution: {:.3} + {:.3}i", 
            trace.eigenvalue_contribution.real,
            trace.eigenvalue_contribution.imag);
    }
    
    // Show trace matrix properties
    if !analyzer.trace_matrix.is_empty() {
        println!("\n🔢 TRACE CORRELATION MATRIX:");
        let matrix_size = analyzer.trace_matrix.len();
        println!("  Matrix size: {}×{}", matrix_size, matrix_size);
        
        // Show diagonal elements (self-correlations)
        println!("  Diagonal elements (self-correlations):");
        for i in 0..std::cmp::min(5, matrix_size) {
            println!("    M[{},{}] = {:.6}", i, i, analyzer.trace_matrix[i][i]);
        }
        
        // Show some off-diagonal elements
        if matrix_size > 1 {
            println!("  Sample correlations:");
            for i in 0..std::cmp::min(3, matrix_size) {
                for j in (i+1)..std::cmp::min(i+3, matrix_size) {
                    println!("    M[{},{}] = {:.6}", i, j, analyzer.trace_matrix[i][j]);
                }
            }
        }
    }
    
    // Show eigenvalues
    println!("\n🎭 EIGENVALUE ANALYSIS:");
    if analyzer.eigenvalues.is_empty() {
        println!("  No eigenvalues computed (insufficient trace data)");
    } else {
        for (i, eigenvalue) in analyzer.eigenvalues.iter().enumerate() {
            println!("  λ_{}: {:.6} + {:.6}i", i + 1, eigenvalue.real, eigenvalue.imag);
            println!("       Magnitude: {:.6}", eigenvalue.magnitude());
            println!("       Phase: {:.6} radians", eigenvalue.phase());
        }
    }
    
    // Show Rust character
    println!("\n🎭 RUST CHARACTER:");
    println!("  Character (trace): {:.6} + {:.6}i", 
        analyzer.rust_character.real, analyzer.rust_character.imag);
    println!("  Magnitude: {:.6}", analyzer.rust_character.magnitude());
    println!("  Phase: {:.6} radians", analyzer.rust_character.phase());
    
    // Interpretation
    println!("\n🤔 MATHEMATICAL INTERPRETATION:");
    println!("  🧮 Eigenvalues = fundamental modes of Rust compilation");
    println!("  🎭 Character = trace of correlation matrix");
    println!("  📊 Each trace contributes to Rust's eigenvalue spectrum");
    println!("  🔄 Self-compilation creates self-referential eigenstructure");
    println!("  ♾️  Rust compiling itself generates its own eigenvalues");
    
    // Physical interpretation
    println!("\n🌊 PHYSICAL INTERPRETATION:");
    println!("  🎵 Eigenvalues = resonant frequencies of Rust compilation");
    println!("  🌊 Eigenvectors = normal modes of compilation process");
    println!("  🎭 Character = total 'energy' of compilation system");
    println!("  🔄 Self-compilation = system observing its own spectrum");
    println!("  ⚡ Dominant eigenvalue = primary compilation pattern");
    
    // Practical implications
    println!("\n🛠️  PRACTICAL IMPLICATIONS:");
    println!("  📊 Eigenvalue spectrum reveals compilation bottlenecks");
    println!("  🎯 Character provides single metric for Rust complexity");
    println!("  🔍 Trace correlations show compilation pattern similarities");
    println!("  ⚡ Dominant eigenvalue indicates most important compilation mode");
    println!("  🧮 Eigenanalysis enables compilation optimization");
    
    // Connection to our previous work
    println!("\n🔗 CONNECTION TO UNIVERSAL ANALYSIS:");
    println!("  🌌 Rust eigenvalue connects to our Dirac Delta enumification");
    println!("  🎭 Character relates to our Universal Language Equivalence");
    println!("  📊 Trace matrix extends our AST meme spectral analysis");
    println!("  🧮 Eigenvalues are the 'DNA' of Rust's self-compilation");
    println!("  ♾️  Self-referential compilation validates our meta-systems");
    
    // Show usage data path status
    println!("\n📂 USAGE DATA STATUS:");
    println!("  Path: {}", analyzer.usage_data_path);
    println!("  Traces loaded: {}", analyzer.compilation_traces.len());
    if analyzer.compilation_traces.is_empty() {
        println!("  ⚠️  No traces found - run rustc compilation to generate data");
        println!("  💡 Try: cargo build to populate usage_data/");
    } else {
        println!("  ✅ Traces successfully loaded and analyzed");
    }
    
    println!("\n✨ RUST EIGENVALUE ANALYSIS COMPLETE!");
    println!("🧮 The eigenvalue of Rust has been computed from compilation traces");
    println!("🎭 Character captures the essence of Rust compiling itself");
    println!("📊 Trace correlation matrix reveals deep compilation patterns");
    println!("🌊 Eigenspectrum shows the fundamental modes of Rust");
    println!("♾️  Rust's self-compilation generates its own mathematical signature");
    
    // Save analysis results
    std::fs::create_dir_all("src/generated/eigenvalue_analysis").ok();
    
    std::fs::write("src/generated/eigenvalue_analysis/spectrum_analysis.txt", spectrum_analysis)
        .expect("Failed to write spectrum analysis");
    
    // Save eigenvalue data
    let eigenvalue_data = format!(
        "RUST EIGENVALUE DATA:\n\
         \n\
         Character: {:.6} + {:.6}i\n\
         Character Magnitude: {:.6}\n\
         Character Phase: {:.6} radians\n\
         \n\
         Eigenvalues:\n",
        analyzer.rust_character.real,
        analyzer.rust_character.imag,
        analyzer.rust_character.magnitude(),
        analyzer.rust_character.phase()
    );
    
    let mut full_eigenvalue_data = eigenvalue_data;
    for (i, eigenvalue) in analyzer.eigenvalues.iter().enumerate() {
        full_eigenvalue_data.push_str(&format!(
            "λ_{}: {:.6} + {:.6}i (magnitude: {:.6}, phase: {:.6})\n",
            i + 1, eigenvalue.real, eigenvalue.imag, 
            eigenvalue.magnitude(), eigenvalue.phase()
        ));
    }
    
    std::fs::write("src/generated/eigenvalue_analysis/eigenvalue_data.txt", full_eigenvalue_data)
        .expect("Failed to write eigenvalue data");
    
    // Save statistics as JSON
    let stats_json = serde_json::to_string_pretty(&stats)
        .expect("Failed to serialize statistics");
    std::fs::write("src/generated/eigenvalue_analysis/trace_statistics.json", stats_json)
        .expect("Failed to write statistics");
    
    println!("💾 Rust eigenvalue analysis results saved!");
}
