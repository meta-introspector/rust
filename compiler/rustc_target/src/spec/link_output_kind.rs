use std::str::FromStr;
use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};

crate::target_spec_enum! {
    /// Everything is flattened to a single enum to make the json encoding/decoding less annoying.
    pub enum LinkOutputKind {
        /// Dynamically linked non position-independent executable.
        DynamicNoPicExe = "dynamic-nopic-exe",
        /// Dynamically linked position-independent executable.
        DynamicPicExe = "dynamic-pic-exe",
        /// Statically linked non position-independent executable.
        StaticNoPicExe = "static-nopic-exe",
        /// Statically linked position-independent executable.
        StaticPicExe = "static-pic-exe",
        /// Regular dynamic library ("dynamically linked").
        DynamicDylib = "dynamic-dylib",
        /// Dynamic library with bundled libc ("statically linked").
        StaticDylib = "static-dylib",
        /// WASI module with a lifetime past the _initialize entry point
        WasiReactorExe = "wasi-reactor-exe",
    }

    parse_error_type = "CRT object kind";
}

impl LinkOutputKind {
    pub fn can_link_dylib(self) -> bool {
        match self {
            LinkOutputKind::StaticNoPicExe | LinkOutputKind::StaticPicExe => false,
            LinkOutputKind::DynamicNoPicExe
            | LinkOutputKind::DynamicPicExe
            | LinkOutputKind::DynamicDylib
            | LinkOutputKind::StaticDylib
            | LinkOutputKind::WasiReactorExe => true,
        }
    }
}