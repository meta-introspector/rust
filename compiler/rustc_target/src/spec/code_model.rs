use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_span::Symbol; // Assuming Symbol is needed for `desc_symbol` if it existed.

crate::target_spec_enum! {
    pub enum CodeModel {
        Tiny = "tiny",
        Small = "small",
        Kernel = "kernel",
        Medium = "medium",
        Large = "large",
    }

    parse_error_type = "code model";
}

impl crate::json::ToJson for CodeModel {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}