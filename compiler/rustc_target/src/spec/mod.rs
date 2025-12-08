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
//#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
//pub enum Cc {

// (`argv[0]`) is named in specific way, otherwise it doesn't work and requires a
// `-flavor LLD_FLAVOR` argument to choose which logic to use. Our shipped `rust-lld` in
// particular is not named in such specific way, so it needs the flavor option, so we make our
// linker flavors sufficiently fine-grained to satisfy LLD without inferring its flavor from other
// target properties, in accordance with the first design goal.
//
// The first component of the flavor is tightly coupled with the compilation target,
// while the `Cc` and `Lld` flags can vary within the same target.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LinkerFlavor {
    // Unix-like linker with GNU extensions (both naked and compiler-wrapped forms).
    // Besides similar "default" Linux/BSD linkers this also includes Windows/GNU linker,
    // which is somewhat different because it doesn't produce ELFs.
    Gnu(Cc, Lld),
    // Unix-like linker for Apple targets (both naked and compiler-wrapped forms).
    // Extracted from the "umbrella" `Unix` flavor due to its corresponding LLD flavor.
    Darwin(Cc, Lld),
    // Unix-like linker for Wasm targets (both naked and compiler-wrapped forms).
    // Extracted from the "umbrella" `Unix` flavor due to its corresponding LLD flavor.
    // Non-LLD version does not exist, so the lld flag is currently hardcoded here.
    WasmLld(Cc),
    // Basic Unix-like linker for "any other Unix" targets (Solaris/illumos, L4Re, MSP430, etc),
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

        // List of supported targets
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

mod target_warnings;
pub use target_warnings::*;

// For the [`Target::check_consistency`] function, determines whether the given target is a builtin or a JSON
// target.
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
    pub enum Abi {
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
        other_variant = "Other"
    }
}


// Every field here must be specified, and has no default value.
#[derive(PartialEq, Clone, Debug)]
pub struct Target {
    // Unversioned target tuple to pass to LLVM.
    //
    // Target tuples can optionally contain an OS version (notably Apple targets), which rustc
    // cannot know without querying the environment.
    //
    // Use `rustc_codegen_ssa::back::versioned_llvm_target` if you need the full LLVM target.
    pub llvm_target: StaticCow<str>,
    // Metadata about a target, for example the description or tier.
    // Used for generating target documentation.
    pub metadata: TargetMetadata,
    // Number of bits in a pointer. Influences the `target_pointer_width` `cfg` variable.
    pub pointer_width: u16,
    // Architecture to use for ABI considerations. Valid options include: "x86",
    // "x86_64", "arm", "aarch64", "mips", "powerpc", "powerpc64", and others.
    pub arch: Arch,
    // [Data layout](https://llvm.org/docs/LangRef.html#data-layout) to pass to LLVM.
    pub data_layout: StaticCow<str>,
    // Optional settings with defaults.
    pub options: TargetOptions,
}

// Metadata about a target like the description or tier.
// Part of #120745.
// All fields are optional for now, but intended to be required in the future.
#[derive(Default, PartialEq, Clone, Debug)]
pub struct TargetMetadata {
    // A short description of the target including platform requirements,
    // for example "64-bit Linux (kernel 3.2+, glibc 2.17+)".
    pub description: Option<StaticCow<str>>,
    // The tier of the target. 1, 2 or 3.
    pub tier: Option<u64>,
    // Whether the Rust project ships host tools for a target.
    pub host_tools: Option<bool>,
    // Whether a target has the `std` library. This is usually true for targets running
    // on an operating system.
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

// x86 (32-bit) abi options.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct X86Abi {
    // On x86-32 targets, the regparm N causes the compiler to pass arguments
    // in registers EAX, EDX, and ECX instead of on the stack.
    pub regparm: Option<u32>,
    // Override the default ABI to return small structs in registers
    pub reg_struct_return: bool,
}

pub trait HasX86AbiOpt {
    fn x86_abi_opt(&self) -> X86Abi;
}

type StaticCow<T> = Cow<'static, T>;


// Add arguments for the given flavor and also for its "twin" flavors
// that have a compatible command line interface.
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


