use std::borrow::Cow;
use schemars::JsonSchema;
use serde_derive;

bitflags::bitflags! {
    /// The `-C linker-features` components that can individually be enabled or disabled.
    ///
    /// They are feature flags intended to be a more flexible mechanism than linker flavors, and
    /// also to prevent a combinatorial explosion of flavors whenever a new linker feature is
    /// required. These flags are "generic", in the sense that they can work on multiple targets on
    /// the CLI. Otherwise, one would have to select different linkers flavors for each target.
    ///
    /// Here are some examples of the advantages they offer:
    /// - default feature sets for principal flavors, or for specific targets.
    /// - flavor-specific features: for example, clang offers automatic cross-linking with
    ///   `--target`, which gcc-style compilers don't support. The *flavor* is still a C/C++
    ///   compiler, and we don't need to multiply the number of flavors for this use-case. Instead,
    ///   we can have a single `+target` feature.
    /// - umbrella features: for example if clang accumulates more features in the future than just
    ///   the `+target` above. That could be modeled as `+clang`.
    /// - niche features for resolving specific issues: for example, on Apple targets the linker
    ///   flag implementing the `as-needed` native link modifier (#99424) is only possible on
    ///   sufficiently recent linker versions.
    /// - still allows for discovery and automation, for example via feature detection. This can be
    ///   useful in exotic environments/build systems.
    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct LinkerFeatures: u8 {
        /// Invoke the linker via a C/C++ compiler (e.g. on most unix targets).
        const CC  = 1 << 0;
        /// Use the lld linker, either the system lld or the self-contained linker `rust-lld`.
        const LLD = 1 << 1;
    }
}
rustc_data_structures::external_bitflags_debug! { LinkerFeatures }

impl LinkerFeatures {
    /// Parses a single `-C linker-features` well-known feature, not a set of flags.
    pub fn from_str(s: &str) -> Option<LinkerFeatures> {
        Some(match s {
            "cc" => LinkerFeatures::CC,
            "lld" => LinkerFeatures::LLD,
            _ => return None,
        })
    }

    /// Return the linker feature name, as would be passed on the CLI.
    ///
    /// Returns `None` if the bitflags aren't a singular component (but a mix of multiple flags).
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            LinkerFeatures::CC => "cc",
            LinkerFeatures::LLD => "lld",
            _ => return None,
        })
    }

    /// Returns whether the `lld` linker feature is enabled.
    pub fn is_lld_enabled(self) -> bool {
        self.contains(LinkerFeatures::LLD)
    }

    /// Returns whether the `cc` linker feature is enabled.
    pub fn is_cc_enabled(self) -> bool {
        self.contains(LinkerFeatures::CC)
    }
}
