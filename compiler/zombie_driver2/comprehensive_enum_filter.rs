// Comprehensive Enum Modular Form Filter
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EnumModularFilter {
    enum_signatures: HashMap<String, EnumSignature>,
}

#[derive(Debug, Clone)]
pub struct EnumSignature {
    modular_key: String,
    level: u32,
    weight: u32,
    variants: Vec<String>,
    hotness: f64,
}

impl EnumModularFilter {
    pub fn new() -> Self {
        let mut signatures = HashMap::new();
        signatures.insert("Option".to_string(), EnumSignature {
            modular_key: "5.2.11.j".to_string(),
            level: 5,
            weight: 2,
            variants: vec!["Some", "None"],
            hotness: 11.0,
        });
        signatures.insert("Result".to_string(), EnumSignature {
            modular_key: "12.4.12.q".to_string(),
            level: 12,
            weight: 4,
            variants: vec!["Ok", "Err"],
            hotness: 10.5,
        });
        signatures.insert("Ordering".to_string(), EnumSignature {
            modular_key: "15.2.12.w".to_string(),
            level: 15,
            weight: 2,
            variants: vec!["Less", "Equal", "Greater"],
            hotness: 8.5,
        });
        signatures.insert("ErrorKind".to_string(), EnumSignature {
            modular_key: "28.2.11.f".to_string(),
            level: 28,
            weight: 2,
            variants: vec!["NotFound", "PermissionDenied", "ConnectionRefused"],
            hotness: 8.0,
        });
        signatures.insert("IpAddr".to_string(), EnumSignature {
            modular_key: "14.4.12.w".to_string(),
            level: 14,
            weight: 4,
            variants: vec!["V4", "V6"],
            hotness: 6.0,
        });
        signatures.insert("SocketAddr".to_string(), EnumSignature {
            modular_key: "3.6.11.n".to_string(),
            level: 3,
            weight: 6,
            variants: vec!["V4", "V6"],
            hotness: 5.5,
        });
        signatures.insert("VarError".to_string(), EnumSignature {
            modular_key: "12.2.11.t".to_string(),
            level: 12,
            weight: 2,
            variants: vec!["NotPresent", "NotUnicode"],
            hotness: 4.0,
        });
        signatures.insert("SeekFrom".to_string(), EnumSignature {
            modular_key: "27.6.11.x".to_string(),
            level: 27,
            weight: 6,
            variants: vec!["Start", "End", "Current"],
            hotness: 4.5,
        });
        signatures.insert("Shutdown".to_string(), EnumSignature {
            modular_key: "18.4.12.k".to_string(),
            level: 18,
            weight: 4,
            variants: vec!["Read", "Write", "Both"],
            hotness: 4.5,
        });
        signatures.insert("FpCategory".to_string(), EnumSignature {
            modular_key: "23.4.11.h".to_string(),
            level: 23,
            weight: 4,
            variants: vec!["Nan", "Infinite", "Zero", "Subnormal", "Normal"],
            hotness: 5.5,
        });
        Self { enum_signatures: signatures }
    }

    pub fn find_enum_by_modular_key(&self, key: &str) -> Option<String> {
        self.enum_signatures.iter()
            .find(|(_, sig)| sig.modular_key == key)
            .map(|(name, _)| name.clone())
    }

    pub fn find_enums_by_weight(&self, weight: u32) -> Vec<String> {
        self.enum_signatures.iter()
            .filter(|(_, sig)| sig.weight == weight)
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub fn find_hottest_enums(&self, limit: usize) -> Vec<(String, f64)> {
        let mut enums: Vec<_> = self.enum_signatures.iter()
            .map(|(name, sig)| (name.clone(), sig.hotness))
            .collect();
        enums.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        enums.into_iter().take(limit).collect()
    }
}
