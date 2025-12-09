use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use std::str::FromStr;

crate::target_spec_enum! {
    pub enum FramePointer {
        /// Forces the machine code generator to always preserve the frame pointers.
        Always = "always",
        /// Forces the machine code generator to preserve the frame pointers except for the leaf
        /// functions (i.e. those that don't call other functions).
        NonLeaf = "non-leaf",
        /// Allows the machine code generator to omit the frame pointers.
        ///
        /// This option does not guarantee that the frame pointers will be omitted.
        MayOmit = "may-omit",
    }

    parse_error_type = "frame pointer";
}

impl FramePointer {
    /// It is intended that the "force frame pointer" transition is "one way"
    /// so this convenience assures such if used
    #[inline]
    pub fn ratchet(&mut self, rhs: FramePointer) -> FramePointer {
        *self = match (*self, rhs) {
            (FramePointer::Always, _) | (_, FramePointer::Always) => FramePointer::Always,
            (FramePointer::NonLeaf, _) | (_, FramePointer::NonLeaf) => FramePointer::NonLeaf,
            _ => FramePointer::MayOmit,
        };
        *self
    }
}

impl crate::json::ToJson for FramePointer {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}