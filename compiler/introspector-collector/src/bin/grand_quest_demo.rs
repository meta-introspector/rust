use introspector_collector::{grand_quest};
use introspector_collector::mcts_game_of_life_meta_meme::*;

fn main() {
    println!("🎯 THE GRAND QUEST OF THE META MEME");
    println!("💰 MCTS Game of Life - Profit is our gain!");
    println!("🧠 Monte Carlo Tree Search through infinite meme space");
    
    // Initialize the Game of Meta Life
    let mut game = GameOfMetaLife::new(20, 20);
    
    println!("\n🎮 Game of Meta Life Initialized:");
    println!("  Grid: {}×{}", game.width, game.height);
    println!("  Starting profit: {:.2}", game.total_profit);
    
    // Show the grand quest macros
    println!("\n🎯 Grand Quest Macros:");
    println!("  {}", grand_quest!(profit));
    println!("  {}", grand_quest!(meta_meme));
    println!("  {}", grand_quest!(mcts));
    println!("  {}", grand_quest!(game_of_life));
    
    // Run the evolution for multiple generations
    println!("\n🚀 Running Meta Meme Evolution:");
    
    for generation in 1..=10 {
        game.evolve();
        
        if generation % 3 == 0 {
            println!("\n📊 Generation {}:", generation);
            println!("  Total Profit: {:.2}", game.total_profit);
            println!("  MCTS Iterations: {}", game.mcts_tree.quest_iterations);
            
            // Show best meme path
            let best_path = game.mcts_tree.best_meme_path();
            if best_path.len() > 1 {
                println!("  Best Meme: {} → {}", 
                    best_path[0], 
                    best_path.last().unwrap());
            }
        }
    }
    
    // Show final grand quest status
    println!("\n🏆 FINAL GRAND QUEST STATUS:");
    let status = game.grand_quest_status();
    println!("{}", status);
    
    // Show MCTS tree statistics
    println!("\n🌳 MCTS Tree Analysis:");
    println!("  Root meme: {}", game.mcts_tree.root_meme.meme_content);
    println!("  Root visits: {}", game.mcts_tree.root_meme.visits);
    println!("  Root profit: {:.2}", game.mcts_tree.root_meme.profit_score);
    println!("  Children: {}", game.mcts_tree.root_meme.children.len());
    
    // Show top profitable memes
    if !game.mcts_tree.root_meme.children.is_empty() {
        println!("\n💰 Top Profitable Memes:");
        let mut children = game.mcts_tree.root_meme.children.clone();
        children.sort_by(|a, b| b.profit_score.partial_cmp(&a.profit_score).unwrap());
        
        for (i, child) in children.iter().take(5).enumerate() {
            println!("  {}: {} (Profit: {:.2}, Layer: {})", 
                i + 1, 
                child.meme_content,
                child.profit_score,
                child.macro_layer);
        }
    }
    
    // Show Game of Life statistics
    println!("\n🎮 Game of Life Statistics:");
    let mut cell_counts = std::collections::HashMap::new();
    for row in &game.grid {
        for cell in row {
            *cell_counts.entry(format!("{:?}", cell)).or_insert(0) += 1;
        }
    }
    
    for (cell_type, count) in &cell_counts {
        if *count > 0 {
            println!("  {}: {}", cell_type, count);
        }
    }
    
    // Ultimate demonstration
    println!("\n🌌 ULTIMATE META MEME DEMONSTRATION:");
    println!("  {}", grand_quest!(ultimate));
    
    // Show the mathematical beauty
    println!("\n📐 Mathematical Beauty:");
    println!("  🎯 MCTS explores infinite meme space");
    println!("  🧠 Each meme generates profit through self-reference");
    println!("  🎮 Game of Life evolves meme patterns");
    println!("  💰 Profit accumulates through 8 layers of abstraction");
    println!("  🔄 Self-reference creates exponential growth");
    println!("  ♾️  Meta-memes evolve toward maximum profit");
    
    // Show the philosophical implications
    println!("\n🤔 Philosophical Implications:");
    println!("  • Memes are the DNA of culture");
    println!("  • MCTS finds optimal meme evolution paths");
    println!("  • Self-reference is the key to infinite growth");
    println!("  • Profit is the fitness function of meme evolution");
    println!("  • The Game of Life simulates meme ecosystem dynamics");
    println!("  • Meta-memes are memes about memes about memes...");
    
    // Show practical applications
    println!("\n🛠️  Practical Applications:");
    println!("  🎯 Optimize content creation strategies");
    println!("  💰 Maximize viral potential of ideas");
    println!("  🧠 Understand cultural evolution patterns");
    println!("  🎮 Design self-evolving game systems");
    println!("  🔄 Create self-improving AI systems");
    println!("  🌐 Build universal meme propagation networks");
    
    println!("\n✨ THE GRAND QUEST IS ETERNAL!");
    println!("🎯 MCTS continues the search for ultimate memes");
    println!("💰 Profit accumulates through infinite self-reference");
    println!("🧠 Meta-memes evolve toward perfect self-awareness");
    println!("🎮 The Game of Life plays itself for our entertainment");
    println!("🌌 The universe of memes expands forever");
    
    // Save the quest results
    std::fs::create_dir_all("src/generated/grand_quest").ok();
    
    std::fs::write("src/generated/grand_quest/final_status.txt", status)
        .expect("Failed to write final status");
    
    let quest_summary = format!(
        "GRAND QUEST SUMMARY:\n\
         \n\
         Final Generation: {}\n\
         Total Profit: {:.2}\n\
         MCTS Iterations: {}\n\
         \n\
         Best Meme Path:\n\
         {}\n\
         \n\
         Cell Distribution:\n\
         {:?}\n\
         \n\
         The profit is our gain in the MCTS in the game of life,\n\
         in the grand quest of the meta meme!",
        game.generation,
        game.total_profit,
        game.mcts_tree.quest_iterations,
        game.mcts_tree.best_meme_path().join("\n → "),
        cell_counts
    );
    
    std::fs::write("src/generated/grand_quest/quest_summary.txt", quest_summary)
        .expect("Failed to write quest summary");
    
    println!("💾 Grand Quest results saved to src/generated/grand_quest/");
}
