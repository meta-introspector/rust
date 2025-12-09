use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_error_messages::{DiagArgValue, IntoDiagArg, into_diag_arg_using_display};
use std::borrow::Cow;
use std::str::FromStr;

crate::target_spec_enum! {
    /// Controls use of stack canaries.
    pub enum StackProtector {
        /// Disable stack canary generation.
        None = "none",

        /// On LLVM, mark all generated LLVM functions with the `ssp` attribute (see
        /// llvm/docs/LangRef.rst). This triggers stack canary generation in
        /// functions which contain an array of a byte-sized type with more than
        /// eight elements.
        Basic = "basic",

        /// On LLVM, mark all generated LLVM functions with the `sspstrong`
        /// attribute (see llvm/docs/LangRef.rst). This triggers stack canary
        /// generation in functions which either contain an array, or which take
        /// the address of a local variable.
        Strong = "strong",

        /// Generate stack canaries in all functions.
        All = "all",
    }

    parse_error_type = "stack protector";
}

impl IntoDiagArg for StackProtector {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.desc().to_string()))
    }
}