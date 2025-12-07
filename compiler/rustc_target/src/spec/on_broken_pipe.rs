use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Encodable, Decodable, HashStable_Generic)]
pub enum OnBrokenPipe {
    Default,
    Kill,
    Error,
    Inherit,
}