use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};

crate::target_spec_enum! {
    pub enum Env {
        Gnu = "gnu",
        MacAbi = "macabi",
        Mlibc = "mlibc",
        Msvc = "msvc",
        Musl = "musl",
        Newlib = "newlib",
        Nto70 = "nto70",
        Nto71 = "nto71",
        Nto71IoSock = "nto71_iosock",
        Nto80 = "nto80",
        Ohos = "ohos",
        Relibc = "relibc",
        Sgx = "sgx",
        Sim = "sim",
        P1 = "p1",
        P2 = "p2",
        P3 = "p3",
        Uclibc = "uclibc",
        V5 = "v5",
        Unspecified = "",
    }
}

impl crate::json::ToJson for Env {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}