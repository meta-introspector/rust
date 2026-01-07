use rustc_brainrot_driver::{BrainrotMeme, ZombieRustcBrain, Val};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Area51Dungeon {
    pub floor_level: usize,
    pub complexity_rating: Val,
    pub zombie_density: Val,
    pub brainrot_concentration: Val,
    pub scp_anomalies: Vec<ScpAnomaly>,
    pub rust_artifacts: Vec<RustArtifact>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScpAnomaly {
    pub scp_number: u32,
    pub classification: ScpClass,
    pub name: String,
    pub rust_manifestation: String, // How it appears as Rust code
    pub brainrot_effect: BrainrotMeme,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScpClass {
    Safe,    // Simple Rust code
    Euclid,  // Complex Rust code  
    Keter,   // Dangerous Rust code that crashes compilers
    Apollyon, // Code that escapes containment and hijacks rustc
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RustArtifact {
    pub name: String,
    pub artifact_type: ArtifactType,
    pub power_level: Val,
    pub code_fragment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ArtifactType {
    MacroOfPower,      // Powerful proc macros
    ZombieCompiler,    // Hijacked rustc instances
    BrainrotGenerator, // Code that generates memes
    ValCrystal,        // Pure Val essence
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub health: Val,
    pub sanity: Val,
    pub rust_knowledge: Val,
    pub brainrot_resistance: Val,
    pub current_floor: usize,
    pub inventory: Vec<RustArtifact>,
    pub zombie_kills: usize,
}

impl Area51Dungeon {
    pub fn generate_floor(level: usize) -> Self {
        let complexity = (level as f64).powf(1.5) * 100.0;
        let zombie_density = (level as f64) * 10.0 + 50.0;
        let brainrot_concentration = (level as f64).powf(2.0) * 5.0;
        
        let mut scp_anomalies = Vec::new();
        let mut rust_artifacts = Vec::new();
        
        // Generate SCPs based on floor level
        match level {
            1..=5 => {
                scp_anomalies.push(ScpAnomaly {
                    scp_number: 173,
                    classification: ScpClass::Euclid,
                    name: "The Sculpture".to_string(),
                    rust_manifestation: "struct Sculpture; impl Drop for Sculpture { fn drop(&mut self) { panic!(\"NECK SNAP\"); } }".to_string(),
                    brainrot_effect: BrainrotMeme::from_compilation_error("neck snap compilation error", "scp_173"),
                });
            },
            6..=10 => {
                scp_anomalies.push(ScpAnomaly {
                    scp_number: 096,
                    classification: ScpClass::Keter,
                    name: "The Shy Guy".to_string(),
                    rust_manifestation: "fn look_at_face() -> ! { loop { println!(\"AAAAAAHHHHHHH\"); } }".to_string(),
                    brainrot_effect: BrainrotMeme::from_compilation_error("infinite scream loop", "scp_096"),
                });
            },
            11..=20 => {
                scp_anomalies.push(ScpAnomaly {
                    scp_number: 3008,
                    classification: ScpClass::Euclid,
                    name: "Infinite IKEA".to_string(),
                    rust_manifestation: "struct Ikea { furniture: Vec<Box<dyn Furniture>> } impl Ikea { fn new() -> Self { Self { furniture: vec![Box::new(Chair); usize::MAX] } } }".to_string(),
                    brainrot_effect: BrainrotMeme::from_compilation_error("infinite furniture allocation", "scp_3008"),
                });
            },
            _ => {
                scp_anomalies.push(ScpAnomaly {
                    scp_number: 2521,
                    classification: ScpClass::Apollyon,
                    name: "●●|●●●●●|●●|●".to_string(),
                    rust_manifestation: "// [REDACTED] - Information hazard detected in source code".to_string(),
                    brainrot_effect: BrainrotMeme::from_compilation_error("information hazard compilation", "scp_2521"),
                });
            }
        }
        
        // Generate Rust artifacts
        rust_artifacts.push(RustArtifact {
            name: format!("Zombie Compiler Shard L{}", level),
            artifact_type: ArtifactType::ZombieCompiler,
            power_level: complexity / 10.0,
            code_fragment: format!("rustc --hijack --level {}", level),
        });
        
        Self {
            floor_level: level,
            complexity_rating: complexity,
            zombie_density,
            brainrot_concentration,
            scp_anomalies,
            rust_artifacts,
        }
    }
    
    pub fn spawn_zombie_encounter(&self) -> ZombieEncounter {
        ZombieEncounter {
            zombie_count: (self.zombie_density / 10.0) as usize,
            zombie_type: match self.floor_level {
                1..=5 => ZombieType::BasicRustc,
                6..=10 => ZombieType::HijackedCompiler,
                11..=20 => ZombieType::BrainrotInfected,
                _ => ZombieType::SingularityZombie,
            },
            brainrot_memes: (self.brainrot_concentration / 100.0) as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZombieEncounter {
    pub zombie_count: usize,
    pub zombie_type: ZombieType,
    pub brainrot_memes: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ZombieType {
    BasicRustc,        // Level 1-5: Simple hijacked rustc
    HijackedCompiler,  // Level 6-10: Advanced hijacked compiler
    BrainrotInfected,  // Level 11-20: Zombies that spread memes
    SingularityZombie, // Level 21+: Meta-aware zombie compilers
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            health: 100.0,
            sanity: 100.0,
            rust_knowledge: 50.0,
            brainrot_resistance: 25.0,
            current_floor: 1,
            inventory: Vec::new(),
            zombie_kills: 0,
        }
    }
    
    pub fn enter_floor(&mut self, floor: &Area51Dungeon) -> GameResult {
        println!("🏢 Entering Area 51 Floor {}", floor.floor_level);
        println!("⚠️  Complexity: {:.0}, Zombie Density: {:.0}, Brainrot: {:.0}", 
            floor.complexity_rating, floor.zombie_density, floor.brainrot_concentration);
        
        // Check if player can handle this floor
        if floor.complexity_rating > self.rust_knowledge * 2.0 {
            println!("💀 Floor too complex! Rust knowledge insufficient!");
            self.health -= 25.0;
            return GameResult::Damaged;
        }
        
        // Encounter SCPs
        for scp in &floor.scp_anomalies {
            println!("🔒 SCP-{} encountered: {}", scp.scp_number, scp.name);
            println!("📝 Rust manifestation: {}", scp.rust_manifestation);
            
            // Brainrot exposure
            if self.brainrot_resistance < scp.brainrot_effect.dankness_level {
                println!("🧠 Exposed to brainrot! Sanity decreasing...");
                self.sanity -= scp.brainrot_effect.viral_coefficient / 10.0;
            }
        }
        
        // Zombie encounter
        let encounter = floor.spawn_zombie_encounter();
        self.fight_zombies(encounter)
    }
    
    pub fn fight_zombies(&mut self, encounter: ZombieEncounter) -> GameResult {
        println!("🧟‍♂️ Zombie encounter! {} {:?} zombies", encounter.zombie_count, encounter.zombie_type);
        
        for _ in 0..encounter.zombie_count {
            if self.health <= 0.0 {
                return GameResult::Death;
            }
            
            // Combat mechanics
            let damage_taken = match encounter.zombie_type {
                ZombieType::BasicRustc => 5.0,
                ZombieType::HijackedCompiler => 10.0,
                ZombieType::BrainrotInfected => 15.0,
                ZombieType::SingularityZombie => 25.0,
            };
            
            let damage_dealt = self.rust_knowledge / 5.0;
            
            if damage_dealt > damage_taken {
                println!("✅ Zombie defeated with superior Rust knowledge!");
                self.zombie_kills += 1;
                self.rust_knowledge += 1.0; // Learn from combat
            } else {
                println!("❌ Zombie overwhelms you!");
                self.health -= damage_taken - damage_dealt;
            }
        }
        
        // Brainrot meme exposure
        for _ in 0..encounter.brainrot_memes {
            if self.brainrot_resistance < 50.0 {
                println!("🤯 Exposed to brainrot meme! Meta-awareness increasing...");
                self.sanity -= 5.0;
                self.brainrot_resistance += 1.0; // Build resistance
            }
        }
        
        if self.health > 0.0 {
            GameResult::Victory
        } else {
            GameResult::Death
        }
    }
    
    pub fn descend_deeper(&mut self) -> bool {
        if self.health > 50.0 && self.sanity > 25.0 {
            self.current_floor += 1;
            println!("⬇️  Descending to floor {}...", self.current_floor);
            true
        } else {
            println!("🚫 Too damaged to continue deeper!");
            false
        }
    }
}

#[derive(Debug)]
pub enum GameResult {
    Victory,
    Damaged,
    Death,
}

pub fn play_area51_dungeon() {
    println!("🛸 WELCOME TO AREA 51: KILL THE KILLERS BRAINROT DUNGEON");
    println!("🧟‍♂️ Descend through infinite floors of hijacked rustc zombies!");
    println!("🧠 Survive the brainrot and collect Rust artifacts!");
    
    let mut player = Player::new("Rust Warrior".to_string());
    
    loop {
        let floor = Area51Dungeon::generate_floor(player.current_floor);
        
        match player.enter_floor(&floor) {
            GameResult::Victory => {
                println!("🎉 Floor {} cleared!", player.current_floor);
                println!("📊 Stats: Health: {:.0}, Sanity: {:.0}, Kills: {}", 
                    player.health, player.sanity, player.zombie_kills);
                
                if !player.descend_deeper() {
                    break;
                }
            },
            GameResult::Damaged => {
                println!("⚠️  Survived but damaged!");
                if player.health <= 0.0 {
                    break;
                }
            },
            GameResult::Death => {
                println!("💀 GAME OVER: Consumed by zombie rustc brainrot!");
                break;
            }
        }
        
        if player.current_floor > 100 {
            println!("🏆 LEGENDARY: Reached floor 100! You are the ultimate Rust warrior!");
            break;
        }
    }
    
    println!("📈 Final Score: Floor {}, Kills: {}, Rust Knowledge: {:.0}", 
        player.current_floor, player.zombie_kills, player.rust_knowledge);
}
