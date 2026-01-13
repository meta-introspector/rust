#[derive(Debug, Clone)]
pub struct TestFeature {
    pub name: String,
    pub complexity: f64,
    pub feature_type: String,
}

impl TestFeature {
    pub fn new(name: String, complexity: f64, feature_type: String) -> Self {
        Self { name, complexity, feature_type }
    }
}

#[derive(Debug)]
pub struct ExperimentResults {
    pub total_features: usize,
    pub passed: usize,
    pub failed: usize,
    pub success_rate: f64,
}

impl ExperimentResults {
    pub fn new(total: usize, passed: usize) -> Self {
        let failed = total - passed;
        let success_rate = if total > 0 { passed as f64 / total as f64 } else { 0.0 };
        
        Self {
            total_features: total,
            passed,
            failed,
            success_rate,
        }
    }
}
