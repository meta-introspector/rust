use std::fs;
use std::path::Path;
use serde_json::Value;

pub struct AnalysisDataStore {
    base_path: String,
}

impl AnalysisDataStore {
    pub fn new() -> Self {
        let base_path = std::env::var("ZOMBIE_DATA_DIR")
            .unwrap_or_else(|_| "./zombie_analysis_data".to_string());
        
        // Ensure directory exists
        fs::create_dir_all(&base_path).expect("Failed to create data directory");
        
        Self { base_path }
    }
    
    pub fn store_analysis(&self, file_path: &str, analysis_data: &Value) -> Result<String, Box<dyn std::error::Error>> {
        let file_hash = self.hash_file_path(file_path);
        let output_path = format!("{}/{}.json", self.base_path, file_hash);
        
        let metadata = serde_json::json!({
            "source_file": file_path,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "analysis": analysis_data
        });
        
        fs::write(&output_path, serde_json::to_string_pretty(&metadata)?)?;
        Ok(output_path)
    }
    
    pub fn get_all_analyses(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let mut analyses = Vec::new();
        
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                let content = fs::read_to_string(entry.path())?;
                let data: Value = serde_json::from_str(&content)?;
                analyses.push(data);
            }
        }
        
        Ok(analyses)
    }
    
    pub fn count_analyses(&self) -> usize {
        fs::read_dir(&self.base_path)
            .map(|entries| entries.filter_map(|e| e.ok()).count())
            .unwrap_or(0)
    }
    
    fn hash_file_path(&self, file_path: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        file_path.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}
