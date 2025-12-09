use std::str::FromStr;
use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};

crate::target_spec_enum! {
    pub enum RelroLevel {
        Full = "full",
        Partial = "partial",
        Off = "off",
        None = "none",
    }

    parse_error_type = "relro level";
}

impl crate::json::ToJson for RelroLevel {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}