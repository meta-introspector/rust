use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcxFrequency {
    pub frequency: String,
    pub function_filter: Vec<String>,
    pub def_kind_filter: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcxSubset {
    pub frequency: String,
    pub crate_name: String,
    pub filtered_defs: Vec<String>,
    pub processing_node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencySubscription {
    pub node_id: String,
    pub subscribed_frequencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationData {
    pub peer_id: String,
    pub crate_name: String,
    pub ast_nodes: u32,
    pub compilation_time_ms: u64,
}
