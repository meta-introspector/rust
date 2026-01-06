use crate::usage_types::*;
use crate::usage_classifier::UsageClassifier;
use crate::generic_tracker::GenericTracker;
use crate::ast_extractor::AstExtractor;
use std::collections::HashMap;
use std::sync::OnceLock;

static USAGE_CACHE: OnceLock<HashMap<String, Vec<UsageEntry>>> = OnceLock::new();

pub struct UsageCollector {
    pub module_data: HashMap<String, Vec<UsageEntry>>,
    pub enum_data: HashMap<String, EnumInfo>,
    pub item_complexity: HashMap<String, ItemComplexity>,
}

impl UsageCollector {
    pub fn new() -> Self {
        Self {
            module_data: HashMap::new(),
            enum_data: HashMap::new(),
            item_complexity: HashMap::new(),
        }
    }
    
    pub fn get_cached_usages(&self) -> &HashMap<String, Vec<UsageEntry>> {
        USAGE_CACHE.get_or_init(|| {
            self.load_previous_data().unwrap_or_default()
        })
    }
    
    pub fn load_previous_data(&self) -> Result<HashMap<String, Vec<UsageEntry>>, Box<dyn std::error::Error>> {
        if let Ok(content) = std::fs::read_to_string("usage_eigenmatrix.json") {
            if let Ok(_data) = serde_json::from_str::<serde_json::Value>(&content) {
                return Ok(HashMap::new()); // Simplified for now
            }
        }
        Ok(HashMap::new())
    }
    
    pub fn add_usage(&mut self, module: &str, symbol: String, kind: String, usage_type: String, node_type: String, user_def_id: String, used_def_id: String, user_crate: Option<String>, used_crate: Option<String>) {
        let classification = UsageClassifier::classify_usage(&kind, &usage_type, &used_def_id);
        
        let entry = UsageEntry {
            symbol,
            kind,
            usage_count: 1,
            usage_type,
            node_type,
            user_def_id,
            used_def_id,
            user_crate,
            used_crate,
        };
        
        self.module_data.entry(module.to_string()).or_insert_with(Vec::new).push(entry);
    }
    
    /// Track generic parameters - HIGH IMPACT implementation
    pub fn track_generics<'tcx>(&mut self, generics: &rustc_hir::Generics<'tcx>, item_name: &str, crate_name: &str, item_type: &str) {
        let mut add_usage_closure = |module: &str, symbol: String, kind: String, usage_type: String, node_type: String, user_def_id: String, used_def_id: String, user_crate: Option<String>, used_crate: Option<String>| {
            self.add_usage(module, symbol, kind, usage_type, node_type, user_def_id, used_def_id, user_crate, used_crate);
        };
        
        GenericTracker::track_generics(generics, item_name, crate_name, item_type, &mut add_usage_closure);
    }
}
