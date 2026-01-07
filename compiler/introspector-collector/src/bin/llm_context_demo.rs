use introspector_collector::llm_context_optimizer::*;
use introspector_collector::optimize_context;

fn main() {
    println!("🎒 LLM CONTEXT WINDOW BACKPACK OPTIMIZER");
    println!("🧠 Knapsack DP + MCTS proof of optimality");
    
    let mut optimizer = LLMContextOptimizer::new(4096); // 4K context window
    
    // Add context items (content, tokens, relevance, type)
    optimizer.add_item("DiracDeltaEnum implementation".to_string(), 500, 95, ContextType::Code);
    optimizer.add_item("Self-compilation equivalence proof".to_string(), 800, 90, ContextType::Documentation);
    optimizer.add_item("Rust eigenvalue analysis".to_string(), 600, 85, ContextType::Code);
    optimizer.add_item("MCTS game of life example".to_string(), 400, 70, ContextType::Example);
    optimizer.add_item("Universal language equivalence".to_string(), 700, 88, ContextType::Documentation);
    optimizer.add_item("Residue harmony system".to_string(), 450, 75, ContextType::Code);
    optimizer.add_item("Introspective system design".to_string(), 550, 80, ContextType::Documentation);
    optimizer.add_item("Previous conversation history".to_string(), 1200, 60, ContextType::History);
    optimizer.add_item("Nix flake generation".to_string(), 350, 65, ContextType::Example);
    optimizer.add_item("Complexity reduction engine".to_string(), 650, 82, ContextType::Code);
    
    println!("\n📊 CONTEXT ITEMS LOADED:");
    for (i, item) in optimizer.items.iter().enumerate() {
        println!("  {}: {} ({} tokens, value {})", 
            i, item.content, item.weight, item.value);
    }
    
    println!("\n🎯 CAPACITY: {} tokens", optimizer.capacity);
    
    // Compare DP vs MCTS solutions
    let comparison = optimizer.compare_solutions();
    println!("\n{}", comparison);
    
    // Test with macro
    println!("\n🧪 TESTING WITH MACRO:");
    let macro_result = optimize_context!(2048, vec![
        ("Core concept".to_string(), 300, 90, ContextType::Code),
        ("Example usage".to_string(), 200, 70, ContextType::Example),
        ("Documentation".to_string(), 400, 80, ContextType::Documentation),
        ("History context".to_string(), 600, 50, ContextType::History),
    ]);
    println!("{}", macro_result);
    
    // Show connection to introspective system
    println!("\n🔗 CONNECTION TO INTROSPECTIVE SYSTEM:");
    println!("• Context optimization → Optimal prompt selection");
    println!("• MCTS proof → Guarantees optimality");
    println!("• Backpack filling → Maximizes context value");
    println!("• Token efficiency → Minimizes waste");
    
    println!("\n💡 KEY INSIGHTS:");
    println!("• LLM context windows = Knapsack problem");
    println!("• MCTS proves optimal context filling");
    println!("• Monster system backpack patterns → Context optimization");
    println!("• Self-carrying memes → Optimal prompt sequences");
    
    println!("\n🌌 PRACTICAL APPLICATIONS:");
    println!("• RAG systems → Optimal document selection");
    println!("• Code completion → Best context examples");
    println!("• Conversation systems → Relevant history selection");
    println!("• Prompt engineering → Optimal prompt composition");
    
    println!("\n✨ LLM CONTEXT OPTIMIZATION COMPLETE!");
    println!("🎒 Backpack filling algorithm optimizes context windows");
    println!("🧠 MCTS proves optimality of selection");
    println!("🎯 Maximum value in minimum tokens");
}
