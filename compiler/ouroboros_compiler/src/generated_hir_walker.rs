
// Auto-generated HIR walker from Prime Monster data
// Integrates syn walker with real HIR objects
use std::collections::HashMap;

pub struct DataDrivenHirWalker {
    prime_transformations: HashMap<String, u64>,
    hir_objects: Vec<HirNode>,
}

#[derive(Debug, Clone)]
pub struct HirNode {
    name: String,
    usage_count: u64,
    syn_mapping: String,
}

#[derive(Debug, Clone)]
pub struct DataSlice {
    node_name: String,
    usage_count: u64,
    syn_mapping: String,
    high_usage_symbols: Vec<String>,
    total_symbols: usize,
}

impl DataDrivenHirWalker {
    pub fn new() -> Self {
        let mut transformations = HashMap::new();
        let mut hir_objects = Vec::new();
        transformations.insert("rustc_query_impl::metadata".to_string(), 4382);
        hir_objects.push(HirNode {
            name: "rustc_query_impl::metadata".to_string(),
            usage_count: 4382,
            syn_mapping: "syn_rustc_query_impl_metadata".to_string(),
        });
        transformations.insert("rustc_query_impl::fields".to_string(), 2504);
        hir_objects.push(HirNode {
            name: "rustc_query_impl::fields".to_string(),
            usage_count: 2504,
            syn_mapping: "syn_rustc_query_impl_fields".to_string(),
        });
        transformations.insert("rustc_query_impl::le".to_string(), 2504);
        hir_objects.push(HirNode {
            name: "rustc_query_impl::le".to_string(),
            usage_count: 2504,
            syn_mapping: "syn_rustc_query_impl_le".to_string(),
        });
        transformations.insert("rustc_target::into".to_string(), 1970);
        hir_objects.push(HirNode {
            name: "rustc_target::into".to_string(),
            usage_count: 1970,
            syn_mapping: "syn_rustc_target_into".to_string(),
        });
        transformations.insert("rustc_query_impl::iter".to_string(), 1259);
        hir_objects.push(HirNode {
            name: "rustc_query_impl::iter".to_string(),
            usage_count: 1259,
            syn_mapping: "syn_rustc_query_impl_iter".to_string(),
        });
        
        Self { 
            prime_transformations: transformations,
            hir_objects,
        }
    }
    
    pub fn walk_hir(&self) {
        println!("🚶 Walking HIR with syn walker integration");
        
        // Sort HIR objects by usage count (highest to lowest)
        let mut sorted_nodes = self.hir_objects.clone();
        sorted_nodes.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        
        // Calculate cutoff (top 80% of total usage)
        let total_usage: u64 = sorted_nodes.iter().map(|n| n.usage_count).sum();
        let cutoff_threshold = (total_usage as f64 * 0.8) as u64;
        
        println!("📊 Total usage: {}, Cutoff threshold: {} (80%)", total_usage, cutoff_threshold);
        
        let mut accumulated_usage = 0;
        let mut data_driven_slice = Vec::new();
        
        for hir_node in &sorted_nodes {
            accumulated_usage += hir_node.usage_count;
            
            // Apply syn walker pattern to HIR node
            if let Some(hir_data) = self.load_hir_data(&hir_node.name) {
                let slice_data = self.create_data_slice(&hir_data, &hir_node);
                data_driven_slice.push(slice_data);
                
                self.apply_syn_walker_to_hir(&hir_node);
            }
            
            // Stop when we hit the cutoff
            if accumulated_usage >= cutoff_threshold {
                println!("🎯 Cutoff reached at {} usage ({}% of total)", 
                        accumulated_usage, 
                        (accumulated_usage as f64 / total_usage as f64 * 100.0) as u32);
                break;
            }
        }
        
        // Generate the data-driven slice
        self.generate_slice_output(&data_driven_slice);
        
        println!("✅ HIR traversal complete with {} nodes in slice", data_driven_slice.len());
    }
    
    fn create_data_slice(&self, hir_data: &str, hir_node: &HirNode) -> DataSlice {
        if let Ok(hir_json) = serde_json::from_str::<serde_json::Value>(hir_data) {
            if let Some(usages) = hir_json["usages"].as_array() {
                let high_usage_symbols: Vec<String> = usages.iter()
                    .filter_map(|usage| {
                        if let (Some(symbol), Some(count)) = (usage["symbol"].as_str(), usage["usage_count"].as_u64()) {
                            if count > 1 { // Only include symbols used more than once
                                Some(symbol.to_string())
                            } else { None }
                        } else { None }
                    })
                    .take(10) // Top 10 symbols per node
                    .collect();
                
                return DataSlice {
                    node_name: hir_node.name.clone(),
                    usage_count: hir_node.usage_count,
                    syn_mapping: hir_node.syn_mapping.clone(),
                    high_usage_symbols,
                    total_symbols: usages.len(),
                };
            }
        }
        
        DataSlice {
            node_name: hir_node.name.clone(),
            usage_count: hir_node.usage_count,
            syn_mapping: hir_node.syn_mapping.clone(),
            high_usage_symbols: vec![],
            total_symbols: 0,
        }
    }
    
    fn generate_slice_output(&self, slice: &[DataSlice]) {
        println!("🔪 Generating data-driven slice:");
        for (i, data) in slice.iter().enumerate() {
            println!("  {}. {} (usage: {}, symbols: {})", 
                    i + 1, data.node_name, data.usage_count, data.total_symbols);
            for symbol in data.high_usage_symbols.iter().take(3) {
                println!("     • {}", symbol);
            }
        }
    }
    
    fn apply_syn_walker_to_hir(&self, hir_node: &HirNode) {
        // This is where we apply the syn walker logic to actual HIR data
        println!("  🔄 Syn walker → HIR: {} (usage: {}, syn: {})", 
                hir_node.name, hir_node.usage_count, hir_node.syn_mapping);
        
        // Load actual HIR object from mycelial data
        if let Some(hir_data) = self.load_hir_data(&hir_node.name) {
            self.traverse_hir_with_syn_patterns(&hir_data, &hir_node.syn_mapping);
        }
    }
    
    fn load_hir_data(&self, node_name: &str) -> Option<String> {
        // Load real HIR data from mycelial dataset
        let mycelial_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/test_usage_data";
        
        // Map our Prime Monster names to actual files
        let file_mapping = match node_name {
            "rustc_query_impl::metadata" => "rustc_metadata_literals.json",
            "rustc_query_impl::fields" => "rustc_middle_complexity.json", 
            "rustc_query_impl::le" => "rustc_session_complexity.json",
            "rustc_target::into" => "rustc_target_constants.json",
            "rustc_query_impl::iter" => "rustc_query_impl_literals.json",
            _ => return None,
        };
        
        let hir_file = format!("{}/{}", mycelial_path, file_mapping);
        if let Ok(content) = std::fs::read_to_string(&hir_file) {
            println!("    📁 Loaded HIR data from: {}", file_mapping);
            Some(content)
        } else {
            println!("    ⚠️  Failed to load: {}", file_mapping);
            None
        }
    }
    
    fn traverse_hir_with_syn_patterns(&self, hir_data: &str, syn_mapping: &str) {
        // Apply syn walker traversal patterns to HIR data
        println!("    📊 Traversing HIR data with syn pattern: {}", syn_mapping);
        
        // Parse HIR JSON and apply syn walker logic
        if let Ok(hir_json) = serde_json::from_str::<serde_json::Value>(hir_data) {
            if let Some(usages) = hir_json["usages"].as_array() {
                println!("    🎯 Found {} HIR usages to traverse", usages.len());
                
                let mut total_usage_count = 0;
                for usage in usages.iter().take(5) {
                    if let Some(symbol) = usage["symbol"].as_str() {
                        if let Some(count) = usage["usage_count"].as_u64() {
                            total_usage_count += count;
                            println!("      • HIR symbol: {} (count: {}, syn: {})", 
                                   symbol, count, syn_mapping);
                        }
                    }
                }
                
                println!("    ✅ Syn walker processed {} total usages", total_usage_count);
            }
        }
    }
}
