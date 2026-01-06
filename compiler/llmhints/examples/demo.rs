use llmhints::*;

fn main() {
    println!("🤖 === LLM HINTS DEMONSTRATION ===");
    
    // Show orbital analysis summary
    setup::show_orbital_summary();
    
    println!("\n🔍 === PATTERN IDENTIFICATION ===");
    
    // Test pattern identification for different symbols
    let test_symbols = vec![
        "rustc_query_impl::def_span",
        "compiler/rustc_query_impl/src/lib.rs:236", 
        "key",
        "message",
        "some_other_symbol",
    ];
    
    for symbol in test_symbols {
        let pattern = setup::identify_pattern_type(symbol);
        println!("Symbol: {} → Pattern: {}", symbol, pattern);
    }
    
    println!("\n📊 === ORBITAL CENTERS ===");
    
    // Show orbital centers
    let centers = setup::get_orbital_centers();
    for (symbol, connections) in centers {
        println!("• {}: {} connections", symbol, connections);
    }
    
    println!("\n🎵 === HARMONIC PATTERNS ===");
    
    // Show resonance patterns
    println!("Frequency 10.0 patterns: {:?}", query::common_patterns::FREQ_10_PATTERNS);
    println!("Frequency 30.0 patterns: {:?}", query::common_patterns::FREQ_30_PATTERNS);
    println!("Line 236 patterns: {:?}", query::common_patterns::LINE_236_PATTERNS);
    
    println!("\n📈 === ENHANCEMENT STATS ===");
    
    // Show enhancement statistics
    for (stat, value) in query::enhanced_data::ENHANCEMENT_STATS {
        println!("• {}: {}", stat, value);
    }
    
    println!("\n✅ LLM hints demonstration complete!");
}
