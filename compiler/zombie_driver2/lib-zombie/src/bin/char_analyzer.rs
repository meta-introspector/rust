use lib_zombie::CharLevelAnalyzer;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <rust_file>", args[0]);
        return Ok(());
    }
    
    println!("🔤 Character-Level Markov Analysis");
    println!("=================================");
    println!("Finding most likely next character transitions...");
    
    let mut analyzer = CharLevelAnalyzer::new();
    let file_path = &args[1];
    
    // Analyze character transitions
    analyzer.analyze_file(file_path)?;
    
    // Show top transitions
    analyzer.print_top_transitions(20);
    
    // Test prediction
    println!("\n🔮 Next character predictions:");
    let test_chars = ['f', 'n', ' ', 't', 'r', 's', 'e'];
    for &ch in &test_chars {
        if let Some((next_ch, prob)) = analyzer.predict_next_char(ch) {
            println!("   '{}' → '{}' ({:.1}%)", 
                    if ch == ' ' { "SPC" } else { &ch.to_string() },
                    if next_ch == ' ' { "SPC" } else { &next_ch.to_string() },
                    prob * 100.0);
        }
    }
    
    // Show essential arrows that must be preserved
    let essential_arrows = analyzer.get_essential_arrows();
    let must_preserve: Vec<_> = essential_arrows.iter()
        .filter(|a| a.must_preserve)
        .collect();
    
    println!("\n🔒 Essential arrows that MUST be preserved: {}", must_preserve.len());
    for (i, arrow) in must_preserve.iter().take(10).enumerate() {
        println!("   {}. '{}' → '{}' (strength: {:.3})", 
                i + 1,
                if arrow.pair.0 == ' ' { "SPC" } else { &arrow.pair.0.to_string() },
                if arrow.pair.1 == ' ' { "SPC" } else { &arrow.pair.1.to_string() },
                arrow.strength);
    }
    
    // Export analysis
    analyzer.export_analysis("char_analysis.json")?;
    
    println!("\n✅ Character-level analysis complete!");
    println!("🎯 Essential arrows identified - these character transitions");
    println!("   must be preserved in any compilation transformation");
    
    Ok(())
}
