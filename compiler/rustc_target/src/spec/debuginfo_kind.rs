use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};

crate::target_spec_enum! {
    /// Which kind of debuginfo does the target use?
    ///
    /// Useful in determining whether a target supports Split DWARF (a target with
    /// `DebuginfoKind::Dwarf` and supporting `SplitDebuginfo::Unpacked` for example).
    #[derive(Default)]
    pub enum DebuginfoKind {
        /// DWARF debuginfo (such as that used on `x86_64_unknown_linux_gnu`).
        #[default]
        Dwarf = "dwarf",
        /// DWARF debuginfo in dSYM files (such as on Apple platforms).
        DwarfDsym = "dwarf-dsym",
        /// Program database files (such as on Windows).
        Pdb = "pdb",
    }

    parse_error_type = "debuginfo kind";
}

impl crate::json::ToJson for DebuginfoKind {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}