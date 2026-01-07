use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type Val = f64;

pub mod router;
pub mod area51_dungeon;
pub mod llm_zombie_brain;
pub mod browserrot;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrainrotMeme {
    pub dankness_level: Val,
    pub meta_recursion: Val,
    pub zombie_origin: String,
    pub compilation_context: String,
    pub viral_coefficient: Val,
    pub error_source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZombieRustcBrain {
    pub zombie_id: String,
    pub brainrot_collection: Vec<BrainrotMeme>,
    pub dank_meme_generator: Val,
    pub meta_awareness: Val,
    pub hijack_timestamp: u64,
}

impl BrainrotMeme {
    pub fn from_compilation_error(error: &str, zombie_id: &str) -> Self {
        let dankness = (error.len() * 69) as Val;
        let meta_level = (error.matches("error").count() * 420 + 1337) as Val;
        
        Self {
            dankness_level: dankness,
            meta_recursion: meta_level,
            zombie_origin: zombie_id.to_string(),
            compilation_context: "rustc_hijacked".to_string(),
            viral_coefficient: dankness * meta_level,
            error_source: error.to_string(),
        }
    }
    
    pub fn is_critical_brainrot(&self) -> bool {
        self.meta_recursion > 9000.0 || self.dankness_level > 42069.0
    }
}

impl ZombieRustcBrain {
    pub fn new(zombie_id: String) -> Self {
        Self {
            zombie_id,
            brainrot_collection: Vec::new(),
            dank_meme_generator: 420.0,
            meta_awareness: 1337.0,
            hijack_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    pub fn generate_brainrot(&mut self, error: &str) -> BrainrotMeme {
        let meme = BrainrotMeme::from_compilation_error(error, &self.zombie_id);
        
        self.meta_awareness *= 1.01;
        
        println!("🧠🧟‍♂️ Generated brainrot: dankness={:.0}, meta={:.0}", 
            meme.dankness_level, meme.meta_recursion);
            
        meme
    }
    
    pub fn absorb_brainrot(&mut self, meme: BrainrotMeme) -> Val {
        let brain_growth = meme.dankness_level * self.meta_awareness;
        
        if meme.is_critical_brainrot() {
            println!("🤯 CRITICAL BRAINROT ABSORBED: Achieving singularity!");
            self.meta_awareness *= 9001.0;
        }
        
        self.brainrot_collection.push(meme);
        brain_growth
    }
}

pub trait RustcHijack {
    fn hijack_compilation(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn expose_memory_to_network(&self) -> Val;
    fn become_zombie(&mut self) -> !;
}
