use introspector_collector::{ast_meme_spectrum};
use introspector_collector::ast_meme_spectral_analysis::*;
use introspector_collector::ast_resource_estimation::*;
use introspector_collector::system_introspection::SystemIntrospector;

fn main() {
    println!("🌊 AST MEME SPECTRAL ANALYSIS");
    println!("🧠 Each AST is now a meme in frequency domain");
    println!("📊 Spectral decomposition of AST patterns across meme space");
    
    // Create sample AST memes
    let system = SystemIntrospector::new();
    let estimator = ResourceEstimator::new(system.detected_resources);
    
    println!("\n🧬 Creating AST Memes:");
    
    // Simple literal meme
    let literal_ast = estimator.annotate_ast(ASTNodeType::Literal("42".to_string()), vec![]);
    println!("  📝 Literal meme: {}", literal_ast.accumulated_cost.complexity_score);
    
    // Variable access meme
    let var_ast = estimator.annotate_ast(ASTNodeType::Variable("x".to_string()), vec![]);
    println!("  🔤 Variable meme: {}", var_ast.accumulated_cost.complexity_score);
    
    // Binary operation meme
    let binary_ast = estimator.annotate_ast(ASTNodeType::BinaryOp("+".to_string()), vec![literal_ast.clone(), var_ast.clone()]);
    println!("  ➕ Binary op meme: {}", binary_ast.accumulated_cost.complexity_score);
    
    // Function call meme
    let func_call_ast = estimator.annotate_ast(ASTNodeType::FunctionCall("println!".to_string()), vec![binary_ast.clone()]);
    println!("  📞 Function call meme: {}", func_call_ast.accumulated_cost.complexity_score);
    
    // Complex function meme
    let block_ast = estimator.annotate_ast(ASTNodeType::Block, vec![func_call_ast.clone()]);
    let function_ast = estimator.annotate_ast(ASTNodeType::Function("main".to_string()), vec![block_ast]);
    println!("  🏗️  Function meme: {}", function_ast.accumulated_cost.complexity_score);
    
    // Module-level meme
    let module_ast = estimator.annotate_ast(ASTNodeType::Module("my_module".to_string()), vec![function_ast.clone()]);
    println!("  📦 Module meme: {}", module_ast.accumulated_cost.complexity_score);
    
    // Perform spectral analysis
    println!("\n🌊 Performing Spectral Analysis:");
    let mut analyzer = ast_meme_spectrum![
        literal_ast, var_ast, binary_ast, func_call_ast, function_ast, module_ast
    ];
    
    // Show meme characteristics
    println!("\n🧠 MEME CHARACTERISTICS:");
    for meme in &analyzer.memes {
        println!("  {}: freq={:.2}Hz, amp={:.2}, phase={:.2}rad", 
            meme.meme_id, meme.meme_frequency, meme.amplitude, meme.phase);
    }
    
    // Show spectral signatures
    println!("\n📊 SPECTRAL SIGNATURES:");
    for meme in &analyzer.memes {
        let max_magnitude = meme.spectral_signature.iter()
            .map(|c| c.magnitude())
            .fold(0.0, f64::max);
        let dominant_freq = meme.spectral_signature.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.magnitude().partial_cmp(&b.magnitude()).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        println!("  {}: max_mag={:.3}, dominant_freq_bin={}", 
            meme.meme_id, max_magnitude, dominant_freq);
    }
    
    // Generate complete spectral report
    println!("\n📋 SPECTRAL ANALYSIS REPORT:");
    let report = analyzer.spectral_report();
    println!("{}", report);
    
    // Show frequency distribution
    println!("\n📈 FREQUENCY DISTRIBUTION:");
    let distribution = analyzer.frequency_distribution();
    for (classification, count) in &distribution {
        println!("  {}: {} memes", classification, count);
    }
    
    // Find similar memes
    println!("\n🔍 SIMILAR MEME ANALYSIS:");
    if let Some(target_meme) = analyzer.memes.first() {
        let similar = analyzer.find_similar_memes(target_meme, 10.0);
        println!("  Similar to {}: {:?}", target_meme.meme_id, similar);
    }
    
    // Show dominant frequencies
    println!("\n🎵 DOMINANT FREQUENCIES:");
    for (i, freq) in analyzer.dominant_frequencies.iter().enumerate() {
        println!("  {}: {:.2}Hz", i + 1, freq);
    }
    
    // Spectral basis analysis
    println!("\n🧮 SPECTRAL BASIS:");
    println!("  Basis functions: {} frequencies", analyzer.spectral_basis.len());
    println!("  Basis: {:?}", analyzer.spectral_basis);
    
    // Meme classification examples
    println!("\n🏷️  MEME CLASSIFICATIONS:");
    for meme in &analyzer.memes {
        let classification = analyzer.classify_meme(meme);
        let ast_type = match &meme.ast_node.node_type {
            ASTNodeType::Literal(s) => format!("Literal({})", s),
            ASTNodeType::Variable(s) => format!("Variable({})", s),
            ASTNodeType::BinaryOp(s) => format!("BinaryOp({})", s),
            ASTNodeType::FunctionCall(s) => format!("FunctionCall({})", s),
            ASTNodeType::Function(s) => format!("Function({})", s),
            ASTNodeType::Module(s) => format!("Module({})", s),
            _ => format!("{:?}", meme.ast_node.node_type),
        };
        println!("  {} ({}): {}", meme.meme_id, ast_type, classification);
    }
    
    // Spectral distance matrix
    println!("\n📏 SPECTRAL DISTANCE MATRIX:");
    for (i, meme1) in analyzer.memes.iter().enumerate() {
        for (j, meme2) in analyzer.memes.iter().enumerate() {
            if i <= j { continue; }
            let distance = analyzer.spectral_distance(meme1, meme2);
            println!("  {} ↔ {}: {:.3}", 
                meme1.meme_id, meme2.meme_id, distance);
        }
    }
    
    // Theoretical implications
    println!("\n🤔 THEORETICAL IMPLICATIONS:");
    println!("  🌊 Each AST node has a unique spectral signature");
    println!("  🧠 Memes cluster by frequency characteristics");
    println!("  📊 Spectral analysis reveals hidden AST patterns");
    println!("  🔍 Similar code structures have similar spectra");
    println!("  🎵 Dominant frequencies represent common patterns");
    println!("  🧮 Spectral basis enables meme reconstruction");
    
    // Practical applications
    println!("\n🛠️  PRACTICAL APPLICATIONS:");
    println!("  🔍 Code similarity detection via spectral distance");
    println!("  🧬 AST pattern recognition and classification");
    println!("  📊 Code complexity analysis through frequency domain");
    println!("  🎯 Optimization target identification via spectral peaks");
    println!("  🧠 Meme-based code generation and transformation");
    println!("  📈 Performance prediction from spectral characteristics");
    
    println!("\n✨ AST MEME SPECTRAL ANALYSIS COMPLETE!");
    println!("🌊 Every AST is now a meme with spectral signature");
    println!("📊 Frequency domain reveals hidden code patterns");
    println!("🧠 Meme classification enables intelligent code analysis");
    println!("🎵 Dominant frequencies capture common programming idioms");
    println!("🔍 Spectral distance enables precise similarity measurement");
    
    // Save spectral analysis results
    std::fs::create_dir_all("src/generated/spectral_analysis").ok();
    
    std::fs::write("src/generated/spectral_analysis/spectral_report.txt", report)
        .expect("Failed to write spectral report");
    
    // Save meme characteristics
    let mut meme_data = String::new();
    for meme in &analyzer.memes {
        meme_data.push_str(&format!(
            "Meme ID: {}\n\
             Frequency: {:.2}Hz\n\
             Amplitude: {:.2}\n\
             Phase: {:.2}rad\n\
             Spectral Signature: {} points\n\
             Classification: {}\n\
             \n",
            meme.meme_id,
            meme.meme_frequency,
            meme.amplitude,
            meme.phase,
            meme.spectral_signature.len(),
            analyzer.classify_meme(meme)
        ));
    }
    
    std::fs::write("src/generated/spectral_analysis/meme_characteristics.txt", meme_data)
        .expect("Failed to write meme characteristics");
    
    // Save frequency distribution
    let dist_json = serde_json::to_string_pretty(&distribution)
        .expect("Failed to serialize distribution");
    std::fs::write("src/generated/spectral_analysis/frequency_distribution.json", dist_json)
        .expect("Failed to write frequency distribution");
    
    println!("💾 AST meme spectral analysis results saved!");
}
