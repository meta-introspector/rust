use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_error_messages::{DiagArgValue, IntoDiagArg, into_diag_arg_using_display};
use std::borrow::Cow;

crate::target_spec_enum! {
    #[derive(Default)]
    pub enum SplitDebuginfo {
        /// Split debug-information is disabled, meaning that on supported platforms
        /// you can find all debug information in the executable itself. This is
        /// only supported for ELF effectively.
        ///
        /// * Windows - not supported
        /// * macOS - don't run `dsymutil`
        /// * ELF - `.debug_*` sections
        #[default]
        Off = "off",

        /// Split debug-information can be found in a "packed" location separate
        /// from the final artifact. This is supported on all platforms.
        ///
        /// * Windows - `*.pdb`
        /// * macOS - `*.dSYM` (run `dsymutil`)
        /// * ELF - `*.dwp` (run `thorin`)
        Packed = "packed",

        /// Split debug-information can be found in individual object files on the
        /// filesystem. The main executable may point to the object files.
        ///
        /// * Windows - not supported
        /// * macOS - supported, scattered object files
        /// * ELF - supported, scattered `*.dwo` or `*.o` files (see `SplitDwarfKind`)
        Unpacked = "unpacked",
    }

    parse_error_type = "split debuginfo";
}

impl crate::json::ToJson for SplitDebuginfo {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}

impl IntoDiagArg for SplitDebuginfo {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.desc().to_string()))
    }
}
