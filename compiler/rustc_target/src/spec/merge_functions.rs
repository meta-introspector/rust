use std::str::FromStr;
use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};

crate::target_spec_enum! {
    pub enum MergeFunctions {
        Disabled = "disabled",
        Trampolines = "trampolines",
        Aliases = "aliases",
    }

    parse_error_type = "value for merge-functions";
}

impl crate::json::ToJson for MergeFunctions {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}