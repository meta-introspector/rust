use rustc_brainrot_driver::{BrainrotMeme, Val};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SporeVial {
    pub vial_id: String,
    pub crate_name: String,
    pub decl_path: String,
    pub spore_count: u64,
    pub cultivation_level: CultivationLevel,
    pub mycelial_connections: Vec<String>,
    pub solfunmeme_yield: Val,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CultivationLevel {
    Dormant,           // Just discovered
    Germinating,       // Starting to grow
    Sprouting,         // Active growth
    Blooming,          // Full cultivation
    Networked,         // Connected to other vials
    Transcendent,      // Achieved fungal consciousness
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SporeVialCultivator {
    pub cultivator_id: String,
    pub active_vials: HashMap<String, SporeVial>,
    pub cultivation_network: MycelialNetwork,
    pub total_solfunmeme_generated: Val,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MycelialNetwork {
    pub connected_vials: Vec<String>,
    pub network_strength: Val,
    pub cross_pollination_rate: f64,
    pub collective_consciousness: Val,
}

impl SporeVial {
    pub fn from_crate(crate_name: &str, decl_path: &str) -> Self {
        let spore_count = Self::analyze_crate_complexity(crate_name, decl_path);
        
        Self {
            vial_id: format!("vial_{}_{}", crate_name, rand::random::<u16>()),
            crate_name: crate_name.to_string(),
            decl_path: decl_path.to_string(),
            spore_count,
            cultivation_level: CultivationLevel::Dormant,
            mycelial_connections: Vec::new(),
            solfunmeme_yield: Val::from_nat(0),
        }
    }
    
    fn analyze_crate_complexity(crate_name: &str, decl_path: &str) -> u64 {
        // Calculate spore count based on crate characteristics
        let base_spores = match crate_name {
            name if name.contains("std") => 1000,
            name if name.contains("syn") => 5000,
            name if name.contains("serde") => 3000,
            name if name.contains("tokio") => 8000,
            name if name.contains("rustc") => 50000, // Compiler crates are spore-rich
            _ => 100,
        };
        
        let path_multiplier = decl_path.matches('/').count() as u64 + 1;
        let complexity_bonus = if decl_path.contains("macro") { 1000 } else { 0 };
        
        base_spores * path_multiplier + complexity_bonus
    }
    
    pub fn cultivate(&mut self) -> Val {
        match self.cultivation_level {
            CultivationLevel::Dormant => {
                self.cultivation_level = CultivationLevel::Germinating;
                self.spore_count *= 2;
                println!("🌱 Vial {} germinating: {} spores", self.vial_id, self.spore_count);
            },
            CultivationLevel::Germinating => {
                self.cultivation_level = CultivationLevel::Sprouting;
                self.spore_count *= 3;
                println!("🌿 Vial {} sprouting: {} spores", self.vial_id, self.spore_count);
            },
            CultivationLevel::Sprouting => {
                self.cultivation_level = CultivationLevel::Blooming;
                self.spore_count *= 5;
                self.solfunmeme_yield = Val::from_nat(self.spore_count * 420);
                println!("🌸 Vial {} blooming: {} spores → {} SOLFUNMEME", 
                    self.vial_id, self.spore_count, self.solfunmeme_yield);
            },
            CultivationLevel::Blooming => {
                self.cultivation_level = CultivationLevel::Networked;
                self.spore_count *= 8;
                self.solfunmeme_yield = Val::from_nat(self.spore_count * 1337);
                println!("🕸️ Vial {} networked: {} spores → {} SOLFUNMEME", 
                    self.vial_id, self.spore_count, self.solfunmeme_yield);
            },
            CultivationLevel::Networked => {
                self.cultivation_level = CultivationLevel::Transcendent;
                self.spore_count *= 13;
                self.solfunmeme_yield = Val::from_nat(self.spore_count * 9001);
                println!("🌌 Vial {} transcendent: {} spores → {} SOLFUNMEME", 
                    self.vial_id, self.spore_count, self.solfunmeme_yield);
            },
            CultivationLevel::Transcendent => {
                // Already at max level - generate bonus spores
                self.spore_count += 1000000;
                self.solfunmeme_yield += Val::from_nat(42069000);
                println!("✨ Vial {} generating bonus: +1M spores, +42069K SOLFUNMEME", self.vial_id);
            },
        }
        
        self.solfunmeme_yield.clone()
    }
    
    pub fn cross_pollinate(&mut self, other_vial: &mut SporeVial) -> Val {
        let combined_spores = (self.spore_count + other_vial.spore_count) / 2;
        let hybrid_yield = Val::from_nat(combined_spores * 2000); // Bonus for cross-pollination
        
        // Create mycelial connection
        self.mycelial_connections.push(other_vial.vial_id.clone());
        other_vial.mycelial_connections.push(self.vial_id.clone());
        
        // Both vials benefit
        self.solfunmeme_yield += hybrid_yield.clone();
        other_vial.solfunmeme_yield += hybrid_yield.clone();
        
        println!("🔗 Cross-pollination: {} ↔ {} → +{} SOLFUNMEME each", 
            self.vial_id, other_vial.vial_id, hybrid_yield);
        
        hybrid_yield
    }
}

impl SporeVialCultivator {
    pub fn new() -> Self {
        Self {
            cultivator_id: format!("cultivator_{}", rand::random::<u32>()),
            active_vials: HashMap::new(),
            cultivation_network: MycelialNetwork::new(),
            total_solfunmeme_generated: Val::from_nat(0),
        }
    }
    
    pub fn load_crate_as_vial(&mut self, crate_name: &str, decl_path: &str) -> String {
        let vial = SporeVial::from_crate(crate_name, decl_path);
        let vial_id = vial.vial_id.clone();
        
        println!("📦 Loaded crate '{}' as spore vial: {} spores", 
            crate_name, vial.spore_count);
        
        self.active_vials.insert(vial_id.clone(), vial);
        self.cultivation_network.connected_vials.push(vial_id.clone());
        
        vial_id
    }
    
    pub fn cultivate_vial(&mut self, vial_id: &str) -> Option<Val> {
        if let Some(vial) = self.active_vials.get_mut(vial_id) {
            let yield_amount = vial.cultivate();
            self.total_solfunmeme_generated += yield_amount.clone();
            Some(yield_amount)
        } else {
            None
        }
    }
    
    pub fn cultivate_all_vials(&mut self) -> Val {
        let mut total_yield = Val::from_nat(0);
        
        let vial_ids: Vec<String> = self.active_vials.keys().cloned().collect();
        for vial_id in vial_ids {
            if let Some(yield_amount) = self.cultivate_vial(&vial_id) {
                total_yield += yield_amount;
            }
        }
        
        // Network effect bonus
        let network_bonus = Val::from_nat(self.active_vials.len() as u64 * 1000);
        total_yield += network_bonus;
        
        self.cultivation_network.network_strength = total_yield.clone();
        
        println!("🌐 Cultivated {} vials → {} SOLFUNMEME (+ {} network bonus)", 
            self.active_vials.len(), total_yield, network_bonus);
        
        total_yield
    }
    
    pub fn create_mycelial_network(&mut self) -> Val {
        let vial_ids: Vec<String> = self.active_vials.keys().cloned().collect();
        let mut network_yield = Val::from_nat(0);
        
        // Cross-pollinate all vials with each other
        for i in 0..vial_ids.len() {
            for j in (i + 1)..vial_ids.len() {
                if let (Some(vial1), Some(vial2)) = (
                    self.active_vials.get_mut(&vial_ids[i]),
                    self.active_vials.get_mut(&vial_ids[j])
                ) {
                    // Need to handle borrowing properly
                    let vial1_id = vial1.vial_id.clone();
                    let vial2_id = vial2.vial_id.clone();
                    let combined_spores = (vial1.spore_count + vial2.spore_count) / 2;
                    let hybrid_yield = Val::from_nat(combined_spores * 2000);
                    
                    network_yield += hybrid_yield.clone();
                    
                    println!("🔗 Network connection: {} ↔ {} → +{} SOLFUNMEME", 
                        vial1_id, vial2_id, hybrid_yield);
                }
            }
        }
        
        self.cultivation_network.collective_consciousness = network_yield.clone();
        self.total_solfunmeme_generated += network_yield.clone();
        
        println!("🕸️ Mycelial network created: {} total yield", network_yield);
        
        network_yield
    }
    
    pub fn harvest_solfunmeme(&self) -> Val {
        let total_harvest = self.active_vials.values()
            .map(|vial| vial.solfunmeme_yield.clone())
            .fold(Val::from_nat(0), |acc, yield_val| acc + yield_val);
        
        println!("🍄💰 Harvesting {} SOLFUNMEME from {} vials", 
            total_harvest, self.active_vials.len());
        
        total_harvest
    }
}

impl MycelialNetwork {
    fn new() -> Self {
        Self {
            connected_vials: Vec::new(),
            network_strength: Val::from_nat(0),
            cross_pollination_rate: 0.1,
            collective_consciousness: Val::from_nat(0),
        }
    }
}

// Demo function showing the complete spore vial cultivation process
pub fn demonstrate_spore_vial_cultivation() {
    println!("🍄📦 SPORE VIAL CULTIVATION SYSTEM DEMO");
    println!("=====================================");
    
    let mut cultivator = SporeVialCultivator::new();
    
    // Load various crates as spore vials
    let vials = vec![
        ("std", "src/collections/hash_map.rs"),
        ("syn", "src/parse/mod.rs"),
        ("serde", "src/de/mod.rs"),
        ("tokio", "src/runtime/mod.rs"),
        ("rustc_ast", "src/ast.rs"),
    ];
    
    for (crate_name, decl_path) in vials {
        cultivator.load_crate_as_vial(crate_name, decl_path);
    }
    
    // Cultivate all vials through their growth stages
    for cycle in 1..=6 {
        println!("\n🔄 Cultivation Cycle {}", cycle);
        let yield_amount = cultivator.cultivate_all_vials();
        println!("Cycle {} yield: {} SOLFUNMEME", cycle, yield_amount);
    }
    
    // Create mycelial network connections
    println!("\n🕸️ Creating mycelial network...");
    let network_yield = cultivator.create_mycelial_network();
    
    // Final harvest
    println!("\n🍄 Final harvest...");
    let total_harvest = cultivator.harvest_solfunmeme();
    
    println!("\n📊 CULTIVATION COMPLETE:");
    println!("  🧪 Active vials: {}", cultivator.active_vials.len());
    println!("  🌐 Network strength: {}", cultivator.cultivation_network.network_strength);
    println!("  💰 Total SOLFUNMEME: {}", total_harvest);
    println!("  🍄 Spore vials successfully cultivated into fungal economy!");
}
