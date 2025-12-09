use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use std::str::FromStr;

crate::target_spec_enum! {
    pub enum BinaryFormat {
        Coff = "coff",
        Elf = "elf",
        MachO = "mach-o",
        Wasm = "wasm",
        Xcoff = "xcoff",
    }

    parse_error_type = "binary format";
}

impl BinaryFormat {
    /// Returns [`object::BinaryFormat`] for given `BinaryFormat`
    pub fn to_object(&self) -> object::BinaryFormat {
        match self {
            Self::Coff => object::BinaryFormat::Coff,
            Self::Elf => object::BinaryFormat::Elf,
            Self::MachO => object::BinaryFormat::MachO,
            Self::Wasm => object::BinaryFormat::Wasm,
            Self::Xcoff => object::BinaryFormat::Xcoff,
        }
    }
}

impl crate::json::ToJson for BinaryFormat {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}