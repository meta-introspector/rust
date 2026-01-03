// No imports needed - using parent's HashMap

#[derive(Debug, Clone)]
pub struct ConstantInfo {
    pub name: String,
    pub value: String,
    pub visibility: String, // "pub" or "private"
    pub const_type: String,
    pub crate_name: String,
}

pub struct LeafAnalyzer {
    pub constants: Vec<ConstantInfo>,
    pub numeric_histogram: HashMap<String, u32>,
}

impl LeafAnalyzer {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            numeric_histogram: HashMap::new(),
        }
    }
    
    pub fn add_constant(&mut self, name: &str, value: &str, visibility: &str, const_type: &str, crate_name: &str) {
        let constant = ConstantInfo {
            name: name.to_string(),
            value: value.to_string(),
            visibility: visibility.to_string(),
            const_type: const_type.to_string(),
            crate_name: crate_name.to_string(),
        };
        
        // Add to histogram if numeric
        if let Ok(num) = value.parse::<i64>() {
            *self.numeric_histogram.entry(num.to_string()).or_insert(0) += 1;
        }
        
        self.constants.push(constant);
    }
    
    pub fn get_leaf_crates(&self) -> Vec<&str> {
        // These are the foundational crates with no dependencies
        vec!["compiler_builtins", "core", "rustc_std_workspace_core"]
    }
}
