use core::result::Result;
use std::borrow::Cow;
use std::collections::BTreeMap; // Add this, as LinkerFlavor uses BTreeMap
use std::str::FromStr;

use rustc_span::{Symbol, kw, sym}; // Add kw and sym here
use crate::json::{Json, ToJson};
use crate::target_spec_enum;


/// Linker is called through a C/C++ compiler.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Cc {
    Yes,
    No,
}

/// Linker is LLD.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Lld {
    Yes,
    No,
}

/// All linkers have some kinds of command line interfaces and rustc needs to know which commands
/// to use with each of them. So we cluster all such interfaces into a (somewhat arbitrary) number
/// of classes that we call "linker flavors".
///
/// Technically, it's not even necessary, we can nearly always infer the flavor from linker name
/// and target properties like `is_like_windows`/`is_like_darwin`/etc. However, the PRs originally
/// introducing `-Clinker-flavor` (#40018 and friends) were aiming to reduce this kind of inference
/// and provide something certain and explicitly specified instead, and that design goal is still
/// relevant now.
///
/// The second goal is to keep the number of flavors to the minimum if possible.
/// LLD somewhat forces our hand here because that linker is self-sufficient only if its executable
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
    /// possibly with non-GNU extensions (both naked and compiler-wrapped forms).
    /// LLD doesn't support any of these.
    Unix(Cc),
    /// MSVC-style linker for Windows and UEFI, LLD supports it.
    Msvc(Lld),
    /// Emscripten Compiler Frontend, a wrapper around `WasmLld(Cc::Yes)` that has a different
    /// interface and produces some additional JavaScript output.
    EmCc,
    // Below: other linker-like tools with unique interfaces for exotic targets.
    /// Linker tool for BPF.
    Bpf,
    /// Linker tool for Nvidia PTX.
    Ptx,
    /// LLVM bitcode linker that can be used as a `self-contained` linker
    Llbc,
}

/// Linker flavors available externally through command line (`-Clinker-flavor`)
/// or json target specifications.
/// This set has accumulated historically, and contains both (stable and unstable) legacy values, as
/// well as modern ones matching the internal linker flavors (`LinkerFlavor`).
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LinkerFlavorCli {
    // Modern (unstable) flavors, with direct counterparts in `LinkerFlavor`.
    Gnu(Cc, Lld),
    Darwin(Cc, Lld),
    WasmLld(Cc),
    Unix(Cc),
    // Note: `Msvc(Lld::No)` is also a stable value.
    Msvc(Lld),
    EmCc,
    Bpf,
    Ptx,
    Llbc,

    // Legacy stable values
    Gcc,
    Ld,
    Lld(LldFlavor),
    Em,
}

impl LinkerFlavorCli {
    /// Returns whether this `-C linker-flavor` option is one of the unstable values.
    pub fn is_unstable(&self) -> bool {
        match self {
            LinkerFlavorCli::Gnu(..)
            | LinkerFlavorCli::Darwin(..)
            | LinkerFlavorCli::WasmLld(..)
            | LinkerFlavorCli::Unix(..)
            | LinkerFlavorCli::Msvc(Lld::Yes)
            | LinkerFlavorCli::EmCc
            | LinkerFlavorCli::Bpf
            | LinkerFlavorCli::Llbc
            | LinkerFlavorCli::Ptx => true,
            LinkerFlavorCli::Gcc
            | LinkerFlavorCli::Ld
            | LinkerFlavorCli::Lld(..)
            | LinkerFlavorCli::Msvc(Lld::No)
            | LinkerFlavorCli::Em => false,
        }
    }
}

crate::target_spec_enum! {
    pub enum LldFlavor {
        Wasm = "wasm",
        Ld64 = "darwin",
        Ld = "gnu",
        Link = "link",
    }

    parse_error_type = "LLD flavor";
}

impl crate::json::ToJson for LldFlavor {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}

impl LinkerFlavor {
    /// At this point the target's reference linker flavor doesn't yet exist and we need to infer
    /// it. The inference always succeeds and gives some result, and we don't report any flavor
    /// incompatibility errors for json target specs. The CLI flavor is used as the main source
    /// of truth, other flags are used in case of ambiguities.
    pub fn from_cli_json(cli: LinkerFlavorCli, lld_flavor: LldFlavor, is_gnu: bool) -> LinkerFlavor {
        match cli {
            LinkerFlavorCli::Gnu(cc, lld) => LinkerFlavor::Gnu(cc, lld),
            LinkerFlavorCli::Darwin(cc, lld) => LinkerFlavor::Darwin(cc, lld),
            LinkerFlavorCli::WasmLld(cc) => LinkerFlavor::WasmLld(cc),
            LinkerFlavorCli::Unix(cc) => LinkerFlavor::Unix(cc),
            LinkerFlavorCli::Msvc(lld) => LinkerFlavor::Msvc(lld),
            LinkerFlavorCli::EmCc => LinkerFlavor::EmCc,
            LinkerFlavorCli::Bpf => LinkerFlavor::Bpf,
            LinkerFlavorCli::Llbc => LinkerFlavor::Llbc,
            LinkerFlavorCli::Ptx => LinkerFlavor::Ptx,

            // Below: legacy stable values
            LinkerFlavorCli::Gcc => match lld_flavor {
                LldFlavor::Ld if is_gnu => LinkerFlavor::Gnu(Cc::Yes, Lld::No),
                LldFlavor::Ld64 => LinkerFlavor::Darwin(Cc::Yes, Lld::No),
                LldFlavor::Wasm => LinkerFlavor::WasmLld(Cc::Yes),
                LldFlavor::Ld | LldFlavor::Link => LinkerFlavor::Unix(Cc::Yes),
            },
            LinkerFlavorCli::Ld => match lld_flavor {
                LldFlavor::Ld if is_gnu => LinkerFlavor::Gnu(Cc::No, Lld::No),
                LldFlavor::Ld64 => LinkerFlavor::Darwin(Cc::No, Lld::No),
                LldFlavor::Ld | LldFlavor::Wasm | LldFlavor::Link => LinkerFlavor::Unix(Cc::No),
            },
            LinkerFlavorCli::Lld(LldFlavor::Ld) => LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            LinkerFlavorCli::Lld(LldFlavor::Ld64) => LinkerFlavor::Darwin(Cc::No, Lld::Yes),
            LinkerFlavorCli::Lld(LldFlavor::Wasm) => LinkerFlavor::WasmLld(Cc::No),
            LinkerFlavorCli::Lld(LldFlavor::Link) => LinkerFlavor::Msvc(Lld::Yes),
            LinkerFlavorCli::Em => LinkerFlavor::EmCc,
        }
    }

    /// Returns the corresponding backwards-compatible CLI flavor.
    fn to_cli(self) -> LinkerFlavorCli {
        match self {
            LinkerFlavor::Gnu(Cc::Yes, _)
            | LinkerFlavor::Darwin(Cc::Yes, _)
            | LinkerFlavor::WasmLld(Cc::Yes)
            | LinkerFlavor::Unix(Cc::Yes) => LinkerFlavorCli::Gcc,
            LinkerFlavor::Gnu(_, Lld::Yes) => LinkerFlavorCli::Lld(LldFlavor::Ld),
            LinkerFlavor::Darwin(_, Lld::Yes) => LinkerFlavorCli::Lld(LldFlavor::Ld64),
            LinkerFlavor::WasmLld(..) => LinkerFlavorCli::Lld(LldFlavor::Wasm),
            LinkerFlavor::Gnu(..) | LinkerFlavor::Darwin(..) | LinkerFlavor::Unix(..) => {
                LinkerFlavorCli::Ld
            }
            LinkerFlavor::Msvc(Lld::Yes) => LinkerFlavorCli::Lld(LldFlavor::Link),
            LinkerFlavor::Msvc(..) => LinkerFlavorCli::Msvc(Lld::No),
            LinkerFlavor::EmCc => LinkerFlavorCli::Em,
            LinkerFlavor::Bpf => LinkerFlavorCli::Bpf,
            LinkerFlavor::Llbc => LinkerFlavorCli::Llbc,
            LinkerFlavor::Ptx => LinkerFlavorCli::Ptx,
        }
    }

    /// Returns the modern CLI flavor that is the counterpart of this flavor.
    pub fn to_cli_counterpart(self) -> LinkerFlavorCli {
        match self {
            LinkerFlavor::Gnu(cc, lld) => LinkerFlavorCli::Gnu(cc, lld),
            LinkerFlavor::Darwin(cc, lld) => LinkerFlavorCli::Darwin(cc, lld),
            LinkerFlavor::WasmLld(cc) => LinkerFlavorCli::WasmLld(cc),
            LinkerFlavor::Unix(cc) => LinkerFlavorCli::Unix(cc),
            LinkerFlavor::Msvc(lld) => LinkerFlavorCli::Msvc(lld),
            LinkerFlavor::EmCc => LinkerFlavorCli::EmCc,
            LinkerFlavor::Bpf => LinkerFlavorCli::Bpf,
            LinkerFlavor::Llbc => LinkerFlavorCli::Llbc,
            LinkerFlavor::Ptx => LinkerFlavorCli::Ptx,
        }
    }

    fn infer_cli_hints(cli: LinkerFlavorCli) -> (Option<Cc>, Option<Lld>) {
        match cli {
            LinkerFlavorCli::Gnu(cc, lld) | LinkerFlavorCli::Darwin(cc, lld) => {
                (Some(cc), Some(lld))
            }
            LinkerFlavorCli::WasmLld(cc) => (Some(cc), Some(Lld::Yes)),
            LinkerFlavorCli::Unix(cc) => (Some(cc), None),
            LinkerFlavorCli::Msvc(lld) => (Some(Cc::No), Some(lld)),
            LinkerFlavorCli::EmCc => (Some(Cc::Yes), Some(Lld::Yes)),
            LinkerFlavorCli::Bpf | LinkerFlavorCli::Ptx => (None, None),
            LinkerFlavorCli::Llbc => (None, None),

            // Below: legacy stable values
            LinkerFlavorCli::Gcc => (Some(Cc::Yes), None),
            LinkerFlavorCli::Ld => (Some(Cc::No), Some(Lld::No)),
            LinkerFlavorCli::Lld(_) => (Some(Cc::No), Some(Lld::Yes)),
            LinkerFlavorCli::Em => (Some(Cc::Yes), Some(Lld::Yes)),
        }
    }

    fn infer_linker_hints(linker_stem: &str) -> Result<Self, (Option<Cc>, Option<Lld>)> {
        // Remove any version postfix.
        let stem = linker_stem
            .rsplit_once('-')
            .and_then(|(lhs, rhs)| rhs.chars().all(char::is_numeric).then_some(lhs))
            .unwrap_or(linker_stem);

        if stem == "llvm-bitcode-linker" {
            Ok(Self::Llbc)
        } else if stem == "emcc" // GCC/Clang can have an optional target prefix.
            || stem == "gcc"
            || stem.ends_with("-gcc")
            || stem == "g++"
            || stem.ends_with("-g++")
            || stem == "clang"
            || stem.ends_with("-clang")
            || stem == "clang++"
            || stem.ends_with("-clang++")
        {
            Err((Some(Cc::Yes), Some(Lld::No)))
        } else if stem == "wasm-ld"
            || stem.ends_with("-wasm-ld")
            || stem == "ld.lld"
            || stem == "lld"
            || stem == "rust-lld"
            || stem == "lld-link"
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


pub type StaticCow<T> = Cow<'static, T>;

pub type LinkArgs = BTreeMap<LinkerFlavor, Vec<StaticCow<str>>>;
pub type LinkArgsCli = BTreeMap<LinkerFlavorCli, Vec<StaticCow<str>>>;

pub fn add_link_args_iter(
    link_args: &mut LinkArgs,
    flavor: LinkerFlavor,
    args: impl Iterator<Item = StaticCow<str>> + Clone,
) {
    let mut insert = |flavor| link_args.entry(flavor).or_default().extend(args.clone());
    insert(flavor);
    match flavor {
        LinkerFlavor::Gnu(cc, Lld::No) => {
            insert(LinkerFlavor::Gnu(cc, Lld::Yes));
        }
        LinkerFlavor::Darwin(cc, Lld::No) => {
            insert(LinkerFlavor::Darwin(cc, Lld::Yes));
        }
        LinkerFlavor::Msvc(Lld::No) => {
            insert(LinkerFlavor::Msvc(Lld::Yes));
        }
        LinkerFlavor::WasmLld(..)
        | LinkerFlavor::Unix(..)
        | LinkerFlavor::EmCc
        | LinkerFlavor::Bpf
        | LinkerFlavor::Llbc
        | LinkerFlavor::Ptx => {}
        _ => {} // Catch-all for other LinkerFlavor variants
    }
}

pub fn add_link_args(link_args: &mut LinkArgs, flavor: LinkerFlavor, args: &[&'static str]) {
    add_link_args_iter(link_args, flavor, args.iter().copied().map(Cow::Borrowed))
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