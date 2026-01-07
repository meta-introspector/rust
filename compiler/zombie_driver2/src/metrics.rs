use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZombieMetrics {
    pub compilation_count: u64,
    pub network_peers: u32,
    pub avg_compile_time: f64,
    pub errors: Vec<String>,
}

impl ZombieMetrics {
    pub fn new() -> Self {
        Self {
            compilation_count: 0,
            network_peers: 0,
            avg_compile_time: 0.0,
            errors: Vec::new(),
        }
    }

    pub fn record_compilation(&mut self, time_ms: u64) {
        self.compilation_count += 1;
        self.avg_compile_time = (self.avg_compile_time * (self.compilation_count - 1) as f64 + time_ms as f64) / self.compilation_count as f64;
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}
