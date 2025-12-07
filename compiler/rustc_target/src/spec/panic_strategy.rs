use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_span::{Symbol, sym};
use crate::json::ToJson;
use std::borrow::Cow;
use rustc_error_messages::{DiagArgValue, IntoDiagArg, into_diag_arg_using_display};

crate::target_spec_enum! {
    #[derive(Encodable, Decodable, HashStable_Generic)]
    pub enum PanicStrategy {
        Unwind = "unwind",
        Abort = "abort",
        ImmediateAbort = "immediate-abort",
    }

    parse_error_type = "panic strategy";
}

impl PanicStrategy {
    pub const fn desc_symbol(&self) -> Symbol {
        match *self {
            PanicStrategy::Unwind => sym::unwind,
            PanicStrategy::Abort => sym::abort,
            PanicStrategy::ImmediateAbort => sym::immediate_abort,
        }
    }

    pub fn unwinds(self) -> bool {
        matches!(self, PanicStrategy::Unwind)
    }
}

impl crate::json::ToJson for PanicStrategy {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}

impl IntoDiagArg for PanicStrategy {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.desc().to_string()))
    }
}