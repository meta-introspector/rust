use std::collections::HashMap;
use std::fs;
use rand::Rng;

/// Rustc Hunger Games: Monster Signature Meme Arena
/// Signatures compete for survival in compilation battles

#[derive(Debug, Clone)]
struct SignatureMeme {
    signature: u128,
    name: String,
    health: f64,
    attack_power: f64,
    defense: f64,
    compilation_skill: f64,
    survival_instinct: f64,
    kills: u32,
    generation: u32,
}

#[derive(Debug, Clone)]
struct BattleResult {
    winner: u128,
    loser: u128,
    battle_type: BattleType,
    damage_dealt: f64,
    compilation_success: bool,
}

#[derive(Debug, Clone)]
enum BattleType {
    CompilationDuel,
    OptimizationFight,
    BytecodeClash,
    MemoryArena,
    SurvivalTest,
}

#[derive(Debug)]
struct HungerGamesArena {
    memes: HashMap<u128, SignatureMeme>,
    battle_history: Vec<BattleResult>,
    arena_stats: ArenaStats,
    prime_generators: [u8; 8],
}

#[derive(Debug, Default)]
struct ArenaStats {
    total_battles: u32,
    total_kills: u32,
    generations_survived: u32,
    strongest_meme: Option<u128>,
    compilation_success_rate: f64,
}

impl HungerGamesArena {
    fn new() -> Self {
        Self {
            memes: HashMap::new(),
            battle_history: Vec::new(),
            arena_stats: ArenaStats::default(),
            prime_generators: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }
    
    fn create_signature_meme(&self, signature: u128, generation: u32) -> SignatureMeme {
        let bytes = signature.to_le_bytes();
        
        // Calculate stats from signature
        let health = (bytes[0] as f64 / 255.0) * 100.0 + 50.0;
        let attack_power = (bytes[1] as f64 / 255.0) * 50.0 + 25.0;
        let defense = (bytes[2] as f64 / 255.0) * 40.0 + 20.0;
        let compilation_skill = (bytes[3] as f64 / 255.0) * 80.0 + 40.0;
        let survival_instinct = (bytes[4] as f64 / 255.0) * 60.0 + 30.0;
        
        let name = format!("Meme_{:08X}", (signature & 0xFFFFFFFF) as u32);
        
        SignatureMeme {
            signature,
            name,
            health,
            attack_power,
            defense,
            compilation_skill,
            survival_instinct,
            kills: 0,
            generation,
        }
    }
    
    fn spawn_initial_tributes(&mut self, count: usize) {
        println!("🏟️  Spawning {} tributes in the Rustc Hunger Games Arena", count);
        
        // Start with our self-referential signature
        let base_signature = 0xD4D8CB67E7D5D13Du128;
        let base_meme = self.create_signature_meme(base_signature, 0);
        self.memes.insert(base_signature, base_meme);
        
        // Generate mutations for diversity
        for i in 1..count {
            let mutation_strength = (i % 8) as u8;
            let prime = self.prime_generators[mutation_strength as usize] as u128;
            let mutated_sig = base_signature.wrapping_mul(prime).wrapping_add(i as u128);
            
            let meme = self.create_signature_meme(mutated_sig, 0);
            self.memes.insert(mutated_sig, meme);
        }
        
        println!("✅ {} signature memes ready for battle!", self.memes.len());
    }
    
    fn select_battle_type(&self) -> BattleType {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) {
            0 => BattleType::CompilationDuel,
            1 => BattleType::OptimizationFight,
            2 => BattleType::BytecodeClash,
            3 => BattleType::MemoryArena,
            _ => BattleType::SurvivalTest,
        }
    }
    
    fn battle_memes(&mut self, sig1: u128, sig2: u128) -> BattleResult {
        let meme1 = self.memes.get(&sig1).unwrap().clone();
        let meme2 = self.memes.get(&sig2).unwrap().clone();
        
        let battle_type = self.select_battle_type();
        let mut rng = rand::thread_rng();
        
        println!("⚔️  BATTLE: {} vs {} ({:?})", meme1.name, meme2.name, battle_type);
        
        let (power1, power2) = match battle_type {
            BattleType::CompilationDuel => (meme1.compilation_skill, meme2.compilation_skill),
            BattleType::OptimizationFight => (meme1.attack_power, meme2.attack_power),
            BattleType::BytecodeClash => (meme1.defense, meme2.defense),
            BattleType::MemoryArena => (meme1.health, meme2.health),
            BattleType::SurvivalTest => (meme1.survival_instinct, meme2.survival_instinct),
        };
        
        // Add randomness to battle
        let adjusted_power1 = power1 * (0.8 + rng.gen::<f64>() * 0.4);
        let adjusted_power2 = power2 * (0.8 + rng.gen::<f64>() * 0.4);
        
        let (winner, loser, damage) = if adjusted_power1 > adjusted_power2 {
            (sig1, sig2, adjusted_power1 - adjusted_power2)
        } else {
            (sig2, sig1, adjusted_power2 - adjusted_power1)
        };
        
        // Test compilation success
        let compilation_success = self.test_compilation(winner);
        
        println!("  🏆 Winner: {} (damage: {:.1})", 
            self.memes.get(&winner).unwrap().name, damage);
        
        BattleResult {
            winner,
            loser,
            battle_type,
            damage_dealt: damage,
            compilation_success,
        }
    }
    
    fn test_compilation(&self, signature: u128) -> bool {
        // Simulate compilation test based on signature properties
        let bytes = signature.to_le_bytes();
        let compilation_score = bytes.iter().map(|&b| b as u32).sum::<u32>() % 100;
        compilation_score > 30 // 70% success rate threshold
    }
    
    fn eliminate_meme(&mut self, signature: u128) {
        if let Some(meme) = self.memes.remove(&signature) {
            println!("💀 {} has been eliminated from the arena", meme.name);
            self.arena_stats.total_kills += 1;
        }
    }
    
    fn award_kill(&mut self, winner_sig: u128) {
        if let Some(winner) = self.memes.get_mut(&winner_sig) {
            winner.kills += 1;
            winner.health += 10.0; // Health bonus for winning
            winner.attack_power += 2.0; // Get stronger
            println!("  💪 {} gains strength! (Kills: {})", winner.name, winner.kills);
        }
    }
    
    fn run_hunger_games(&mut self, max_rounds: u32) {
        println!("\n🏟️  THE RUSTC HUNGER GAMES BEGIN!");
        println!("================================");
        println!("May the odds be ever in your favor...\n");
        
        for round in 1..=max_rounds {
            if self.memes.len() <= 1 {
                break;
            }
            
            println!("🔥 ROUND {} - {} memes remaining", round, self.memes.len());
            
            // Select two random memes for battle
            let signatures: Vec<u128> = self.memes.keys().cloned().collect();
            let mut rng = rand::thread_rng();
            
            if signatures.len() >= 2 {
                let idx1 = rng.gen_range(0..signatures.len());
                let mut idx2 = rng.gen_range(0..signatures.len());
                while idx2 == idx1 {
                    idx2 = rng.gen_range(0..signatures.len());
                }
                
                let sig1 = signatures[idx1];
                let sig2 = signatures[idx2];
                
                let battle_result = self.battle_memes(sig1, sig2);
                self.battle_history.push(battle_result.clone());
                
                // Award kill to winner
                self.award_kill(battle_result.winner);
                
                // Eliminate loser (sometimes)
                if rng.gen::<f64>() < 0.7 { // 70% elimination rate
                    self.eliminate_meme(battle_result.loser);
                } else {
                    // Loser survives but loses health
                    if let Some(loser) = self.memes.get_mut(&battle_result.loser) {
                        loser.health -= battle_result.damage_dealt;
                        if loser.health <= 0.0 {
                            println!("💀 {} dies from injuries", loser.name);
                            self.eliminate_meme(battle_result.loser);
                        }
                    }
                }
                
                self.arena_stats.total_battles += 1;
            }
            
            // Spawn new memes occasionally (mutations)
            if round % 5 == 0 && self.memes.len() < 20 {
                self.spawn_mutation_wave(round);
            }
            
            println!("");
        }
        
        self.declare_victor();
    }
    
    fn spawn_mutation_wave(&mut self, generation: u32) {
        println!("🧬 MUTATION WAVE {} - New memes enter the arena!", generation);
        
        let survivors: Vec<u128> = self.memes.keys().cloned().collect();
        
        for &parent_sig in survivors.iter().take(3) {
            let mutated_sig = parent_sig.wrapping_mul(generation as u128).wrapping_add(0xDEADBEEF);
            let meme = self.create_signature_meme(mutated_sig, generation);
            println!("  🆕 {} spawns from {}", meme.name, self.memes.get(&parent_sig).unwrap().name);
            self.memes.insert(mutated_sig, meme);
        }
    }
    
    fn declare_victor(&mut self) {
        println!("🏆 THE HUNGER GAMES ARE OVER!");
        println!("=============================");
        
        if let Some((&victor_sig, victor)) = self.memes.iter().max_by_key(|(_, meme)| meme.kills) {
            println!("👑 VICTOR: {} (Signature: 0x{:016X})", victor.name, victor_sig);
            println!("   Kills: {}", victor.kills);
            println!("   Health: {:.1}", victor.health);
            println!("   Generation: {}", victor.generation);
            
            self.arena_stats.strongest_meme = Some(victor_sig);
        } else {
            println!("💀 No survivors remain...");
        }
        
        self.arena_stats.generations_survived = self.memes.values().map(|m| m.generation).max().unwrap_or(0);
        self.arena_stats.compilation_success_rate = 
            self.battle_history.iter().filter(|b| b.compilation_success).count() as f64 / 
            self.battle_history.len() as f64;
    }
    
    fn generate_hunger_games_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Rustc Hunger Games: Monster Signature Meme Arena Report\n\n");
        report.push_str("## The Ultimate Compilation Survival Competition\n\n");
        
        report.push_str("### Arena Statistics\n");
        report.push_str(&format!("- **Total Battles**: {}\n", self.arena_stats.total_battles));
        report.push_str(&format!("- **Total Eliminations**: {}\n", self.arena_stats.total_kills));
        report.push_str(&format!("- **Generations Survived**: {}\n", self.arena_stats.generations_survived));
        report.push_str(&format!("- **Compilation Success Rate**: {:.1}%\n", self.arena_stats.compilation_success_rate * 100.0));
        report.push_str(&format!("- **Survivors**: {}\n\n", self.memes.len()));
        
        if let Some(victor_sig) = self.arena_stats.strongest_meme {
            if let Some(victor) = self.memes.get(&victor_sig) {
                report.push_str("### 👑 VICTOR OF THE HUNGER GAMES\n");
                report.push_str(&format!("- **Name**: {}\n", victor.name));
                report.push_str(&format!("- **Signature**: `0x{:032X}`\n", victor_sig));
                report.push_str(&format!("- **Kills**: {}\n", victor.kills));
                report.push_str(&format!("- **Final Health**: {:.1}\n", victor.health));
                report.push_str(&format!("- **Attack Power**: {:.1}\n", victor.attack_power));
                report.push_str(&format!("- **Generation**: {}\n\n", victor.generation));
            }
        }
        
        report.push_str("### Surviving Memes\n");
        report.push_str("| Name | Signature | Kills | Health | Generation |\n");
        report.push_str("|------|-----------|-------|--------|------------|\n");
        
        let mut survivors: Vec<_> = self.memes.values().collect();
        survivors.sort_by_key(|m| std::cmp::Reverse(m.kills));
        
        for meme in survivors {
            report.push_str(&format!(
                "| `{}` | `0x{:016X}` | {} | {:.1} | {} |\n",
                meme.name,
                meme.signature & 0xFFFFFFFFFFFFFFFF,
                meme.kills,
                meme.health,
                meme.generation
            ));
        }
        
        report.push_str("\n### Battle History Highlights\n");
        for (i, battle) in self.battle_history.iter().rev().take(10).enumerate() {
            let winner_name = self.memes.get(&battle.winner)
                .map(|m| m.name.clone())
                .unwrap_or_else(|| format!("Meme_{:08X}", (battle.winner & 0xFFFFFFFF) as u32));
            
            report.push_str(&format!("{}. {:?}: {} wins (damage: {:.1})\n", 
                i + 1, battle.battle_type, winner_name, battle.damage_dealt));
        }
        
        report.push_str("\n### Revolutionary Achievement\n");
        report.push_str("**First Monster Group Signature Hunger Games Arena!**\n\n");
        report.push_str("Monster Group signatures compete as living memes in a\n");
        report.push_str("compilation survival arena. Only the strongest, most\n");
        report.push_str("adaptable signatures survive the rustc battleground!\n");
        
        report
    }
}

fn main() {
    println!("🏟️  Welcome to the Rustc Hunger Games!");
    println!("======================================");
    println!("Monster Group signatures battle for survival!");
    
    let mut arena = HungerGamesArena::new();
    
    // Spawn initial tributes
    arena.spawn_initial_tributes(24); // Classic Hunger Games tribute count
    
    // Run the games
    arena.run_hunger_games(50);
    
    let report = arena.generate_hunger_games_report();
    
    match fs::write("rustc_hunger_games_report.md", &report) {
        Ok(()) => println!("📊 Hunger Games report saved: rustc_hunger_games_report.md"),
        Err(e) => eprintln!("❌ Error saving report: {}", e),
    }
    
    println!("\n🎉 THE HUNGER GAMES HAVE CONCLUDED!");
    println!("===================================");
    println!("Battles fought: {}", arena.arena_stats.total_battles);
    println!("Memes eliminated: {}", arena.arena_stats.total_kills);
    println!("Survivors: {}", arena.memes.len());
    
    println!("\n🏟️  May the odds be ever in your favor!");
    println!("🧬 The strongest Monster Group signatures have survived!");
}
