use std::str::FromStr;
use rustc_span::{Symbol, sym};

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
        Nto71Iosock = "nto710-iosock",
        Nto80 = "nto80",
        Uwp = "uwp",
        Uclibc = "uclibc",
        Sim = "sim",
        Relibc = "relibc",
        Ohos = "ohos",
        Unspecified = "",
        other_variant = "Other",
    }
    parse_error_type = "environment";
}

impl crate::json::ToJson for Env {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}