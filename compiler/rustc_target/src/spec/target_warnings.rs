use rustc_macros::{Decodable, Encodable, HashStable_Generic};

/// Warnings encountered when parsing the target `json`.
///
/// Includes fields that weren't recognized and fields that don't have the expected type.
#[derive(Debug, PartialEq)]
pub struct TargetWarnings {
    unused_fields: Vec<String>,
}

impl TargetWarnings {
    pub fn empty() -> Self {
        Self { unused_fields: Vec::new() }
    }

    pub fn warning_messages(&self) -> Vec<String> {
        let mut warnings = vec![];
        if !self.unused_fields.is_empty() {
            warnings.push(format!(
                "target json file contains unused fields: {}",
                self.unused_fields.join(", ")
            ));
        }
        warnings
    }
}