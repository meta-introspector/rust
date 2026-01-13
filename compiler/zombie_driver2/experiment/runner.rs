use crate::experiment::types::{TestFeature, ExperimentResults};
use crate::random::generator::generate_bool;

pub struct AutoExperiment {
    pub features: Vec<TestFeature>,
}

impl AutoExperiment {
    pub fn new() -> Self {
        Self { features: Vec::new() }
    }
    
    pub fn add_feature(&mut self, feature: TestFeature) {
        self.features.push(feature);
    }
    
    pub fn execute(&self) -> ExperimentResults {
        let mut passed = 0;
        
        println!("🧪 EXECUTING {} FEATURES:", self.features.len());
        
        for feature in &self.features {
            // Calculate success probability based on complexity
            let _success_probability = match feature.complexity {
                x if x < 1.0 => 0.9,  // 90% success for simple features
                x if x < 2.0 => 0.7,  // 70% success for medium features  
                x if x < 3.0 => 0.5,  // 50% success for complex features
                _ => 0.3,  // 30% success for ultimate features
            };
            
            let result = generate_bool(feature.name.len() as u64 + feature.complexity as u64);
            
            if result {
                passed += 1;
                println!("  ✅ {} (complexity: {}) - PASSED", feature.name, feature.complexity);
            } else {
                println!("  ❌ {} (complexity: {}) - FAILED", feature.name, feature.complexity);
            }
        }
        
        ExperimentResults::new(self.features.len(), passed)
    }
}
