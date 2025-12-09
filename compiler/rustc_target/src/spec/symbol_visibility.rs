use std::str::FromStr;
use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};

crate::target_spec_enum! {
    pub enum SymbolVisibility {
        Hidden = "hidden",
        Protected = "protected",
        Interposable = "interposable",
    }

    parse_error_type = "symbol visibility";
}

impl crate::json::ToJson for SymbolVisibility {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}