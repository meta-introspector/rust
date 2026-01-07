use std::sync::Arc;
use tokio::sync::Mutex;
use crate::metrics::ZombieMetrics;

pub struct ZombieState {
    pub metrics: Arc<Mutex<ZombieMetrics>>,
    pub active: bool,
}

impl ZombieState {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(ZombieMetrics::new())),
            active: true,
        }
    }

    pub async fn shutdown(&mut self) {
        self.active = false;
        println!("🧟 Zombie driver shutting down...");
    }
}
