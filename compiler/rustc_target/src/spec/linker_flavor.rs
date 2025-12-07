use core::result::Result;
use std::borrow::Cow;
use std::str::FromStr;

use rustc_span::Symbol;

use crate::json::{Json, ToJson};
use crate::target_spec_enum;

mod linker_flavor;
pub use linker_flavor!;