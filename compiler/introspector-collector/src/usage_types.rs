use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CleanGraphData {
    pub crate_name: String,
    pub call_graph: HashMap<String, Vec<String>>,
    pub symbol_table: HashMap<String, String>,
    pub def_paths: HashMap<String, String>,
    pub total_nodes: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UsageEntry {
    pub symbol: String,
    pub kind: String,
    pub usage_count: usize,
    pub usage_type: String,
    pub node_type: String,
    pub user_def_id: String,
    pub used_def_id: String,
    pub user_crate: Option<String>,
    pub used_crate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UsageClassification {
    pub string_conversion: usize,
    pub pattern_matching: usize,
    pub construction: usize,
    pub comparison: usize,
    pub debug_format: usize,
    pub serialization: usize,
    pub general: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EnumVariantUsage {
    pub enum_name: String,
    pub variant_name: String,
    pub usage_classes: UsageClassification,
    pub top_converters: HashMap<String, Vec<(String, usize)>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemComplexity {
    pub name: String,
    pub item_type: String,
    pub complexity: String,
    pub field_count: Option<usize>,
    pub variant_count: Option<usize>,
    pub param_count: Option<usize>,
    pub fields: Option<Vec<FieldInfo>>,
    pub variants: Option<Vec<VariantInfo>>,
    pub parameters: Option<Vec<ParamInfo>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub is_public: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VariantInfo {
    pub name: String,
    pub has_fields: bool,
    pub field_count: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ParamInfo {
    pub name: String,
    pub param_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EnumInfo {
    pub name: String,
    pub variants: Vec<EnumVariantUsage>,
    pub total_usage_classes: UsageClassification,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleData {
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub module: String,
    pub usages: Vec<UsageEntry>,
}
