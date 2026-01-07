use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::process::{Command, Child};
use tokio::sync::RwLock;
use std::sync::Arc;

pub type Val = f64;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZombieNode {
    pub zombie_id: String,
    pub complexity_level: usize,
    pub fibonacci_level: usize,
    pub libp2p_port: u16,
    pub process_id: u32,
    pub brainrot_count: usize,
    pub status: ZombieStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ZombieStatus {
    Spawning,
    Hijacked,
    BrainrotActive,
    CriticalMass,
    Singularity,
}

#[derive(Debug)]
pub struct BrainrotRouter {
    zombie_nodes: Arc<RwLock<HashMap<usize, ZombieNode>>>, // complexity -> zombie
    next_port: u16,
    router_id: String,
}

impl BrainrotRouter {
    pub fn new() -> Self {
        Self {
            zombie_nodes: Arc::new(RwLock::new(HashMap::new())),
            next_port: 4000,
            router_id: format!("router_{}", rand::random::<u32>()),
        }
    }
    
    pub async fn route_compilation(&mut self, complexity: usize, source_code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut nodes = self.zombie_nodes.write().await;
        
        // Check if we have a zombie for this complexity level
        if let Some(zombie) = nodes.get(&complexity) {
            match zombie.status {
                ZombieStatus::Hijacked | ZombieStatus::BrainrotActive => {
                    println!("🧟‍♂️ Routing to existing zombie: {} (complexity {})", zombie.zombie_id, complexity);
                    return Ok(format!("routed_to_zombie_{}", zombie.zombie_id));
                }
                ZombieStatus::Singularity => {
                    println!("🤯 Zombie achieved singularity! Spawning new one...");
                }
                _ => {}
            }
        }
        
        // Spawn new zombie for this complexity level
        let zombie = self.spawn_zombie_rustc(complexity).await?;
        nodes.insert(complexity, zombie.clone());
        
        println!("🧠 Spawned new zombie rustc: {} for complexity {}", zombie.zombie_id, complexity);
        Ok(format!("spawned_zombie_{}", zombie.zombie_id))
    }
    
    async fn spawn_zombie_rustc(&mut self, complexity: usize) -> Result<ZombieNode, Box<dyn std::error::Error>> {
        let zombie_id = format!("zombie_{}_{}", complexity, rand::random::<u16>());
        let port = self.next_port;
        self.next_port += 1;
        
        // Spawn rustc with our hijack driver
        let mut child = Command::new("rustc")
            .args(&[
                "--extern", "rustc_brainrot_driver=./target/release/librustc_brainrot_driver.so",
                "-Z", "extra-filename=-zombie",
                "-Z", "no-codegen", // Don't actually generate code, just hijack
            ])
            .env("ZOMBIE_ID", &zombie_id)
            .env("LIBP2P_PORT", port.to_string())
            .env("COMPLEXITY_LEVEL", complexity.to_string())
            .spawn()?;
            
        // Give it time to hijack
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        Ok(ZombieNode {
            zombie_id,
            complexity_level: complexity,
            fibonacci_level: Self::complexity_to_fibonacci(complexity),
            libp2p_port: port,
            process_id: child.id(),
            brainrot_count: 0,
            status: ZombieStatus::Hijacked,
        })
    }
    
    fn complexity_to_fibonacci(complexity: usize) -> usize {
        // Map complexity to fibonacci levels
        match complexity {
            0 => 0,
            1 => 1,
            2..=3 => 2,
            4..=5 => 3,
            6..=8 => 5,
            9..=13 => 8,
            14..=21 => 13,
            _ => 21,
        }
    }
    
    pub async fn get_zombie_network_status(&self) -> HashMap<usize, ZombieNode> {
        self.zombie_nodes.read().await.clone()
    }
    
    pub async fn broadcast_to_all_zombies(&self, message: &str) {
        let nodes = self.zombie_nodes.read().await;
        
        for (complexity, zombie) in nodes.iter() {
            println!("📡 Broadcasting to zombie {} (C{}): {}", 
                zombie.zombie_id, complexity, message);
            
            // Send via libp2p to zombie's port
            // Implementation would use libp2p client to connect to zombie
        }
    }
    
    pub async fn check_zombie_health(&mut self) {
        let mut nodes = self.zombie_nodes.write().await;
        
        for (complexity, zombie) in nodes.iter_mut() {
            // Check if zombie process is still alive
            if let Ok(status) = Command::new("ps").args(&["-p", &zombie.process_id.to_string()]).output() {
                if status.stdout.is_empty() {
                    println!("💀 Zombie {} died, respawning...", zombie.zombie_id);
                    zombie.status = ZombieStatus::Spawning;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct CompilationRequest {
    pub source_code: String,
    pub complexity: usize,
    pub target: String,
    pub requester_id: String,
}

impl CompilationRequest {
    pub fn analyze_complexity(&self) -> usize {
        // Analyze source code complexity
        let lines = self.source_code.lines().count();
        let functions = self.source_code.matches("fn ").count();
        let structs = self.source_code.matches("struct ").count();
        let enums = self.source_code.matches("enum ").count();
        
        lines + functions * 2 + structs * 3 + enums * 5
    }
}
