use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_span::Symbol; // Assuming Symbol is needed if desc_symbol was used

crate::target_spec_enum! {
    /// The Rustc-specific variant of the ABI used for this target.
    pub enum RustcAbi {
        /// On x86-32 only: make use of SSE and SSE2 for ABI purposes.
        X86Sse2 = "x86-sse2",
        /// On x86-32/64 only: do not use any FPU or SIMD registers for the ABI.
        X86Softfloat = "x86-softfloat",
    }

    parse_error_type = "rustc abi";
}

impl crate::json::ToJson for RustcAbi {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}
