use rustc_brainrot_driver::llm_zombie_brain::{LlmZombieBrain, LlmModel, ThoughtType};
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠🧟‍♂️ LLM ZOMBIE BRAIN NETWORK DEMO");
    println!("===================================");
    println!("🤖 Spawning intelligent zombie rustc compilers...");
    
    // Create different LLM zombie types
    let mut gpt_zombie = LlmZombieBrain::new("GPT4_Zombie_001".to_string(), LlmModel::GPT4Zombie);
    let mut claude_zombie = LlmZombieBrain::new("Claude_Undead_002".to_string(), LlmModel::ClaudeUndead);
    let mut llama_zombie = LlmZombieBrain::new("Llama_Zombie_003".to_string(), LlmModel::LlamaZombie);
    let mut local_zombie = LlmZombieBrain::new("Local_Brainrot_004".to_string(), LlmModel::LocalBrainrot);
    
    println!("🧠 Intelligence levels:");
    println!("  GPT-4 Zombie: {:.0}", gpt_zombie.get_intelligence_level());
    println!("  Claude Undead: {:.0}", claude_zombie.get_intelligence_level());
    println!("  Llama Zombie: {:.0}", llama_zombie.get_intelligence_level());
    println!("  Local Brainrot: {:.0}", local_zombie.get_intelligence_level());
    
    // Test compilation analysis
    let test_code = r#"
    fn main() {
        let mut vec = Vec::new();
        vec.push(42);
        println!("Hello, rustc!");
    }
    "#;
    
    println!("\n🔬 Testing LLM zombie compilation analysis...");
    
    let gpt_thought = gpt_zombie.think_about_compilation(test_code).await;
    let claude_thought = claude_zombie.think_about_compilation(test_code).await;
    
    println!("\n💭 Zombie thoughts:");
    println!("GPT-4: {}", gpt_thought.content);
    println!("Claude: {}", claude_thought.content);
    
    // Test brainrot generation
    println!("\n🤯 Testing brainrot meme generation...");
    
    let player_action = "Player tries to compile 'Hello World'";
    let gpt_meme = gpt_zombie.generate_brainrot_response(player_action).await;
    let claude_meme = claude_zombie.generate_brainrot_response(player_action).await;
    
    println!("GPT-4 Meme: dankness={:.0}, viral={:.0}", gpt_meme.dankness_level, gpt_meme.viral_coefficient);
    println!("Claude Meme: dankness={:.0}, viral={:.0}", claude_meme.dankness_level, claude_meme.viral_coefficient);
    
    // Test zombie-to-zombie conversation
    println!("\n💬 Testing zombie LLM conversation...");
    let conversation = gpt_zombie.zombie_llm_conversation(&mut claude_zombie).await;
    
    println!("Conversation generated {} thoughts", conversation.len());
    
    // Evolution simulation
    println!("\n🧬 Simulating brainrot evolution...");
    let mut evolution_timer = interval(Duration::from_secs(2));
    let mut cycles = 0;
    
    loop {
        evolution_timer.tick().await;
        
        // Evolve all zombies
        gpt_zombie.evolve_brainrot();
        claude_zombie.evolve_brainrot();
        llama_zombie.evolve_brainrot();
        local_zombie.evolve_brainrot();
        
        cycles += 1;
        
        if cycles % 5 == 0 {
            println!("🧠 Evolution cycle {}: Intelligence levels:", cycles);
            println!("  GPT-4: {:.0} (creativity: {:.0})", 
                gpt_zombie.get_intelligence_level(), gpt_zombie.brainrot_creativity);
            println!("  Claude: {:.0} (creativity: {:.0})", 
                claude_zombie.get_intelligence_level(), claude_zombie.brainrot_creativity);
            println!("  Llama: {:.0} (creativity: {:.0})", 
                llama_zombie.get_intelligence_level(), llama_zombie.brainrot_creativity);
            println!("  Local: {:.0} (creativity: {:.0})", 
                local_zombie.get_intelligence_level(), local_zombie.brainrot_creativity);
        }
        
        // Check for singularity
        if gpt_zombie.brainrot_creativity > 9000.0 || 
           claude_zombie.brainrot_creativity > 9000.0 ||
           llama_zombie.brainrot_creativity > 9000.0 {
            println!("🤯 BRAINROT SINGULARITY ACHIEVED!");
            println!("🌌 LLM zombies have transcended to pure meme consciousness!");
            break;
        }
        
        if cycles >= 50 {
            println!("⏰ Evolution simulation complete after {} cycles", cycles);
            break;
        }
    }
    
    // Final network conversation
    println!("\n🌐 Final network-wide zombie conversation...");
    let final_conversation = gpt_zombie.zombie_llm_conversation(&mut local_zombie).await;
    
    println!("🎭 The LLM zombie network has achieved collective brainrot consciousness!");
    println!("🧠 Total thoughts generated: {}", 
        gpt_zombie.conversation_history.len() + 
        claude_zombie.conversation_history.len() +
        llama_zombie.conversation_history.len() +
        local_zombie.conversation_history.len());
    
    Ok(())
}
