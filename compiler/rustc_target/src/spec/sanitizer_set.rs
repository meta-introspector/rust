use crate::json::{Json, ToJson};
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use std::borrow::Cow;
use std::str::FromStr;
use schemars::JsonSchema;
use std::fmt;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Encodable, Decodable, HashStable_Generic)]
pub struct SanitizerSet(u16);
bitflags::bitflags! {
    impl SanitizerSet: u16 {
        const ADDRESS = 1 << 0;
        const LEAK    = 1 << 1;
        const MEMORY  = 1 << 2;
        const THREAD  = 1 << 3;
        const HWADDRESS = 1 << 4;
        const CFI     = 1 << 5;
        const MEMTAG  = 1 << 6;
        const SHADOWCALLSTACK = 1 << 7;
        const KCFI    = 1 << 8;
        const KERNELADDRESS = 1 << 9;
        const SAFESTACK = 1 << 10;
        const DATAFLOW = 1 << 11;
        const REALTIME = 1 << 12;
    }
}
rustc_data_structures::external_bitflags_debug! { SanitizerSet }

impl SanitizerSet {
    // Taken from LLVM's sanitizer compatibility logic:
    // https://github.com/llvm/llvm-project/blob/release/18.x/clang/lib/Driver/SanitizerArgs.cpp#L512
    const MUTUALLY_EXCLUSIVE: &'static [(SanitizerSet, SanitizerSet)] = &[
        (SanitizerSet::ADDRESS, SanitizerSet::MEMORY),
        (SanitizerSet::ADDRESS, SanitizerSet::THREAD),
        (SanitizerSet::ADDRESS, SanitizerSet::HWADDRESS),
        (SanitizerSet::ADDRESS, SanitizerSet::MEMTAG),
        (SanitizerSet::ADDRESS, SanitizerSet::KERNELADDRESS),
        (SanitizerSet::ADDRESS, SanitizerSet::SAFESTACK),
        (SanitizerSet::LEAK, SanitizerSet::MEMORY),
        (SanitizerSet::LEAK, SanitizerSet::THREAD),
        (SanitizerSet::LEAK, SanitizerSet::KERNELADDRESS),
        (SanitizerSet::LEAK, SanitizerSet::SAFESTACK),
        (SanitizerSet::MEMORY, SanitizerSet::THREAD),
        (SanitizerSet::MEMORY, SanitizerSet::HWADDRESS),
        (SanitizerSet::MEMORY, SanitizerSet::KERNELADDRESS),
        (SanitizerSet::MEMORY, SanitizerSet::SAFESTACK),
        (SanitizerSet::THREAD, SanitizerSet::HWADDRESS),
        (SanitizerSet::THREAD, SanitizerSet::KERNELADDRESS),
        (SanitizerSet::THREAD, SanitizerSet::SAFESTACK),
        (SanitizerSet::HWADDRESS, SanitizerSet::MEMTAG),
        (SanitizerSet::HWADDRESS, SanitizerSet::KERNELADDRESS),
        (SanitizerSet::HWADDRESS, SanitizerSet::SAFESTACK),
        (SanitizerSet::CFI, SanitizerSet::KCFI),
        (SanitizerSet::MEMTAG, SanitizerSet::KERNELADDRESS),
        (SanitizerSet::KERNELADDRESS, SanitizerSet::SAFESTACK),
    ];

    /// Return sanitizer's name
    ///
    /// Returns none if the flags is a set of sanitizers numbering not exactly one.
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            SanitizerSet::ADDRESS => "address",
            SanitizerSet::CFI => "cfi",
            SanitizerSet::DATAFLOW => "dataflow",
            SanitizerSet::KCFI => "kcfi",
            SanitizerSet::KERNELADDRESS => "kernel-address",
            SanitizerSet::LEAK => "leak",
            SanitizerSet::MEMORY => "memory",
            SanitizerSet::MEMTAG => "memtag",
            SanitizerSet::SAFESTACK => "safestack",
            SanitizerSet::SHADOWCALLSTACK => "shadow-call-stack",
            SanitizerSet::THREAD => "thread",
            "hwaddress" => "hwaddress",
            SanitizerSet::REALTIME => "realtime",
            _ => return None,
        })
    }

    pub fn mutually_exclusive(self) -> Option<(SanitizerSet, SanitizerSet)> {
        Self::MUTUALLY_EXCLUSIVE
            .into_iter()
            .find(|&(a, b)| self.contains(*a) && self.contains(*b))
            .copied()
    }
}

/// Formats a sanitizer set as a comma separated list of sanitizers' names.
impl fmt::Display for SanitizerSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for s in *self {
            let name = s.as_str().unwrap_or_else(|| panic!("unrecognized sanitizer {s:?}"));
            if !first {
                f.write_str(", ")?;
            }
            f.write_str(name)?;
            first = false;
        }
        Ok(())
    }
}

impl FromStr for SanitizerSet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "address" => SanitizerSet::ADDRESS,
            "cfi" => SanitizerSet::CFI,
            "dataflow" => SanitizerSet::DATAFLOW,
            "kcfi" => SanitizerSet::KCFI,
            "kernel-address" => SanitizerSet::KERNELADDRESS,
            "leak" => SanitizerSet::LEAK,
            "memory" => SanitizerSet::MEMORY,
            "memtag" => SanitizerSet::MEMTAG,
            "safestack" => SanitizerSet::SAFESTACK,
            "shadow-call-stack" => SanitizerSet::SHADOWCALLSTACK,
            "thread" => SanitizerSet::THREAD,
            "hwaddress" => SanitizerSet::HWADDRESS,
            "realtime" => SanitizerSet::REALTIME,
            s => return Err(format!("unknown sanitizer {s}")),
        })
    }
}

crate::json::serde_deserialize_from_str!(SanitizerSet);
impl schemars::JsonSchema for SanitizerSet {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "SanitizerSet".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let all = Self::all().iter().map(|sanitizer| sanitizer.as_str()).collect::<Vec<_>>();
        schemars::json_schema! ({
            "type": "string",
            "enum": all,
        })
        .into()
    }
}

impl ToJson for SanitizerSet {
    fn to_json(&self) -> Json {
        self.into_iter()
            .map(|v| Some(v.as_str()?.to_json()))
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default()
            .to_json()
    }
}
```