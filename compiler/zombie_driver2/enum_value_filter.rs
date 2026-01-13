// Auto-generated enum value filter
use std::collections::HashMap;

pub struct EnumValueFilter {
    modular_keys: HashMap<String, u64>,
}

impl EnumValueFilter {
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        Self { modular_keys: keys }
    }

    pub fn find_enum_usage(&self, modular_key: &str) -> Option<u64> {
        self.modular_keys.get(modular_key).copied()
    }
}
