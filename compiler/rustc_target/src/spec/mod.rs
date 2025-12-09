//! [Flexible target specification.](https://github.com/rust-lang/rfcs/pull/131)
//!
//! Rust targets a wide variety of usecases, and in the interest of flexibility,
//! allows new target tuples to be defined in configuration files. Most users
//! will not need to care about these, but this is invaluable when porting Rust
//! to a new platform, and allows for an unprecedented level of control over how
//! the compiler works.
//!
//! # Using targets and target.json
//!
//! Invoking "rustc --target=${TUPLE}" will result in rustc initiating the [`Target::search`] by
//! - checking if "$TUPLE" is a complete path to a json (ending with ".json") and loading if so
//! - checking builtin targets for "${TUPLE}"
//! - checking directories in "${RUST_TARGET_PATH}" for "${TUPLE}.json"
//! - checking for "${RUSTC_SYSROOT}/lib/rustlib/${TUPLE}/target.json"
//!
//! Code will then be compiled using the first discovered target spec.
//!
//! # Defining a new target
//!
//! Targets are defined using a struct which additionally has serialization to and from [JSON].
//! The `Target` struct in this module loosely corresponds with the format the JSON takes.
//! We usually try to make the fields equivalent but we have given up on a 1:1 correspondence
//! between the JSON and the actual structure itself.
//!
//! Some fields are required in every target spec, and they should be embedded in Target directly.
//! Optional keys are in TargetOptions, but Target derefs to it, for no practical difference.
//! Most notable is the "data-layout" field which specifies Rust's notion of sizes and alignments
//! for several key types, such as f64, pointers, and so on.
//!
//! At one point we felt `-C` options should override the target's settings, like in C compilers,
//! but that was an essentially-unmarked route for making code incorrect and Rust unsound.
//! Confronted with programmers who prefer a compiler with a good UX instead of a lethal weapon,
//! we have almost-entirely recanted that notion, though we hope "target modifiers" will offer
//! a way to have a decent UX yet still extend the necessary compiler controls, without
//! requiring a new target spec for each and every single possible target micro-variant.
//!
//! [JSON]: https://json.org

// All `use` statements from the old mod.rs that are actually needed by re-exported items.
// Most of these are not needed anymore as their consumers are in submodules.
use std::borrow::Cow; // For StaticCow
use std::collections::BTreeMap; // For LinkArgs

// Only import what is absolutely necessary for the few items that mod.rs itself uses
// (like the comments above needing Target::search, debug! needs tracing)
use ::rustc_abi::Align; // For ToJson for Align
use tracing::debug; // For the debug! macro in supported_targets!
use crate::json::{Json, ToJson}; // For ToJson for Align

// Cow-Vec-Str: Cow<'static, [Cow<'static, str>]>
macro_rules! cvs {
    () => {
        ::std::borrow::Cow::Borrowed(&[])
    };
    ($($x:expr),+ $(,)?) => {
        ::std::borrow::Cow::Borrowed(&[
            $(
                ::std::borrow::Cow::Borrowed($x),
            )*
        ])
    };
}

pub(crate) use cvs;

// Module declarations
pub mod crt_objects;
pub mod target;
pub mod abi_map;
pub mod base;
pub mod json;
pub mod linker_flavor;
pub mod link_self_contained;
pub mod linker_features;
pub mod panic_strategy;
pub mod on_broken_pipe;
pub mod relro_level;
pub mod symbol_visibility;
pub mod small_data_threshold_support;
pub mod merge_functions;
pub mod reloc_model;
pub mod code_model;
pub mod float_abi;
pub mod rustc_abi;
pub mod tls_model;
pub mod link_output_kind;
pub mod debuginfo_kind; // ADDED
pub mod target_options;
pub mod target_tuple;
pub mod split_debuginfo;
pub mod stack_probe_type;
pub mod sanitizer_set;
pub mod frame_pointer;
pub mod stack_protector;
pub mod binary_format;
pub mod target_warnings;
pub mod arch;
pub mod os;
pub mod env;
pub mod targets; // The new targets module

// Public re-exports
pub use abi_map::{AbiMap, AbiMapping};
pub use base::apple;
pub use base::avr::ef_avr_arch;
pub use json::json_schema;
pub use target::{Target, TargetMetadata, X86Abi, HasTargetSpec, HasX86AbiOpt, Abi};
pub use linker_flavor::*; // Re-export everything from linker_flavor
pub use link_self_contained::*;
pub use linker_features::*;
pub use panic_strategy::*;
pub use on_broken_pipe::*;
pub use relro_level::*;
pub use symbol_visibility::*;
pub use small_data_threshold_support::*;
pub use merge_functions::*;
pub use reloc_model::*;
pub use code_model::*;
pub use float_abi::*;
pub use rustc_abi::*;
pub use tls_model::*;
pub use link_output_kind::*;
pub use debuginfo_kind::*; // ADDED
pub use target_options::TargetOptions;
pub use target_tuple::*;
pub use split_debuginfo::*;
pub use stack_probe_type::*;
pub use sanitizer_set::*;
pub use frame_pointer::*;
pub use stack_protector::*;
pub use binary_format::*;
pub use target_warnings::*;
pub use arch::*;
pub use os::*;
pub use env::*;
pub use targets::*; // Re-export everything from targets