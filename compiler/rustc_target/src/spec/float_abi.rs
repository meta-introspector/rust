use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};

crate::target_spec_enum! {
    /// The float ABI setting to be configured in the LLVM target machine.
    pub enum FloatAbi {
        Soft = "soft",
        Hard = "hard",
    }

    parse_error_type = "float abi";
}

impl crate::json::ToJson for FloatAbi {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}