use crate::usage_types::*;

pub struct UsageClassifier;

impl UsageClassifier {
    pub fn classify_usage(usage: &str, usage_type: &str, _used_def_id: &str) -> UsageClassification {
        let mut classification = UsageClassification::default();
        
        // String conversion patterns
        if usage.contains("to_string") || usage.contains("Display") || usage.contains("fmt") {
            classification.string_conversion = 1;
        }
        // Pattern matching
        else if usage.contains("match") || usage.contains("if let") || usage.contains("pattern") {
            classification.pattern_matching = 1;
        }
        // Construction patterns
        else if usage.contains("::") && !usage.contains("(") {
            classification.construction = 1;
        }
        // Comparison
        else if usage.contains("==") || usage.contains("!=") || usage.contains("cmp") {
            classification.comparison = 1;
        }
        // Debug formatting
        else if usage.contains("Debug") || usage.contains("{:?}") || usage.contains("dbg!") {
            classification.debug_format = 1;
        }
        // Serialization
        else if usage.contains("serde") || usage.contains("serialize") || usage.contains("json") {
            classification.serialization = 1;
        }
        // General usage
        else {
            classification.general = 1;
        }
        
        classification
    }
    
    pub fn extract_enum_variant(def_id: &str) -> Option<(String, String)> {
        if let Some(last_segment) = def_id.split("::").last() {
            if let Some(variant_pos) = last_segment.find("::") {
                let enum_name = last_segment[..variant_pos].to_string();
                let variant_name = last_segment[variant_pos + 2..].to_string();
                return Some((enum_name, variant_name));
            }
        }
        None
    }
}
