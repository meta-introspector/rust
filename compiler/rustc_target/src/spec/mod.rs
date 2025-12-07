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

use core::result::Result;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, io};

use rustc_abi::{
    Align, CanonAbi, Endian, ExternAbi, Integer, Size, TargetDataLayout, TargetDataLayoutErrors,
};
use rustc_data_structures::fx::{FxHashSet, FxIndexSet};
use rustc_error_messages::{DiagArgValue, IntoDiagArg, into_diag_arg_using_display};
use rustc_fs_util::try_canonicalize;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_span::{Symbol, kw, sym};
use serde_json::Value;
use tracing::debug;

use crate::json::{Json, ToJson};
use crate::spec::crt_objects::CrtObjects;

pub mod crt_objects;

mod abi_map;
mod base;
mod json;

pub use abi_map::{AbiMap, AbiMapping};
pub use base::apple;
pub use base::avr::ef_avr_arch;
pub use json::json_schema;

mod linker_flavor;
pub use linker_flavor::*;

mod link_self_contained;
pub use link_self_contained::*;

mod linker_features;
pub use linker_features::*;

mod panic_strategy;
pub use panic_strategy::*;

mod on_broken_pipe;
pub use on_broken_pipe::*;

mod relro_level;
pub use relro_level::*;

mod symbol_visibility;
pub use symbol_visibility::*;

mod small_data_threshold_support;
pub use small_data_threshold_support::*;

mod merge_functions;
pub use merge_functions::*;

mod reloc_model;
pub use reloc_model::*;

mod code_model;
pub use code_model::*;

mod float_abi;
pub use float_abi::*;

mod rustc_abi;
pub use rustc_abi::*;

mod tls_model;
pub use tls_model::*;

mod link_output_kind;
pub use link_output_kind::*;
pub mod target_options;
pub mod target_tuple;
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Cc {

/// (`argv[0]`) is named in specific way, otherwise it doesn't work and requires a
/// `-flavor LLD_FLAVOR` argument to choose which logic to use. Our shipped `rust-lld` in
/// particular is not named in such specific way, so it needs the flavor option, so we make our
/// linker flavors sufficiently fine-grained to satisfy LLD without inferring its flavor from other
/// target properties, in accordance with the first design goal.
///
/// The first component of the flavor is tightly coupled with the compilation target,
/// while the `Cc` and `Lld` flags can vary within the same target.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LinkerFlavor {
    /// Unix-like linker with GNU extensions (both naked and compiler-wrapped forms).
    /// Besides similar "default" Linux/BSD linkers this also includes Windows/GNU linker,
    /// which is somewhat different because it doesn't produce ELFs.
    Gnu(Cc, Lld),
    /// Unix-like linker for Apple targets (both naked and compiler-wrapped forms).
    /// Extracted from the "umbrella" `Unix` flavor due to its corresponding LLD flavor.
    Darwin(Cc, Lld),
    /// Unix-like linker for Wasm targets (both naked and compiler-wrapped forms).
    /// Extracted from the "umbrella" `Unix` flavor due to its corresponding LLD flavor.
    /// Non-LLD version does not exist, so the lld flag is currently hardcoded here.
    WasmLld(Cc),
    /// Basic Unix-like linker for "any other Unix" targets (Solaris/illumos, L4Re, MSP430, etc),

        {
            Err((Some(Cc::No), Some(Lld::Yes)))
        } else if stem == "ld" || stem.ends_with("-ld") || stem == "link" {
            Err((Some(Cc::No), Some(Lld::No)))
        } else {
            Err((None, None))
        }
    }

    fn with_hints(self, (cc_hint, lld_hint): (Option<Cc>, Option<Lld>)) -> LinkerFlavor {
        match self {
            LinkerFlavor::Gnu(cc, lld) => {
                LinkerFlavor::Gnu(cc_hint.unwrap_or(cc), lld_hint.unwrap_or(lld))
            }
            LinkerFlavor::Darwin(cc, lld) => {
                LinkerFlavor::Darwin(cc_hint.unwrap_or(cc), lld_hint.unwrap_or(lld))
            }
            LinkerFlavor::WasmLld(cc) => LinkerFlavor::WasmLld(cc_hint.unwrap_or(cc)),
            LinkerFlavor::Unix(cc) => LinkerFlavor::Unix(cc_hint.unwrap_or(cc)),
            LinkerFlavor::Msvc(lld) => LinkerFlavor::Msvc(lld_hint.unwrap_or(lld)),
            LinkerFlavor::EmCc | LinkerFlavor::Bpf | LinkerFlavor::Llbc | LinkerFlavor::Ptx => self,
        }
    }

    pub fn with_cli_hints(self, cli: LinkerFlavorCli) -> LinkerFlavor {
        self.with_hints(LinkerFlavor::infer_cli_hints(cli))
    }

    pub fn with_linker_hints(self, linker_stem: &str) -> LinkerFlavor {
        match LinkerFlavor::infer_linker_hints(linker_stem) {
            Ok(linker_flavor) => linker_flavor,
            Err(hints) => self.with_hints(hints),
        }
    }

    pub fn check_compatibility(self, cli: LinkerFlavorCli) -> Option<String> {
        let compatible = |cli| {
            // The CLI flavor should be compatible with the target if:
            match (self, cli) {
                // 1. they are counterparts: they have the same principal flavor.
                (LinkerFlavor::Gnu(..), LinkerFlavorCli::Gnu(..))
                | (LinkerFlavor::Darwin(..), LinkerFlavorCli::Darwin(..))
                | (LinkerFlavor::WasmLld(..), LinkerFlavorCli::WasmLld(..))
                | (LinkerFlavor::Unix(..), LinkerFlavorCli::Unix(..))
                | (LinkerFlavor::Msvc(..), LinkerFlavorCli::Msvc(..))
                | (LinkerFlavor::EmCc, LinkerFlavorCli::EmCc)
                | (LinkerFlavor::Bpf, LinkerFlavorCli::Bpf)
                | (LinkerFlavor::Llbc, LinkerFlavorCli::Llbc)
                | (LinkerFlavor::Ptx, LinkerFlavorCli::Ptx) => return true,
                // 2. The linker flavor is independent of target and compatible
                (LinkerFlavor::Ptx, LinkerFlavorCli::Llbc) => return true,
                _ => {}
            }

            // 3. or, the flavor is legacy and survives this roundtrip.
            cli == self.with_cli_hints(cli).to_cli()
        };
        (!compatible(cli)).then(|| {
            LinkerFlavorCli::all()
                .iter()
                .filter(|cli| compatible(**cli))
                .map(|cli| cli.desc())
                .intersperse(", ")
                .collect()
        })
    }

    pub fn lld_flavor(self) -> LldFlavor {
        match self {
            LinkerFlavor::Gnu(..)
            | LinkerFlavor::Unix(..)
            | LinkerFlavor::EmCc
            | LinkerFlavor::Bpf
            | LinkerFlavor::Llbc
            | LinkerFlavor::Ptx => LldFlavor::Ld,
            LinkerFlavor::Darwin(..) => LldFlavor::Ld64,
            LinkerFlavor::WasmLld(..) => LldFlavor::Wasm,
            LinkerFlavor::Msvc(..) => LldFlavor::Link,
        }
    }

    pub fn is_gnu(self) -> bool {
        matches!(self, LinkerFlavor::Gnu(..))
    }

    /// Returns whether the flavor uses the `lld` linker.
    pub fn uses_lld(self) -> bool {
        // Exhaustive match in case new flavors are added in the future.
        match self {
            LinkerFlavor::Gnu(_, Lld::Yes)
            | LinkerFlavor::Darwin(_, Lld::Yes)
            | LinkerFlavor::WasmLld(..)
            | LinkerFlavor::EmCc
            | LinkerFlavor::Msvc(Lld::Yes) => true,
            LinkerFlavor::Gnu(..)
            | LinkerFlavor::Darwin(..)
            | LinkerFlavor::Msvc(_)
            | LinkerFlavor::Unix(_)
            | LinkerFlavor::Bpf
            | LinkerFlavor::Llbc
            | LinkerFlavor::Ptx => false,
        }
    }

    /// Returns whether the flavor calls the linker via a C/C++ compiler.
    pub fn uses_cc(self) -> bool {
        // Exhaustive match in case new flavors are added in the future.
        match self {
            LinkerFlavor::Gnu(Cc::Yes, _)
            | LinkerFlavor::Darwin(Cc::Yes, _)
            | LinkerFlavor::WasmLld(Cc::Yes)
            | LinkerFlavor::Unix(Cc::Yes)
            | LinkerFlavor::EmCc => true,
            LinkerFlavor::Gnu(..)
            | LinkerFlavor::Darwin(..)
            | LinkerFlavor::WasmLld(_)
            | LinkerFlavor::Msvc(_)
            | LinkerFlavor::Unix(_)
            | LinkerFlavor::Bpf
            | LinkerFlavor::Llbc
            | LinkerFlavor::Ptx => false,
        }
    }

    /// For flavors with an `Lld` component, ensure it's enabled. Otherwise, returns the given
    /// flavor unmodified.
    pub fn with_lld_enabled(self) -> LinkerFlavor {
        match self {
            LinkerFlavor::Gnu(cc, Lld::No) => LinkerFlavor::Gnu(cc, Lld::Yes),
            LinkerFlavor::Darwin(cc, Lld::No) => LinkerFlavor::Darwin(cc, Lld::Yes),
            LinkerFlavor::Msvc(Lld::No) => LinkerFlavor::Msvc(Lld::Yes),
            _ => self,
        }
    }

    /// For flavors with an `Lld` component, ensure it's disabled. Otherwise, returns the given
    /// flavor unmodified.
    pub fn with_lld_disabled(self) -> LinkerFlavor {
        match self {
            LinkerFlavor::Gnu(cc, Lld::Yes) => LinkerFlavor::Gnu(cc, Lld::No),
            LinkerFlavor::Darwin(cc, Lld::Yes) => LinkerFlavor::Darwin(cc, Lld::No),
            LinkerFlavor::Msvc(Lld::Yes) => LinkerFlavor::Msvc(Lld::No),
            _ => self,
        }
    }
}

macro_rules! linker_flavor_cli_impls {
    ($(($($flavor:tt)*) $string:literal)*) => (
        impl LinkerFlavorCli {
            const fn all() -> &'static [LinkerFlavorCli] {
                &[$($($flavor)*,)*]
            }

            pub const fn one_of() -> &'static str {
                concat!("one of: ", $($string, " ",)*)
            }

            pub fn desc(self) -> &'static str {
                match self {
                    $($($flavor)* => $string,)*
                }
            }
        }

        impl FromStr for LinkerFlavorCli {
            type Err = String;

            fn from_str(s: &str) -> Result<LinkerFlavorCli, Self::Err> {
                Ok(match s {
                    $($string => $($flavor)*,)*
                    _ => return Err(format!("invalid linker flavor, allowed values: {}", Self::one_of())),
                })
            }
        }
    )
}

linker_flavor_cli_impls! {
    (LinkerFlavorCli::Gnu(Cc::No, Lld::No)) "gnu"
    (LinkerFlavorCli::Gnu(Cc::No, Lld::Yes)) "gnu-lld"
    (LinkerFlavorCli::Gnu(Cc::Yes, Lld::No)) "gnu-cc"
    (LinkerFlavorCli::Gnu(Cc::Yes, Lld::Yes)) "gnu-lld-cc"
    (LinkerFlavorCli::Darwin(Cc::No, Lld::No)) "darwin"
    (LinkerFlavorCli::Darwin(Cc::No, Lld::Yes)) "darwin-lld"
    (LinkerFlavorCli::Darwin(Cc::Yes, Lld::No)) "darwin-cc"
    (LinkerFlavorCli::Darwin(Cc::Yes, Lld::Yes)) "darwin-lld-cc"
    (LinkerFlavorCli::WasmLld(Cc::No)) "wasm-lld"
    (LinkerFlavorCli::WasmLld(Cc::Yes)) "wasm-lld-cc"
    (LinkerFlavorCli::Unix(Cc::No)) "unix"
    (LinkerFlavorCli::Unix(Cc::Yes)) "unix-cc"
    (LinkerFlavorCli::Msvc(Lld::Yes)) "msvc-lld"
    (LinkerFlavorCli::Msvc(Lld::No)) "msvc"
    (LinkerFlavorCli::EmCc) "em-cc"
    (LinkerFlavorCli::Bpf) "bpf"
    (LinkerFlavorCli::Llbc) "llbc"
    (LinkerFlavorCli::Ptx) "ptx"

    // Legacy stable flavors
    (LinkerFlavorCli::Gcc) "gcc"
    (LinkerFlavorCli::Ld) "ld"
    (LinkerFlavorCli::Lld(LldFlavor::Ld)) "ld.lld"
    (LinkerFlavorCli::Lld(LldFlavor::Ld64)) "ld64.lld"
    (LinkerFlavorCli::Lld(LldFlavor::Link)) "lld-link"
    (LinkerFlavorCli::Lld(LldFlavor::Wasm)) "wasm-ld"
    (LinkerFlavorCli::Em) "em"
}

crate::json::serde_deserialize_from_str!(LinkerFlavorCli);
impl schemars::JsonSchema for LinkerFlavorCli {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "LinkerFlavor".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let all: Vec<&'static str> =
            Self::all().iter().map(|flavor| flavor.desc()).collect::<Vec<_>>();
        schemars::json_schema! ({
            "type": "string",
            "enum": all
        })
        .into()
    }
}

impl ToJson for LinkerFlavorCli {
    fn to_json(&self) -> Json {
        self.desc().to_json()
    }
}





























pub type LinkArgs = BTreeMap<LinkerFlavor, Vec<StaticCow<str>>>;
pub type LinkArgsCli = BTreeMap<LinkerFlavorCli, Vec<StaticCow<str>>>;

mod split_debuginfo;
pub use split_debuginfo::*;

mod stack_probe_type;
pub use stack_probe_type::*;

mod sanitizer_set;
pub use sanitizer_set::*;

mod frame_pointer;
pub use frame_pointer::*;

mod stack_protector;
pub use stack_protector::*;

mod binary_format;
pub use binary_format::*;















impl ToJson for Align {
    fn to_json(&self) -> Json {
        self.bits().to_json()
    }
}

macro_rules! supported_targets {
    ( $(($tuple:literal, $module:ident),)+ ) => {
        mod targets {
            $(pub(crate) mod $module;)+
        }

        /// List of supported targets
        pub static TARGETS: &[&str] = &[$($tuple),+];

        fn load_builtin(target: &str) -> Option<Target> {
            let t = match target {
                $( $tuple => targets::$module::target(), )+
                _ => return None,
            };
            debug!("got builtin target: {:?}", t);
            Some(t)
        }

        fn load_all_builtins() -> impl Iterator<Item = Target> {
            [
                $( targets::$module::target, )+
            ]
            .into_iter()
            .map(|f| f())
        }

        #[cfg(test)]
        mod tests {
            // Cannot put this into a separate file without duplication, make an exception.
            $(
                #[test] // `#[test]`
                fn $module() {
                    crate::spec::targets::$module::target().test_target()
                }
            )+
        }
    };
}

supported_targets! {
    ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
    ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
    ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
    ("i586-unknown-linux-gnu", i586_unknown_linux_gnu),
    ("loongarch64-unknown-linux-gnu", loongarch64_unknown_linux_gnu),
    ("loongarch64-unknown-linux-musl", loongarch64_unknown_linux_musl),
    ("m68k-unknown-linux-gnu", m68k_unknown_linux_gnu),
    ("m68k-unknown-none-elf", m68k_unknown_none_elf),
    ("csky-unknown-linux-gnuabiv2", csky_unknown_linux_gnuabiv2),
    ("csky-unknown-linux-gnuabiv2hf", csky_unknown_linux_gnuabiv2hf),
    ("mips-unknown-linux-gnu", mips_unknown_linux_gnu),
    ("mips64-unknown-linux-gnuabi64", mips64_unknown_linux_gnuabi64),
    ("mips64el-unknown-linux-gnuabi64", mips64el_unknown_linux_gnuabi64),
    ("mipsisa32r6-unknown-linux-gnu", mipsisa32r6_unknown_linux_gnu),
    ("mipsisa32r6el-unknown-linux-gnu", mipsisa32r6el_unknown_linux_gnu),
    ("mipsisa64r6-unknown-linux-gnuabi64", mipsisa64r6_unknown_linux_gnuabi64),
    ("mipsisa64r6el-unknown-linux-gnuabi64", mipsisa64r6el_unknown_linux_gnuabi64),
    ("mipsel-unknown-linux-gnu", mipsel_unknown_linux_gnu),
    ("powerpc-unknown-linux-gnu", powerpc_unknown_linux_gnu),
    ("powerpc-unknown-linux-gnuspe", powerpc_unknown_linux_gnuspe),
    ("powerpc-unknown-linux-musl", powerpc_unknown_linux_musl),
    ("powerpc-unknown-linux-muslspe", powerpc_unknown_linux_muslspe),
    ("powerpc64-ibm-aix", powerpc64_ibm_aix),
    ("powerpc64-unknown-linux-gnu", powerpc64_unknown_linux_gnu),
    ("powerpc64-unknown-linux-musl", powerpc64_unknown_linux_musl),
    ("powerpc64le-unknown-linux-gnu", powerpc64le_unknown_linux_gnu),
    ("powerpc64le-unknown-linux-musl", powerpc64le_unknown_linux_musl),
    ("s390x-unknown-linux-gnu", s390x_unknown_linux_gnu),
    ("s390x-unknown-linux-musl", s390x_unknown_linux_musl),
    ("sparc-unknown-linux-gnu", sparc_unknown_linux_gnu),
    ("sparc64-unknown-linux-gnu", sparc64_unknown_linux_gnu),
    ("arm-unknown-linux-gnueabi", arm_unknown_linux_gnueabi),
    ("arm-unknown-linux-gnueabihf", arm_unknown_linux_gnueabihf),
    ("armeb-unknown-linux-gnueabi", armeb_unknown_linux_gnueabi),
    ("arm-unknown-linux-musleabi", arm_unknown_linux_musleabi),
    ("arm-unknown-linux-musleabihf", arm_unknown_linux_musleabihf),
    ("armv4t-unknown-linux-gnueabi", armv4t_unknown_linux_gnueabi),
    ("armv5te-unknown-linux-gnueabi", armv5te_unknown_linux_gnueabi),
    ("armv5te-unknown-linux-musleabi", armv5te_unknown_linux_musleabi),
    ("armv5te-unknown-linux-uclibceabi", armv5te_unknown_linux_uclibceabi),
    ("armv7-unknown-linux-gnueabi", armv7_unknown_linux_gnueabi),
    ("armv7-unknown-linux-gnueabihf", armv7_unknown_linux_gnueabihf),
    ("thumbv7neon-unknown-linux-gnueabihf", thumbv7neon_unknown_linux_gnueabihf),
    ("thumbv7neon-unknown-linux-musleabihf", thumbv7neon_unknown_linux_musleabihf),
    ("armv7-unknown-linux-musleabi", armv7_unknown_linux_musleabi),
    ("armv7-unknown-linux-musleabihf", armv7_unknown_linux_musleabihf),
    ("aarch64-unknown-linux-gnu", aarch64_unknown_linux_gnu),
    ("aarch64-unknown-linux-musl", aarch64_unknown_linux_musl),
    ("aarch64_be-unknown-linux-musl", aarch64_be_unknown_linux_musl),
    ("x86_64-unknown-linux-musl", x86_64_unknown_linux_musl),
    ("i686-unknown-linux-musl", i686_unknown_linux_musl),
    ("i586-unknown-linux-musl", i586_unknown_linux_musl),
    ("mips-unknown-linux-musl", mips_unknown_linux_musl),
    ("mipsel-unknown-linux-musl", mipsel_unknown_linux_musl),
    ("mips64-unknown-linux-muslabi64", mips64_unknown_linux_muslabi64),
    ("mips64el-unknown-linux-muslabi64", mips64el_unknown_linux_muslabi64),
    ("hexagon-unknown-linux-musl", hexagon_unknown_linux_musl),
    ("hexagon-unknown-none-elf", hexagon_unknown_none_elf),
    ("hexagon-unknown-qurt", hexagon_unknown_qurt),

    ("mips-unknown-linux-uclibc", mips_unknown_linux_uclibc),
    ("mipsel-unknown-linux-uclibc", mipsel_unknown_linux_uclibc),

    ("i686-linux-android", i686_linux_android),
    ("x86_64-linux-android", x86_64_linux_android),
    ("arm-linux-androideabi", arm_linux_androideabi),
    ("armv7-linux-androideabi", armv7_linux_androideabi),
    ("thumbv7neon-linux-androideabi", thumbv7neon_linux_androideabi),
    ("aarch64-linux-android", aarch64_linux_android),
    ("riscv64-linux-android", riscv64_linux_android),

    ("aarch64-unknown-freebsd", aarch64_unknown_freebsd),
    ("armv6-unknown-freebsd", armv6_unknown_freebsd),
    ("armv7-unknown-freebsd", armv7_unknown_freebsd),
    ("i686-unknown-freebsd", i686_unknown_freebsd),
    ("powerpc-unknown-freebsd", powerpc_unknown_freebsd),
    ("powerpc64-unknown-freebsd", powerpc64_unknown_freebsd),
    ("powerpc64le-unknown-freebsd", powerpc64le_unknown_freebsd),
    ("riscv64gc-unknown-freebsd", riscv64gc_unknown_freebsd),
    ("x86_64-unknown-freebsd", x86_64_unknown_freebsd),

    ("x86_64-unknown-dragonfly", x86_64_unknown_dragonfly),

    ("aarch64-unknown-openbsd", aarch64_unknown_openbsd),
    ("i686-unknown-openbsd", i686_unknown_openbsd),
    ("powerpc-unknown-openbsd", powerpc_unknown_openbsd),
    ("powerpc64-unknown-openbsd", powerpc64_unknown_openbsd),
    ("riscv64gc-unknown-openbsd", riscv64gc_unknown_openbsd),
    ("sparc64-unknown-openbsd", sparc64_unknown_openbsd),
    ("x86_64-unknown-openbsd", x86_64_unknown_openbsd),

    ("aarch64-unknown-netbsd", aarch64_unknown_netbsd),
    ("aarch64_be-unknown-netbsd", aarch64_be_unknown_netbsd),
    ("armv6-unknown-netbsd-eabihf", armv6_unknown_netbsd_eabihf),
    ("armv7-unknown-netbsd-eabihf", armv7_unknown_netbsd_eabihf),
    ("i586-unknown-netbsd", i586_unknown_netbsd),
    ("i686-unknown-netbsd", i686_unknown_netbsd),
    ("mipsel-unknown-netbsd", mipsel_unknown_netbsd),
    ("powerpc-unknown-netbsd", powerpc_unknown_netbsd),
    ("riscv64gc-unknown-netbsd", riscv64gc_unknown_netbsd),
    ("sparc64-unknown-netbsd", sparc64_unknown_netbsd),
    ("x86_64-unknown-netbsd", x86_64_unknown_netbsd),

    ("i686-unknown-haiku", i686_unknown_haiku),
    ("x86_64-unknown-haiku", x86_64_unknown_haiku),

    ("aarch64-unknown-helenos", aarch64_unknown_helenos),
    ("i686-unknown-helenos", i686_unknown_helenos),
    ("powerpc-unknown-helenos", powerpc_unknown_helenos),
    ("sparc64-unknown-helenos", sparc64_unknown_helenos),
    ("x86_64-unknown-helenos", x86_64_unknown_helenos),

    ("i686-unknown-hurd-gnu", i686_unknown_hurd_gnu),
    ("x86_64-unknown-hurd-gnu", x86_64_unknown_hurd_gnu),

    ("aarch64-apple-darwin", aarch64_apple_darwin),
    ("arm64e-apple-darwin", arm64e_apple_darwin),
    ("x86_64-apple-darwin", x86_64_apple_darwin),
    ("x86_64h-apple-darwin", x86_64h_apple_darwin),
    ("i686-apple-darwin", i686_apple_darwin),

    ("aarch64-unknown-fuchsia", aarch64_unknown_fuchsia),
    ("riscv64gc-unknown-fuchsia", riscv64gc_unknown_fuchsia),
    ("x86_64-unknown-fuchsia", x86_64_unknown_fuchsia),

    ("avr-none", avr_none),

    ("x86_64-unknown-l4re-uclibc", x86_64_unknown_l4re_uclibc),

    ("aarch64-unknown-redox", aarch64_unknown_redox),
    ("i586-unknown-redox", i586_unknown_redox),
    ("riscv64gc-unknown-redox", riscv64gc_unknown_redox),
    ("x86_64-unknown-redox", x86_64_unknown_redox),

    ("x86_64-unknown-managarm-mlibc", x86_64_unknown_managarm_mlibc),
    ("aarch64-unknown-managarm-mlibc", aarch64_unknown_managarm_mlibc),
    ("riscv64gc-unknown-managarm-mlibc", riscv64gc_unknown_managarm_mlibc),

    ("i386-apple-ios", i386_apple_ios),
    ("x86_64-apple-ios", x86_64_apple_ios),
    ("aarch64-apple-ios", aarch64_apple_ios),
    ("arm64e-apple-ios", arm64e_apple_ios),
    ("armv7s-apple-ios", armv7s_apple_ios),
    ("x86_64-apple-ios-macabi", x86_64_apple_ios_macabi),
    ("aarch64-apple-ios-macabi", aarch64_apple_ios_macabi),
    ("aarch64-apple-ios-sim", aarch64_apple_ios_sim),

    ("aarch64-apple-tvos", aarch64_apple_tvos),
    ("aarch64-apple-tvos-sim", aarch64_apple_tvos_sim),
    ("arm64e-apple-tvos", arm64e_apple_tvos),
    ("x86_64-apple-tvos", x86_64_apple_tvos),

    ("armv7k-apple-watchos", armv7k_apple_watchos),
    ("arm64_32-apple-watchos", arm64_32_apple_watchos),
    ("x86_64-apple-watchos-sim", x86_64_apple_watchos_sim),
    ("aarch64-apple-watchos", aarch64_apple_watchos),
    ("aarch64-apple-watchos-sim", aarch64_apple_watchos_sim),

    ("aarch64-apple-visionos", aarch64_apple_visionos),
    ("aarch64-apple-visionos-sim", aarch64_apple_visionos_sim),

    ("armebv7r-none-eabi", armebv7r_none_eabi),
    ("armebv7r-none-eabihf", armebv7r_none_eabihf),
    ("armv7r-none-eabi", armv7r_none_eabi),
    ("armv7r-none-eabihf", armv7r_none_eabihf),
    ("armv8r-none-eabihf", armv8r_none_eabihf),

    ("armv7-rtems-eabihf", armv7_rtems_eabihf),

    ("x86_64-pc-solaris", x86_64_pc_solaris),
    ("sparcv9-sun-solaris", sparcv9_sun_solaris),

    ("x86_64-unknown-illumos", x86_64_unknown_illumos),
    ("aarch64-unknown-illumos", aarch64_unknown_illumos),

    ("x86_64-pc-windows-gnu", x86_64_pc_windows_gnu),
    ("x86_64-uwp-windows-gnu", x86_64_uwp_windows_gnu),
    ("x86_64-win7-windows-gnu", x86_64_win7_windows_gnu),
    ("i686-pc-windows-gnu", i686_pc_windows_gnu),
    ("i686-uwp-windows-gnu", i686_uwp_windows_gnu),
    ("i686-win7-windows-gnu", i686_win7_windows_gnu),

    ("aarch64-pc-windows-gnullvm", aarch64_pc_windows_gnullvm),
    ("i686-pc-windows-gnullvm", i686_pc_windows_gnullvm),
    ("x86_64-pc-windows-gnullvm", x86_64_pc_windows_gnullvm),

    ("aarch64-pc-windows-msvc", aarch64_pc_windows_msvc),
    ("aarch64-uwp-windows-msvc", aarch64_uwp_windows_msvc),
    ("arm64ec-pc-windows-msvc", arm64ec_pc_windows_msvc),
    ("x86_64-pc-windows-msvc", x86_64_pc_windows_msvc),
    ("x86_64-uwp-windows-msvc", x86_64_uwp_windows_msvc),
    ("x86_64-win7-windows-msvc", x86_64_win7_windows_msvc),
    ("i686-pc-windows-msvc", i686_pc_windows_msvc),
    ("i686-uwp-windows-msvc", i686_uwp_windows_msvc),
    ("i686-win7-windows-msvc", i686_win7_windows_msvc),
    ("thumbv7a-pc-windows-msvc", thumbv7a_pc_windows_msvc),
    ("thumbv7a-uwp-windows-msvc", thumbv7a_uwp_windows_msvc),

    ("wasm32-unknown-emscripten", wasm32_unknown_emscripten),
    ("wasm32-unknown-unknown", wasm32_unknown_unknown),
    ("wasm32v1-none", wasm32v1_none),
    ("wasm32-wasip1", wasm32_wasip1),
    ("wasm32-wasip2", wasm32_wasip2),
    ("wasm32-wasip3", wasm32_wasip3),
    ("wasm32-wasip1-threads", wasm32_wasip1_threads),
    ("wasm32-wali-linux-musl", wasm32_wali_linux_musl),
    ("wasm64-unknown-unknown", wasm64_unknown_unknown),

    ("thumbv6m-none-eabi", thumbv6m_none_eabi),
    ("thumbv7m-none-eabi", thumbv7m_none_eabi),
    ("thumbv7em-none-eabi", thumbv7em_none_eabi),
    ("thumbv7em-none-eabihf", thumbv7em_none_eabihf),
    ("thumbv8m.base-none-eabi", thumbv8m_base_none_eabi),
    ("thumbv8m.main-none-eabi", thumbv8m_main_none_eabi),
    ("thumbv8m.main-none-eabihf", thumbv8m_main_none_eabihf),

    ("armv7a-none-eabi", armv7a_none_eabi),
    ("armv7a-none-eabihf", armv7a_none_eabihf),
    ("armv7a-nuttx-eabi", armv7a_nuttx_eabi),
    ("armv7a-nuttx-eabihf", armv7a_nuttx_eabihf),
    ("armv7a-vex-v5", armv7a_vex_v5),

    ("msp430-none-elf", msp430_none_elf),

    ("aarch64_be-unknown-hermit", aarch64_be_unknown_hermit),
    ("aarch64-unknown-hermit", aarch64_unknown_hermit),
    ("riscv64gc-unknown-hermit", riscv64gc_unknown_hermit),
    ("x86_64-unknown-hermit", x86_64_unknown_hermit),
    ("x86_64-unknown-motor", x86_64_unknown_motor),

    ("x86_64-unikraft-linux-musl", x86_64_unikraft_linux_musl),

    ("armv7-unknown-trusty", armv7_unknown_trusty),
    ("aarch64-unknown-trusty", aarch64_unknown_trusty),
    ("x86_64-unknown-trusty", x86_64_unknown_trusty),

    ("riscv32i-unknown-none-elf", riscv32i_unknown_none_elf),
    ("riscv32im-risc0-zkvm-elf", riscv32im_risc0_zkvm_elf),
    ("riscv32im-unknown-none-elf", riscv32im_unknown_none_elf),
    ("riscv32ima-unknown-none-elf", riscv32ima_unknown_none_elf),
    ("riscv32imc-unknown-none-elf", riscv32imc_unknown_none_elf),
    ("riscv32imc-esp-espidf", riscv32imc_esp_espidf),
    ("riscv32imac-esp-espidf", riscv32imac_esp_espidf),
    ("riscv32imafc-esp-espidf", riscv32imafc_esp_espidf),

    ("riscv32e-unknown-none-elf", riscv32e_unknown_none_elf),
    ("riscv32em-unknown-none-elf", riscv32em_unknown_none_elf),
    ("riscv32emc-unknown-none-elf", riscv32emc_unknown_none_elf),

    ("riscv32imac-unknown-none-elf", riscv32imac_unknown_none_elf),
    ("riscv32imafc-unknown-none-elf", riscv32imafc_unknown_none_elf),
    ("riscv32imac-unknown-xous-elf", riscv32imac_unknown_xous_elf),
    ("riscv32gc-unknown-linux-gnu", riscv32gc_unknown_linux_gnu),
    ("riscv32gc-unknown-linux-musl", riscv32gc_unknown_linux_musl),
    ("riscv64imac-unknown-none-elf", riscv64imac_unknown_none_elf),
    ("riscv64gc-unknown-none-elf", riscv64gc_unknown_none_elf),
    ("riscv64gc-unknown-linux-gnu", riscv64gc_unknown_linux_gnu),
    ("riscv64gc-unknown-linux-musl", riscv64gc_unknown_linux_musl),
    ("riscv64a23-unknown-linux-gnu", riscv64a23_unknown_linux_gnu),

    ("sparc-unknown-none-elf", sparc_unknown_none_elf),

    ("loongarch32-unknown-none", loongarch32_unknown_none),
    ("loongarch32-unknown-none-softfloat", loongarch32_unknown_none_softfloat),
    ("loongarch64-unknown-none", loongarch64_unknown_none),
    ("loongarch64-unknown-none-softfloat", loongarch64_unknown_none_softfloat),

    ("aarch64-unknown-none", aarch64_unknown_none),
    ("aarch64-unknown-none-softfloat", aarch64_unknown_none_softfloat),
    ("aarch64_be-unknown-none-softfloat", aarch64_be_unknown_none_softfloat),
    ("aarch64-unknown-nuttx", aarch64_unknown_nuttx),

    ("x86_64-fortanix-unknown-sgx", x86_64_fortanix_unknown_sgx),

    ("x86_64-unknown-uefi", x86_64_unknown_uefi),
    ("i686-unknown-uefi", i686_unknown_uefi),
    ("aarch64-unknown-uefi", aarch64_unknown_uefi),

    ("nvptx64-nvidia-cuda", nvptx64_nvidia_cuda),

    ("amdgcn-amd-amdhsa", amdgcn_amd_amdhsa),

    ("xtensa-esp32-none-elf", xtensa_esp32_none_elf),
    ("xtensa-esp32-espidf", xtensa_esp32_espidf),
    ("xtensa-esp32s2-none-elf", xtensa_esp32s2_none_elf),
    ("xtensa-esp32s2-espidf", xtensa_esp32s2_espidf),
    ("xtensa-esp32s3-none-elf", xtensa_esp32s3_none_elf),
    ("xtensa-esp32s3-espidf", xtensa_esp32s3_espidf),

    ("i686-wrs-vxworks", i686_wrs_vxworks),
    ("x86_64-wrs-vxworks", x86_64_wrs_vxworks),
    ("armv7-wrs-vxworks-eabihf", armv7_wrs_vxworks_eabihf),
    ("aarch64-wrs-vxworks", aarch64_wrs_vxworks),
    ("powerpc-wrs-vxworks", powerpc_wrs_vxworks),
    ("powerpc-wrs-vxworks-spe", powerpc_wrs_vxworks_spe),
    ("powerpc64-wrs-vxworks", powerpc64_wrs_vxworks),
    ("riscv32-wrs-vxworks", riscv32_wrs_vxworks),
    ("riscv64-wrs-vxworks", riscv64_wrs_vxworks),

    ("aarch64-kmc-solid_asp3", aarch64_kmc_solid_asp3),
    ("armv7a-kmc-solid_asp3-eabi", armv7a_kmc_solid_asp3_eabi),
    ("armv7a-kmc-solid_asp3-eabihf", armv7a_kmc_solid_asp3_eabihf),

    ("mipsel-sony-psp", mipsel_sony_psp),
    ("mipsel-sony-psx", mipsel_sony_psx),
    ("mipsel-unknown-none", mipsel_unknown_none),
    ("mips-mti-none-elf", mips_mti_none_elf),
    ("mipsel-mti-none-elf", mipsel_mti_none_elf),
    ("thumbv4t-none-eabi", thumbv4t_none_eabi),
    ("armv4t-none-eabi", armv4t_none_eabi),
    ("thumbv5te-none-eabi", thumbv5te_none_eabi),
    ("armv5te-none-eabi", armv5te_none_eabi),

    ("aarch64_be-unknown-linux-gnu", aarch64_be_unknown_linux_gnu),
    ("aarch64-unknown-linux-gnu_ilp32", aarch64_unknown_linux_gnu_ilp32),
    ("aarch64_be-unknown-linux-gnu_ilp32", aarch64_be_unknown_linux_gnu_ilp32),

    ("bpfeb-unknown-none", bpfeb_unknown_none),
    ("bpfel-unknown-none", bpfel_unknown_none),

    ("armv6k-nintendo-3ds", armv6k_nintendo_3ds),

    ("aarch64-nintendo-switch-freestanding", aarch64_nintendo_switch_freestanding),

    ("armv7-sony-vita-newlibeabihf", armv7_sony_vita_newlibeabihf),

    ("armv7-unknown-linux-uclibceabi", armv7_unknown_linux_uclibceabi),
    ("armv7-unknown-linux-uclibceabihf", armv7_unknown_linux_uclibceabihf),

    ("x86_64-unknown-none", x86_64_unknown_none),

    ("aarch64-unknown-teeos", aarch64_unknown_teeos),

    ("mips64-openwrt-linux-musl", mips64_openwrt_linux_musl),

    ("aarch64-unknown-nto-qnx700", aarch64_unknown_nto_qnx700),
    ("aarch64-unknown-nto-qnx710", aarch64_unknown_nto_qnx710),
    ("aarch64-unknown-nto-qnx710_iosock", aarch64_unknown_nto_qnx710_iosock),
    ("aarch64-unknown-nto-qnx800", aarch64_unknown_nto_qnx800),
    ("x86_64-pc-nto-qnx710", x86_64_pc_nto_qnx710),
    ("x86_64-pc-nto-qnx710_iosock", x86_64_pc_nto_qnx710_iosock),
    ("x86_64-pc-nto-qnx800", x86_64_pc_nto_qnx800),
    ("i686-pc-nto-qnx700", i686_pc_nto_qnx700),

    ("aarch64-unknown-linux-ohos", aarch64_unknown_linux_ohos),
    ("armv7-unknown-linux-ohos", armv7_unknown_linux_ohos),
    ("loongarch64-unknown-linux-ohos", loongarch64_unknown_linux_ohos),
    ("x86_64-unknown-linux-ohos", x86_64_unknown_linux_ohos),

    ("x86_64-unknown-linux-none", x86_64_unknown_linux_none),

    ("thumbv6m-nuttx-eabi", thumbv6m_nuttx_eabi),
    ("thumbv7a-nuttx-eabi", thumbv7a_nuttx_eabi),
    ("thumbv7a-nuttx-eabihf", thumbv7a_nuttx_eabihf),
    ("thumbv7m-nuttx-eabi", thumbv7m_nuttx_eabi),
    ("thumbv7em-nuttx-eabi", thumbv7em_nuttx_eabi),
    ("thumbv7em-nuttx-eabihf", thumbv7em_nuttx_eabihf),
    ("thumbv8m.base-nuttx-eabi", thumbv8m_base_nuttx_eabi),
    ("thumbv8m.main-nuttx-eabi", thumbv8m_main_nuttx_eabi),
    ("thumbv8m.main-nuttx-eabihf", thumbv8m_main_nuttx_eabihf),
    ("riscv32imc-unknown-nuttx-elf", riscv32imc_unknown_nuttx_elf),
    ("riscv32imac-unknown-nuttx-elf", riscv32imac_unknown_nuttx_elf),
    ("riscv32imafc-unknown-nuttx-elf", riscv32imafc_unknown_nuttx_elf),
    ("riscv64imac-unknown-nuttx-elf", riscv64imac_unknown_nuttx_elf),
    ("riscv64gc-unknown-nuttx-elf", riscv64gc_unknown_nuttx_elf),
    ("x86_64-lynx-lynxos178", x86_64_lynx_lynxos178),

    ("x86_64-pc-cygwin", x86_64_pc_cygwin),
}

/// Cow-Vec-Str: Cow<'static, [Cow<'static, str>]>
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

mod target_warnings;
pub use target_warnings::*;

/// For the [`Target::check_consistency`] function, determines whether the given target is a builtin or a JSON
/// target.
#[derive(Copy, Clone, Debug, PartialEq)]
enum TargetKind {
    Json,
    Builtin,
}

mod arch;
pub use arch::*;

mod os;
pub use os::*;

mod env;
pub use env::*;





crate::target_spec_enum! {
    pub enum Env {
        Gnu = "gnu",
        MacAbi = "macabi",
        Mlibc = "mlibc",
        Msvc = "msvc",
        Musl = "musl",
        Newlib = "newlib",
        Nto70 = "nto70",
        Nto71 = "nto71",
        Nto71IoSock = "nto71_iosock",
        Nto80 = "nto80",
        Ohos = "ohos",
        Relibc = "relibc",
        Sgx = "sgx",
        Sim = "sim",
        P1 = "p1",
        P2 = "p2",
        P3 = "p3",
        Uclibc = "uclibc",
        V5 = "v5",
        Unspecified = "",
    }


impl crate::json::ToJson for Env {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}

crate::target_spec_enum! {
    pub enum Abi {
        Abi64 = "abi64",
        AbiV2 = "abiv2",
        AbiV2Hf = "abiv2hf",
        Eabi = "eabi",
        EabiHf = "eabihf",
        ElfV1 = "elfv1",
        ElfV2 = "elfv2",
        Fortanix = "fortanix",
        Ilp32 = "ilp32",
        Ilp32e = "ilp32e",
        Llvm = "llvm",
        MacAbi = "macabi",
        Sim = "sim",
        SoftFloat = "softfloat",
        Spe = "spe",
        Uwp = "uwp",
        VecDefault = "vec-default",
        VecExtAbi = "vec-extabi",
        X32 = "x32",
        Unspecified = "",
    }
    other_variant = Other;
}

        Symbol::intern(self.desc())
    }
}

impl crate::json::ToJson for Abi {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}

/// Everything `rustc` knows about how to compile for a specific target.
///
/// Every field here must be specified, and has no default value.
#[derive(PartialEq, Clone, Debug)]
pub struct Target {
    /// Unversioned target tuple to pass to LLVM.
    ///
    /// Target tuples can optionally contain an OS version (notably Apple targets), which rustc
    /// cannot know without querying the environment.
    ///
    /// Use `rustc_codegen_ssa::back::versioned_llvm_target` if you need the full LLVM target.
    pub llvm_target: StaticCow<str>,
    /// Metadata about a target, for example the description or tier.
    /// Used for generating target documentation.
    pub metadata: TargetMetadata,
    /// Number of bits in a pointer. Influences the `target_pointer_width` `cfg` variable.
    pub pointer_width: u16,
    /// Architecture to use for ABI considerations. Valid options include: "x86",
    /// "x86_64", "arm", "aarch64", "mips", "powerpc", "powerpc64", and others.
    pub arch: Arch,
    /// [Data layout](https://llvm.org/docs/LangRef.html#data-layout) to pass to LLVM.
    pub data_layout: StaticCow<str>,
    /// Optional settings with defaults.
    pub options: TargetOptions,
}

/// Metadata about a target like the description or tier.
/// Part of #120745.
/// All fields are optional for now, but intended to be required in the future.
#[derive(Default, PartialEq, Clone, Debug)]
pub struct TargetMetadata {
    /// A short description of the target including platform requirements,
    /// for example "64-bit Linux (kernel 3.2+, glibc 2.17+)".
    pub description: Option<StaticCow<str>>,
    /// The tier of the target. 1, 2 or 3.
    pub tier: Option<u64>,
    /// Whether the Rust project ships host tools for a target.
    pub host_tools: Option<bool>,
    /// Whether a target has the `std` library. This is usually true for targets running
    /// on an operating system.
    pub std: Option<bool>,
}

impl Target {
    pub fn parse_data_layout(&self) -> Result<TargetDataLayout, TargetDataLayoutErrors<'_>> {
        let mut dl = TargetDataLayout::parse_from_llvm_datalayout_string(
            &self.data_layout,
            self.options.default_address_space,
        )?;

        // Perform consistency checks against the Target information.
        if dl.endian != self.endian {
            return Err(TargetDataLayoutErrors::InconsistentTargetArchitecture {
                dl: dl.endian.as_str(),
                target: self.endian.as_str(),
            });
        }

        let target_pointer_width: u64 = self.pointer_width.into();
        let dl_pointer_size: u64 = dl.pointer_size().bits();
        if dl_pointer_size != target_pointer_width {
            return Err(TargetDataLayoutErrors::InconsistentTargetPointerWidth {
                pointer_size: dl_pointer_size,
                target: self.pointer_width,
            });
        }

        dl.c_enum_min_size = Integer::from_size(Size::from_bits(
            self.c_enum_min_bits.unwrap_or(self.c_int_width as _),
        ))
        .map_err(|err| TargetDataLayoutErrors::InvalidBitsSize { err })?;

        Ok(dl)
    }
}

pub trait HasTargetSpec {
    fn target_spec(&self) -> &Target;
}

impl HasTargetSpec for Target {
    #[inline]
    fn target_spec(&self) -> &Target {
        self
    }
}

/// x86 (32-bit) abi options.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct X86Abi {
    /// On x86-32 targets, the regparm N causes the compiler to pass arguments
    /// in registers EAX, EDX, and ECX instead of on the stack.
    pub regparm: Option<u32>,
    /// Override the default ABI to return small structs in registers
    pub reg_struct_return: bool,
}

pub trait HasX86AbiOpt {
    fn x86_abi_opt(&self) -> X86Abi;
}

type StaticCow<T> = Cow<'static, T>;

/// Optional aspects of a target specification.
///
/// This has an implementation of `Default`, see each field for what the default is. In general,
/// these try to take "minimal defaults" that don't assume anything about the runtime they run in.
///
    /// executable, aka there is no native linker for this target.
    pub requires_lto: bool,

    /// This target has no support for threads.
    pub singlethread: bool,

    /// Whether library functions call lowering/optimization is disabled in LLVM
    /// for this target unconditionally.
    pub no_builtins: bool,

    /// The default visibility for symbols in this target.
    ///
    /// This value typically shouldn't be accessed directly, but through the
    /// `rustc_session::Session::default_visibility` method, which allows `rustc` users to override
    /// this setting using cmdline flags.
    pub default_visibility: Option<SymbolVisibility>,

    /// Whether a .debug_gdb_scripts section will be added to the output object file
    pub emit_debug_gdb_scripts: bool,

    /// Whether or not to unconditionally `uwtable` attributes on functions,
    /// typically because the platform needs to unwind for things like stack
    /// unwinders.
    pub requires_uwtable: bool,

    /// Whether or not to emit `uwtable` attributes on functions if `-C force-unwind-tables`
    /// is not specified and `uwtable` is not required on this target.
    pub default_uwtable: bool,

    /// Whether or not SIMD types are passed by reference in the Rust ABI,
    /// typically required if a target can be compiled with a mixed set of
    /// target features. This is `true` by default, and `false` for targets like
    /// wasm32 where the whole program either has simd or not.
    pub simd_types_indirect: bool,

    /// Pass a list of symbol which should be exported in the dylib to the linker.
    pub limit_rdylib_exports: bool,

    /// If set, have the linker export exactly these symbols, instead of using
    /// the usual logic to figure this out from the crate itself.
    pub override_export_symbols: Option<StaticCow<[StaticCow<str>]>>,

    /// Determines how or whether the MergeFunctions LLVM pass should run for
    /// this target. Either "disabled", "trampolines", or "aliases".
    /// The MergeFunctions pass is generally useful, but some targets may need
    /// to opt out. The default is "aliases".
    ///
    /// Workaround for: <https://github.com/rust-lang/rust/issues/57356>
    pub merge_functions: MergeFunctions,

    /// Use platform dependent mcount function
    pub mcount: StaticCow<str>,

    /// Use LLVM intrinsic for mcount function name
    pub llvm_mcount_intrinsic: Option<StaticCow<str>>,

    /// LLVM ABI name, corresponds to the '-mabi' parameter available in multilib C compilers
    /// and the `-target-abi` flag in llc. In the LLVM API this is `MCOptions.ABIName`.
    pub llvm_abiname: StaticCow<str>,

    /// Control the float ABI to use, for architectures that support it. The only architecture we
    /// currently use this for is ARM. Corresponds to the `-float-abi` flag in llc. In the LLVM API
    /// this is `FloatABIType`. (clang's `-mfloat-abi` is similar but more complicated since it
    /// can also affect the `soft-float` target feature.)
    ///
    /// If not provided, LLVM will infer the float ABI from the target triple (`llvm_target`).
    pub llvm_floatabi: Option<FloatAbi>,

    /// Picks a specific ABI for this target. This is *not* just for "Rust" ABI functions,
    /// it can also affect "C" ABI functions; the point is that this flag is interpreted by
    /// rustc and not forwarded to LLVM.
    /// So far, this is only used on x86.
    pub rustc_abi: Option<RustcAbi>,

    /// Whether or not RelaxElfRelocation flag will be passed to the linker
    pub relax_elf_relocations: bool,

    /// Additional arguments to pass to LLVM, similar to the `-C llvm-args` codegen option.
    pub llvm_args: StaticCow<[StaticCow<str>]>,

    /// Whether to use legacy .ctors initialization hooks rather than .init_array. Defaults
    /// to false (uses .init_array).
    pub use_ctors_section: bool,

    /// Whether the linker is instructed to add a `GNU_EH_FRAME` ELF header
    /// used to locate unwinding information is passed
    /// (only has effect if the linker is `ld`-like).
    pub eh_frame_header: bool,

    /// Is true if the target is an ARM architecture using thumb v1 which allows for
    /// thumb and arm interworking.
    pub has_thumb_interworking: bool,

    /// Which kind of debuginfo is used by this target?
    pub debuginfo_kind: DebuginfoKind,
    /// How to handle split debug information, if at all. Specifying `None` has
    /// target-specific meaning.
    pub split_debuginfo: SplitDebuginfo,
    /// Which kinds of split debuginfo are supported by the target?
    pub supported_split_debuginfo: StaticCow<[SplitDebuginfo]>,

    /// The sanitizers supported by this target
    ///
    /// Note that the support here is at a codegen level. If the machine code with sanitizer
    /// enabled can generated on this target, but the necessary supporting libraries are not
    /// distributed with the target, the sanitizer should still appear in this list for the target.
    pub supported_sanitizers: SanitizerSet,

    /// The sanitizers that are enabled by default on this target.
    ///
    /// Note that the support here is at a codegen level. If the machine code with sanitizer
    /// enabled can generated on this target, but the necessary supporting libraries are not
    /// distributed with the target, the sanitizer should still appear in this list for the target.
    pub default_sanitizers: SanitizerSet,

    /// Minimum number of bits in #[repr(C)] enum. Defaults to the size of c_int
    pub c_enum_min_bits: Option<u64>,

    /// Whether or not the DWARF `.debug_aranges` section should be generated.
    pub generate_arange_section: bool,

    /// Whether the target supports stack canary checks. `true` by default,
    /// since this is most common among tier 1 and tier 2 targets.
    pub supports_stack_protector: bool,

    /// The name of entry function.
    /// Default value is "main"
    pub entry_name: StaticCow<str>,

    /// The ABI of the entry function.
    /// Default value is `CanonAbi::C`
    pub entry_abi: CanonAbi,

    /// Whether the target supports XRay instrumentation.
    pub supports_xray: bool,

    /// The default address space for this target. When using LLVM as a backend, most targets simply
    /// use LLVM's default address space (0). Some other targets, such as CHERI targets, use a
    /// custom default address space (in this specific case, `200`).
    pub default_address_space: rustc_abi::AddressSpace,

    /// Whether the targets supports -Z small-data-threshold
    small_data_threshold_support: SmallDataThresholdSupport,
}

/// Add arguments for the given flavor and also for its "twin" flavors
/// that have a compatible command line interface.
fn add_link_args_iter(
    link_args: &mut LinkArgs,
    flavor: LinkerFlavor,
    args: impl Iterator<Item = StaticCow<str>> + Clone,
) {
    let mut insert = |flavor| link_args.entry(flavor).or_default().extend(args.clone());
    insert(flavor);
    match flavor {
        LinkerFlavor::Gnu(cc, lld) => {
            assert_eq!(lld, Lld::No);
            insert(LinkerFlavor::Gnu(cc, Lld::Yes));
        }
        LinkerFlavor::Darwin(cc, lld) => {
            assert_eq!(lld, Lld::No);
            insert(LinkerFlavor::Darwin(cc, Lld::Yes));
        }
        LinkerFlavor::Msvc(lld) => {
            assert_eq!(lld, Lld::No);
            insert(LinkerFlavor::Msvc(Lld::Yes));
        }
        LinkerFlavor::WasmLld(..)
        | LinkerFlavor::Unix(..)
        | LinkerFlavor::EmCc
        | LinkerFlavor::Bpf
        | LinkerFlavor::Llbc
        | LinkerFlavor::Ptx => {}
    }
}

fn add_link_args(link_args: &mut LinkArgs, flavor: LinkerFlavor, args: &[&'static str]) {
    add_link_args_iter(link_args, flavor, args.iter().copied().map(Cow::Borrowed))
}

impl TargetOptions {
    pub fn supports_comdat(&self) -> bool {
        // XCOFF and MachO don't support COMDAT.
        !self.is_like_aix && !self.is_like_darwin
    }
}

impl TargetOptions {
    fn link_args(flavor: LinkerFlavor, args: &[&'static str]) -> LinkArgs {
        let mut link_args = LinkArgs::new();
        add_link_args(&mut link_args, flavor, args);
        link_args
    }

    fn add_pre_link_args(&mut self, flavor: LinkerFlavor, args: &[&'static str]) {
        add_link_args(&mut self.pre_link_args, flavor, args);
    }

    fn update_from_cli(&mut self) {
        self.linker_flavor = LinkerFlavor::from_cli_json(
            self.linker_flavor_json,
            self.lld_flavor_json,
            self.linker_is_gnu_json,
        );
        for (args, args_json) in [
            (&mut self.pre_link_args, &self.pre_link_args_json),
            (&mut self.late_link_args, &self.late_link_args_json),
            (&mut self.late_link_args_dynamic, &self.late_link_args_dynamic_json),
            (&mut self.late_link_args_static, &self.late_link_args_static_json),
            (&mut self.post_link_args, &self.post_link_args_json),
        ] {
            args.clear();
            for (flavor, args_json) in args_json {
                let linker_flavor = self.linker_flavor.with_cli_hints(*flavor);
                // Normalize to no lld to avoid asserts.
                let linker_flavor = match linker_flavor {
                    LinkerFlavor::Gnu(cc, _) => LinkerFlavor::Gnu(cc, Lld::No),
                    LinkerFlavor::Darwin(cc, _) => LinkerFlavor::Darwin(cc, Lld::No),
                    LinkerFlavor::Msvc(_) => LinkerFlavor::Msvc(Lld::No),
                    _ => linker_flavor,
                };
                if !args.contains_key(&linker_flavor) {
                    add_link_args_iter(args, linker_flavor, args_json.iter().cloned());
                }
            }
        }
    }

    fn update_to_cli(&mut self) {
        self.linker_flavor_json = self.linker_flavor.to_cli_counterpart();
        self.lld_flavor_json = self.linker_flavor.lld_flavor();
        self.linker_is_gnu_json = self.linker_flavor.is_gnu();
        for (args, args_json) in [
            (&self.pre_link_args, &mut self.pre_link_args_json),
            (&self.late_link_args, &mut self.late_link_args_json),
            (&self.late_link_args_dynamic, &mut self.late_link_args_dynamic_json),
            (&self.late_link_args_static, &mut self.late_link_args_static_json),
            (&self.post_link_args, &mut self.post_link_args_json),
        ] {
            *args_json = args
                .iter()
                .map(|(flavor, args)| (flavor.to_cli_counterpart(), args.clone()))
                .collect();
        }
    }
}

impl Default for TargetOptions {
    /// Creates a set of "sane defaults" for any target. This is still
    /// incomplete, and if used for compilation, will certainly not work.
    fn default() -> TargetOptions {
        TargetOptions {
            endian: Endian::Little,
            c_int_width: 32,
            os: Os::None,
            env: Env::Unspecified,
            abi: Abi::Unspecified,
            vendor: "unknown".into(),
            linker: option_env!("CFG_DEFAULT_LINKER").map(|s| s.into()),
            linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
            linker_flavor_json: LinkerFlavorCli::Gcc,
            lld_flavor_json: LldFlavor::Ld,
            linker_is_gnu_json: true,
            link_script: None,
            asm_args: cvs![],
            cpu: "generic".into(),
            need_explicit_cpu: false,
            features: "".into(),
            direct_access_external_data: None,
            dynamic_linking: false,
            dll_tls_export: true,
            only_cdylib: false,
            executables: true,
            relocation_model: RelocModel::Pic,
            code_model: None,
            tls_model: TlsModel::GeneralDynamic,
            disable_redzone: false,
            frame_pointer: FramePointer::MayOmit,
            function_sections: true,
            dll_prefix: "lib".into(),
            dll_suffix: ".so".into(),
            exe_suffix: "".into(),
            staticlib_prefix: "lib".into(),
            staticlib_suffix: ".a".into(),
            families: cvs![],
            abi_return_struct_as_int: false,
            is_like_aix: false,
            is_like_darwin: false,
            is_like_gpu: false,
            is_like_solaris: false,
            is_like_windows: false,
            is_like_msvc: false,
            is_like_wasm: false,
            is_like_android: false,
            is_like_vexos: false,
            binary_format: BinaryFormat::Elf,
            default_dwarf_version: 4,
            allows_weak_linkage: true,
            has_rpath: false,
            no_default_libraries: true,
            position_independent_executables: false,
            static_position_independent_executables: false,
            plt_by_default: true,
            relro_level: RelroLevel::None,
            pre_link_objects: Default::default(),
            post_link_objects: Default::default(),
            pre_link_objects_self_contained: Default::default(),
            post_link_objects_self_contained: Default::default(),
            link_self_contained: LinkSelfContainedDefault::False,
            pre_link_args: LinkArgs::new(),
            pre_link_args_json: LinkArgsCli::new(),
            late_link_args: LinkArgs::new(),
            late_link_args_json: LinkArgsCli::new(),
            late_link_args_dynamic: LinkArgs::new(),
            late_link_args_dynamic_json: LinkArgsCli::new(),
            late_link_args_static: LinkArgs::new(),
            late_link_args_static_json: LinkArgsCli::new(),
            post_link_args: LinkArgs::new(),
            post_link_args_json: LinkArgsCli::new(),
            link_env: cvs![],
            link_env_remove: cvs![],
            archive_format: "gnu".into(),
            main_needs_argc_argv: true,
            allow_asm: true,
            has_thread_local: false,
            obj_is_bitcode: false,
            min_atomic_width: None,
            max_atomic_width: None,
            atomic_cas: true,
            panic_strategy: PanicStrategy::Unwind,
            crt_static_allows_dylibs: false,
            crt_static_default: false,
            crt_static_respected: false,
            stack_probes: StackProbeType::None,
            min_global_align: None,
            default_codegen_units: None,
            default_codegen_backend: None,
            trap_unreachable: true,
            requires_lto: false,
            singlethread: false,
            no_builtins: false,
            default_visibility: None,
            emit_debug_gdb_scripts: true,
            requires_uwtable: false,
            default_uwtable: false,
            simd_types_indirect: true,
            limit_rdylib_exports: true,
            override_export_symbols: None,
            merge_functions: MergeFunctions::Aliases,
            mcount: "mcount".into(),
            llvm_mcount_intrinsic: None,
            llvm_abiname: "".into(),
            llvm_floatabi: None,
            rustc_abi: None,
            relax_elf_relocations: false,
            llvm_args: cvs![],
            use_ctors_section: false,
            eh_frame_header: true,
            has_thumb_interworking: false,
            debuginfo_kind: Default::default(),
            split_debuginfo: Default::default(),
            // `Off` is supported by default, but targets can remove this manually, e.g. Windows.
            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
            supported_sanitizers: SanitizerSet::empty(),
            default_sanitizers: SanitizerSet::empty(),
            c_enum_min_bits: None,
            generate_arange_section: true,
            supports_stack_protector: true,
            entry_name: "main".into(),
            entry_abi: CanonAbi::C,
            supports_xray: false,
            default_address_space: rustc_abi::AddressSpace::ZERO,
            small_data_threshold_support: SmallDataThresholdSupport::DefaultForArch,
        }
    }
}

/// `TargetOptions` being a separate type is basically an implementation detail of `Target` that is
/// used for providing defaults. Perhaps there's a way to merge `TargetOptions` into `Target` so
/// this `Deref` implementation is no longer necessary.
impl Deref for Target {
    type Target = TargetOptions;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.options
    }
}
impl DerefMut for Target {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.options
    }
}

impl Target {
    pub fn is_abi_supported(&self, abi: ExternAbi) -> bool {
        let abi_map = AbiMap::from_target(self);
        abi_map.canonize_abi(abi, false).is_mapped()
    }

    /// Minimum integer size in bits that this target can perform atomic
    /// operations on.
    pub fn min_atomic_width(&self) -> u64 {
        self.min_atomic_width.unwrap_or(8)
    }

    /// Maximum integer size in bits that this target can perform atomic
    /// operations on.
    pub fn max_atomic_width(&self) -> u64 {
        self.max_atomic_width.unwrap_or_else(|| self.pointer_width.into())
    }

    /// Check some basic consistency of the current target. For JSON targets we are less strict;
    /// some of these checks are more guidelines than strict rules.
    fn check_consistency(&self, kind: TargetKind) -> Result<(), String> {
        macro_rules! check {
            ($b:expr, $($msg:tt)*) => {
                if !$b {
                    return Err(format!($($msg)*));
                }
            }
        }
        macro_rules! check_eq {
            ($left:expr, $right:expr, $($msg:tt)*) => {
                if ($left) != ($right) {
                    return Err(format!($($msg)*));
                }
            }
        }
        macro_rules! check_ne {
            ($left:expr, $right:expr, $($msg:tt)*) => {
                if ($left) == ($right) {
                    return Err(format!($($msg)*));
                }
            }
        }
        macro_rules! check_matches {
            ($left:expr, $right:pat, $($msg:tt)*) => {
                if !matches!($left, $right) {
                    return Err(format!($($msg)*));
                }
            }
        }

        check_eq!(
            self.is_like_darwin,
            self.vendor == "apple",
            "`is_like_darwin` must be set if and only if `vendor` is `apple`"
        );
        check_eq!(
            self.is_like_solaris,
            matches!(self.os, Os::Solaris | Os::Illumos),
            "`is_like_solaris` must be set if and only if `os` is `solaris` or `illumos`"
        );
        check_eq!(
            self.is_like_gpu,
            self.arch == Arch::Nvptx64 || self.arch == Arch::AmdGpu,
            "`is_like_gpu` must be set if and only if `target` is `nvptx64` or `amdgcn`"
        );
        check_eq!(
            self.is_like_windows,
            matches!(self.os, Os::Windows | Os::Uefi | Os::Cygwin),
            "`is_like_windows` must be set if and only if `os` is `windows`, `uefi` or `cygwin`"
        );
        check_eq!(
            self.is_like_wasm,
            matches!(self.arch, Arch::Wasm32 | Arch::Wasm64),
            "`is_like_wasm` must be set if and only if `arch` is `wasm32` or `wasm64`"
        );
        if self.is_like_msvc {
            check!(self.is_like_windows, "if `is_like_msvc` is set, `is_like_windows` must be set");
        }
        if self.os == Os::Emscripten {
            check!(self.is_like_wasm, "the `emcscripten` os only makes sense on wasm-like targets");
        }

        // Check that default linker flavor is compatible with some other key properties.
        check_eq!(
            self.is_like_darwin,
            matches!(self.linker_flavor, LinkerFlavor::Darwin(..)),
            "`linker_flavor` must be `darwin` if and only if `is_like_darwin` is set"
        );
        check_eq!(
            self.is_like_msvc,
            matches!(self.linker_flavor, LinkerFlavor::Msvc(..)),
            "`linker_flavor` must be `msvc` if and only if `is_like_msvc` is set"
        );
        check_eq!(
            self.is_like_wasm && self.os != Os::Emscripten,
            matches!(self.linker_flavor, LinkerFlavor::WasmLld(..)),
            "`linker_flavor` must be `wasm-lld` if and only if `is_like_wasm` is set and the `os` is not `emscripten`",
        );
        check_eq!(
            self.os == Os::Emscripten,
            matches!(self.linker_flavor, LinkerFlavor::EmCc),
            "`linker_flavor` must be `em-cc` if and only if `os` is `emscripten`"
        );
        check_eq!(
            self.arch == Arch::Bpf,
            matches!(self.linker_flavor, LinkerFlavor::Bpf),
            "`linker_flavor` must be `bpf` if and only if `arch` is `bpf`"
        );
        check_eq!(
            self.arch == Arch::Nvptx64,
            matches!(self.linker_flavor, LinkerFlavor::Ptx),
            "`linker_flavor` must be `ptc` if and only if `arch` is `nvptx64`"
        );

        for args in [
            &self.pre_link_args,
            &self.late_link_args,
            &self.late_link_args_dynamic,
            &self.late_link_args_static,
            &self.post_link_args,
        ] {
            for (&flavor, flavor_args) in args {
                check!(
                    !flavor_args.is_empty() || self.arch == Arch::Avr,
                    "linker flavor args must not be empty"
                );
                // Check that flavors mentioned in link args are compatible with the default flavor.
                match self.linker_flavor {
                    LinkerFlavor::Gnu(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::Gnu(..),
                            "mixing GNU and non-GNU linker flavors"
                        );
                    }
                    LinkerFlavor::Darwin(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::Darwin(..),
                            "mixing Darwin and non-Darwin linker flavors"
                        )
                    }
                    LinkerFlavor::WasmLld(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::WasmLld(..),
                            "mixing wasm and non-wasm linker flavors"
                        )
                    }
                    LinkerFlavor::Unix(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::Unix(..),
                            "mixing unix and non-unix linker flavors"
                        );
                    }
                    LinkerFlavor::Msvc(..) => {
                        check_matches!(
                            flavor,
                            LinkerFlavor::Msvc(..),
                            "mixing MSVC and non-MSVC linker flavors"
                        );
                    }
                    LinkerFlavor::EmCc
                    | LinkerFlavor::Bpf
                    | LinkerFlavor::Ptx
                    | LinkerFlavor::Llbc => {
                        check_eq!(flavor, self.linker_flavor, "mixing different linker flavors")
                    }
                }

                // Check that link args for cc and non-cc versions of flavors are consistent.
                let check_noncc = |noncc_flavor| -> Result<(), String> {
                    if let Some(noncc_args) = args.get(&noncc_flavor) {
                        for arg in flavor_args {
                            if let Some(suffix) = arg.strip_prefix("-Wl,") {
                                check!(
                                    noncc_args.iter().any(|a| a == suffix),
                                    " link args for cc and non-cc versions of flavors are not consistent"
                                );
                            }
                        }
                    }
                    Ok(())
                };

                match self.linker_flavor {
                    LinkerFlavor::Gnu(Cc::Yes, lld) => check_noncc(LinkerFlavor::Gnu(Cc::No, lld))?,
                    LinkerFlavor::WasmLld(Cc::Yes) => check_noncc(LinkerFlavor::WasmLld(Cc::No))?,
                    LinkerFlavor::Unix(Cc::Yes) => check_noncc(LinkerFlavor::Unix(Cc::No))?,
                    _ => {}
                }
            }

            // Check that link args for lld and non-lld versions of flavors are consistent.
            for cc in [Cc::No, Cc::Yes] {
                check_eq!(
                    args.get(&LinkerFlavor::Gnu(cc, Lld::No)),
                    args.get(&LinkerFlavor::Gnu(cc, Lld::Yes)),
                    "link args for lld and non-lld versions of flavors are not consistent",
                );
                check_eq!(
                    args.get(&LinkerFlavor::Darwin(cc, Lld::No)),
                    args.get(&LinkerFlavor::Darwin(cc, Lld::Yes)),
                    "link args for lld and non-lld versions of flavors are not consistent",
                );
            }
            check_eq!(
                args.get(&LinkerFlavor::Msvc(Lld::No)),
                args.get(&LinkerFlavor::Msvc(Lld::Yes)),
                "link args for lld and non-lld versions of flavors are not consistent",
            );
        }

        if self.link_self_contained.is_disabled() {
            check!(
                self.pre_link_objects_self_contained.is_empty()
                    && self.post_link_objects_self_contained.is_empty(),
                "if `link_self_contained` is disabled, then `pre_link_objects_self_contained` and `post_link_objects_self_contained` must be empty",
            );
        }

        // If your target really needs to deviate from the rules below,
        // except it and document the reasons.
        // Keep the default "unknown" vendor instead.
        check_ne!(self.vendor, "", "`vendor` cannot be empty");
        if let Os::Other(s) = &self.os {
            check!(!s.is_empty(), "`os` cannot be empty");
        }
        if !self.can_use_os_unknown() {
            // Keep the default "none" for bare metal targets instead.
            check_ne!(
                self.os,
                Os::Unknown,
                "`unknown` os can only be used on particular targets; use `none` for bare-metal targets"
            );
        }

        // Check dynamic linking stuff.
        // We skip this for JSON targets since otherwise, our default values would fail this test.
        // These checks are not critical for correctness, but more like default guidelines.
        // FIXME (https://github.com/rust-lang/rust/issues/133459): do we want to change the JSON
        // target defaults so that they pass these checks?
        if kind == TargetKind::Builtin {
            // BPF: when targeting user space vms (like rbpf), those can load dynamic libraries.
            // hexagon: when targeting QuRT, that OS can load dynamic libraries.
            // wasm{32,64}: dynamic linking is inherent in the definition of the VM.
            if self.os == Os::None
                && !matches!(self.arch, Arch::Bpf | Arch::Hexagon | Arch::Wasm32 | Arch::Wasm64)
            {
                check!(
                    !self.dynamic_linking,
                    "dynamic linking is not supported on this OS/architecture"
                );
            }
            if self.only_cdylib
                || self.crt_static_allows_dylibs
                || !self.late_link_args_dynamic.is_empty()
            {
                check!(
                    self.dynamic_linking,
                    "dynamic linking must be allowed when `only_cdylib` or `crt_static_allows_dylibs` or `late_link_args_dynamic` are set"
                );
            }
            // Apparently PIC was slow on wasm at some point, see comments in wasm_base.rs
            if self.dynamic_linking && !self.is_like_wasm {
                check_eq!(
                    self.relocation_model,
                    RelocModel::Pic,
                    "targets that support dynamic linking must use the `pic` relocation model"
                );
            }
            if self.position_independent_executables {
                check_eq!(
                    self.relocation_model,
                    RelocModel::Pic,
                    "targets that support position-independent executables must use the `pic` relocation model"
                );
            }
            // The UEFI targets do not support dynamic linking but still require PIC (#101377).
            if self.relocation_model == RelocModel::Pic && self.os != Os::Uefi {
                check!(
                    self.dynamic_linking || self.position_independent_executables,
                    "when the relocation model is `pic`, the target must support dynamic linking or use position-independent executables. \
                Set the relocation model to `static` to avoid this requirement"
                );
            }
            if self.static_position_independent_executables {
                check!(
                    self.position_independent_executables,
                    "if `static_position_independent_executables` is set, then `position_independent_executables` must be set"
                );
            }
            if self.position_independent_executables {
                check!(
                    self.executables,
                    "if `position_independent_executables` is set then `executables` must be set"
                );
            }
        }

        // Check crt static stuff
        if self.crt_static_default || self.crt_static_allows_dylibs {
            check!(
                self.crt_static_respected,
                "static CRT can be enabled but `crt_static_respected` is not set"
            );
        }

        // Check that RISC-V targets always specify which ABI they use,
        // and that ARM targets specify their float ABI.
        match self.arch {
            Arch::RiscV32 => {
                check_matches!(
                    &*self.llvm_abiname,
                    "ilp32" | "ilp32f" | "ilp32d" | "ilp32e",
                    "invalid RISC-V ABI name: {}",
                    self.llvm_abiname,
                );
            }
            Arch::RiscV64 => {
                // Note that the `lp64e` is still unstable as it's not (yet) part of the ELF psABI.
                check_matches!(
                    &*self.llvm_abiname,
                    "lp64" | "lp64f" | "lp64d" | "lp64e",
                    "invalid RISC-V ABI name: {}",
                    self.llvm_abiname,
                );
            }
            Arch::Arm => {
                check!(
                    self.llvm_floatabi.is_some(),
                    "ARM targets must set `llvm-floatabi` to `hard` or `soft`",
                )
            }
            _ => {}
        }

        // Check consistency of Rust ABI declaration.
        if let Some(rust_abi) = self.rustc_abi {
            match rust_abi {
                RustcAbi::X86Sse2 => check_matches!(
                    self.arch,
                    Arch::X86,
                    "`x86-sse2` ABI is only valid for x86-32 targets"
                ),
                RustcAbi::X86Softfloat => check_matches!(
                    self.arch,
                    Arch::X86 | Arch::X86_64,
                    "`x86-softfloat` ABI is only valid for x86 targets"
                ),
            }
        }

        // Check that the given target-features string makes some basic sense.
        if !self.features.is_empty() {
            let mut features_enabled = FxHashSet::default();
            let mut features_disabled = FxHashSet::default();
            for feat in self.features.split(',') {
                if let Some(feat) = feat.strip_prefix("+") {
                    features_enabled.insert(feat);
                    if features_disabled.contains(feat) {
                        return Err(format!(
                            "target feature `{feat}` is both enabled and disabled"
                        ));
                    }
                } else if let Some(feat) = feat.strip_prefix("-") {
                    features_disabled.insert(feat);
                    if features_enabled.contains(feat) {
                        return Err(format!(
                            "target feature `{feat}` is both enabled and disabled"
                        ));
                    }
                } else {
                    return Err(format!(
                        "target feature `{feat}` is invalid, must start with `+` or `-`"
                    ));
                }
            }
            // Check that we don't mis-set any of the ABI-relevant features.
            let abi_feature_constraints = self.abi_required_features();
            for feat in abi_feature_constraints.required {
                // The feature might be enabled by default so we can't *require* it to show up.
                // But it must not be *disabled*.
                if features_disabled.contains(feat) {
                    return Err(format!(
                        "target feature `{feat}` is required by the ABI but gets disabled in target spec"
                    ));
                }
            }
            for feat in abi_feature_constraints.incompatible {
                // The feature might be disabled by default so we can't *require* it to show up.
                // But it must not be *enabled*.
                if features_enabled.contains(feat) {
                    return Err(format!(
                        "target feature `{feat}` is incompatible with the ABI but gets enabled in target spec"
                    ));
                }
            }
        }

        Ok(())
    }

    /// Test target self-consistency and JSON encoding/decoding roundtrip.
    #[cfg(test)]
    fn test_target(mut self) {
        let recycled_target =
            Target::from_json(&serde_json::to_string(&self.to_json()).unwrap()).map(|(j, _)| j);
        self.update_to_cli();
        self.check_consistency(TargetKind::Builtin).unwrap();
        assert_eq!(recycled_target, Ok(self));
    }

    // Add your target to the whitelist if it has `std` library
    // and you certainly want "unknown" for the OS name.
    fn can_use_os_unknown(&self) -> bool {
        self.llvm_target == "wasm32-unknown-unknown"
            || self.llvm_target == "wasm64-unknown-unknown"
            || (self.env == Env::Sgx && self.vendor == "fortanix")
    }

    /// Load a built-in target
    pub fn expect_builtin(target_tuple: &TargetTuple) -> Target {
        match *target_tuple {
            TargetTuple::TargetTuple(ref target_tuple) => {
                load_builtin(target_tuple).expect("built-in target")
            }
            TargetTuple::TargetJson { .. } => {
                panic!("built-in targets doesn't support target-paths")
            }
        }
    }

    /// Load all built-in targets
    pub fn builtins() -> impl Iterator<Item = Target> {
        load_all_builtins()
    }

    /// Search for a JSON file specifying the given target tuple.
    ///
    /// If none is found in `$RUST_TARGET_PATH`, look for a file called `target.json` inside the
    /// sysroot under the target-tuple's `rustlib` directory. Note that it could also just be a
    /// bare filename already, so also check for that. If one of the hardcoded targets we know
    /// about, just return it directly.
    ///
    /// The error string could come from any of the APIs called, including filesystem access and
    /// JSON decoding.
    pub fn search(
        target_tuple: &TargetTuple,
        sysroot: &Path,
    ) -> Result<(Target, TargetWarnings), String> {
        use std::{env, fs};

        fn load_file(path: &Path) -> Result<(Target, TargetWarnings), String> {
            let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
            Target::from_json(&contents)
        }

        match *target_tuple {
            TargetTuple::TargetTuple(ref target_tuple) => {
                // check if tuple is in list of built-in targets
                if let Some(t) = load_builtin(target_tuple) {
                    return Ok((t, TargetWarnings::empty()));
                }

                // search for a file named `target_tuple`.json in RUST_TARGET_PATH
                let path = {
                    let mut target = target_tuple.to_string();
                    target.push_str(".json");
                    PathBuf::from(target)
                };

                let target_path = env::var_os("RUST_TARGET_PATH").unwrap_or_default();

                for dir in env::split_paths(&target_path) {
                    let p = dir.join(&path);
                    if p.is_file() {
                        return load_file(&p);
                    }
                }

                // Additionally look in the sysroot under `lib/rustlib/<tuple>/target.json`
                // as a fallback.
                let rustlib_path = crate::relative_target_rustlib_path(sysroot, target_tuple);
                let p = PathBuf::from_iter([
                    Path::new(sysroot),
                    Path::new(&rustlib_path),
                    Path::new("target.json"),
                ]);
                if p.is_file() {
                    return load_file(&p);
                }

                Err(format!("could not find specification for target {target_tuple:?}"))
            }
            TargetTuple::TargetJson { ref contents, .. } => Target::from_json(contents),
        }
    }

    /// Return the target's small data threshold support, converting
    /// `DefaultForArch` into a concrete value.
    pub fn small_data_threshold_support(&self) -> SmallDataThresholdSupport {
        match &self.options.small_data_threshold_support {
            // Avoid having to duplicate the small data support in every
            // target file by supporting a default value for each
            // architecture.
            SmallDataThresholdSupport::DefaultForArch => match self.arch {
                Arch::Mips | Arch::Mips64 | Arch::Mips32r6 => {
                    SmallDataThresholdSupport::LlvmArg("mips-ssection-threshold".into())
                }
                Arch::Hexagon => {
                    SmallDataThresholdSupport::LlvmArg("hexagon-small-data-threshold".into())
                }
                Arch::M68k => SmallDataThresholdSupport::LlvmArg("m68k-ssection-threshold".into()),
                Arch::RiscV32 | Arch::RiscV64 => {
                    SmallDataThresholdSupport::LlvmModuleFlag("SmallDataLimit".into())
                }
                _ => SmallDataThresholdSupport::None,
            },
            s => s.clone(),
        }
    }

    pub fn object_architecture(
        &self,
        unstable_target_features: &FxIndexSet<Symbol>,
    ) -> Option<(object::Architecture, Option<object::SubArchitecture>)> {
        use object::Architecture;
        Some(match self.arch {
            Arch::Arm => (Architecture::Arm, None),
            Arch::AArch64 => (
                if self.pointer_width == 32 {
                    Architecture::Aarch64_Ilp32
                } else {
                    Architecture::Aarch64
                },
                None,
            ),
            Arch::X86 => (Architecture::I386, None),
            Arch::S390x => (Architecture::S390x, None),
            Arch::M68k => (Architecture::M68k, None),
            Arch::Mips | Arch::Mips32r6 => (Architecture::Mips, None),
            Arch::Mips64 | Arch::Mips64r6 => (
                // While there are currently no builtin targets
                // using the N32 ABI, it is possible to specify
                // it using a custom target specification. N32
                // is an ILP32 ABI like the Aarch64_Ilp32
                // and X86_64_X32 cases above and below this one.
                if self.options.llvm_abiname.as_ref() == "n32" {
                    Architecture::Mips64_N32
                } else {
                    Architecture::Mips64
                },
                None,
            ),
            Arch::X86_64 => (
                if self.pointer_width == 32 {
                    Architecture::X86_64_X32
                } else {
                    Architecture::X86_64
                },
                None,
            ),
            Arch::PowerPC => (Architecture::PowerPc, None),
            Arch::PowerPC64 => (Architecture::PowerPc64, None),
            Arch::RiscV32 => (Architecture::Riscv32, None),
            Arch::RiscV64 => (Architecture::Riscv64, None),
            Arch::Sparc => {
                if unstable_target_features.contains(&sym::v8plus) {
                    // Target uses V8+, aka EM_SPARC32PLUS, aka 64-bit V9 but in 32-bit mode
                    (Architecture::Sparc32Plus, None)
                } else {
                    // Target uses V7 or V8, aka EM_SPARC
                    (Architecture::Sparc, None)
                }
            }
            Arch::Sparc64 => (Architecture::Sparc64, None),
            Arch::Avr => (Architecture::Avr, None),
            Arch::Msp430 => (Architecture::Msp430, None),
            Arch::Hexagon => (Architecture::Hexagon, None),
            Arch::Xtensa => (Architecture::Xtensa, None),
            Arch::Bpf => (Architecture::Bpf, None),
            Arch::LoongArch32 => (Architecture::LoongArch32, None),
            Arch::LoongArch64 => (Architecture::LoongArch64, None),
            Arch::CSky => (Architecture::Csky, None),
            Arch::Arm64EC => (Architecture::Aarch64, Some(object::SubArchitecture::Arm64EC)),
            Arch::AmdGpu
            | Arch::Nvptx64
            | Arch::PowerPC64LE
            | Arch::SpirV
            | Arch::Wasm32
            | Arch::Wasm64
            | Arch::Other(_) => return None,
        })
    }

    /// Returns whether this target is known to have unreliable alignment:
    /// native C code for the target fails to align some data to the degree
    /// required by the C standard. We can't *really* do anything about that
    /// since unsafe Rust code may assume alignment any time, but we can at least
    /// inhibit some optimizations, and we suppress the alignment checks that
    /// would detect this unsoundness.
    ///
    /// Every target that returns less than `Align::MAX` here is still has a soundness bug.
    pub fn max_reliable_alignment(&self) -> Align {
        // FIXME(#112480) MSVC on x86-32 is unsound and fails to properly align many types with
        // more-than-4-byte-alignment on the stack. This makes alignments larger than 4 generally
        // unreliable on 32bit Windows.
        if self.is_like_windows && self.arch == Arch::X86 {
            Align::from_bytes(4).unwrap()
        } else {
            Align::MAX
        }
    }

    pub fn vendor_symbol(&self) -> Symbol {
        Symbol::intern(&self.vendor)
    }
}
