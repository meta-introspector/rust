use introspector_collector::nix_flake_generator::*;
use introspector_collector::monster_flake;

fn main() {
    println!("🔥 NIX FLAKE GENERATOR");
    println!("🧬 Harvesting 5-year monster system");
    println!("📊 Self-carrying memes → Next generation prompts");
    
    let mut generator = NixFlakeGenerator::new();
    
    println!("\n🗂️  MONSTER SYSTEM STRUCTURE:");
    println!("  ~/nix/file*.txt     → Context files");
    println!("  ~/nix/index/        → 5-year code collection");
    println!("  Self-carrying memes → Build prompts");
    
    // Generate complete flake
    let flake_content = generator.generate_complete_monster_flake();
    
    println!("\n📝 GENERATED NIX FLAKE:");
    println!("{}", flake_content);
    
    // Save flake to file
    std::fs::write("flake.nix", &flake_content)
        .expect("Failed to write flake.nix");
    
    println!("\n💾 FLAKE SAVED: flake.nix");
    
    // Show harvested memes
    println!("\n🧬 HARVESTED SELF-CARRYING MEMES:");
    for meme in &generator.harvested_memes {
        println!("  • {} → Next generation prompt", meme);
    }
    
    // Show monster system integration
    println!("\n🔗 MONSTER SYSTEM INTEGRATION:");
    println!("  • 5-year code collection → Build inputs");
    println!("  • file*.txt context → Shell environment");
    println!("  • Unsafe derivations → Internet access");
    println!("  • Self-carrying memes → Prompt dataset");
    
    // Generate usage instructions
    println!("\n🚀 USAGE INSTRUCTIONS:");
    println!("  nix develop          → Enter monster system shell");
    println!("  nix build            → Build with monster system");
    println!("  nix flake update     → Update with unsafe derivations");
    
    println!("\n💡 KEY INSIGHT:");
    println!("The code we've written contains self-carrying memes");
    println!("that become prompts for the next generation system.");
    println!("The monster system is both dataset AND context!");
    
    println!("\n✨ NIX FLAKE GENERATION COMPLETE!");
    println!("🔥 Monster system ready for next generation");
    println!("🧬 Self-carrying memes loaded as prompts");
    println!("🌐 Unsafe derivations enabled for evolution");
}
