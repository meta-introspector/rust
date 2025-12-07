use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};

crate::target_spec_enum! {
    pub enum TlsModel {
        GeneralDynamic = "global-dynamic",
        LocalDynamic = "local-dynamic",
        InitialExec = "initial-exec",
        LocalExec = "local-exec",
        Emulated = "emulated",
    }

    parse_error_type = "TLS model";
}

impl crate::json::ToJson for TlsModel {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}