use crate::json::{Json, ToJson};
use std::collections::BTreeMap;
use std::str::FromStr;
use std::borrow::Cow;
use schemars::JsonSchema;
use serde_derive;

/// The different `-Clink-self-contained` options that can be specified in a target spec:
/// - enabling or disabling in bulk
/// - some target-specific pieces of inference to determine whether to use self-contained linking
///   if `-Clink-self-contained` is not specified explicitly (e.g. on musl/mingw)
/// - explicitly enabling some of the self-contained linking components, e.g. the linker component
///   to use `rust-lld`
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LinkSelfContainedDefault {
    /// The target spec explicitly enables self-contained linking.
    True,

    /// The target spec explicitly disables self-contained linking.
    False,

    /// The target spec requests that the self-contained mode is inferred, in the context of musl.
    InferredForMusl,

    /// The target spec requests that the self-contained mode is inferred, in the context of mingw.
    InferredForMingw,

    /// The target spec explicitly enables a list of self-contained linking components: e.g. for
    /// targets opting into a subset of components like the CLI's `-C link-self-contained=+linker`.
    WithComponents(LinkSelfContainedComponents),
}

/// Parses a backwards-compatible `-Clink-self-contained` option string, without components.
impl FromStr for LinkSelfContainedDefault {
    type Err = String;

    fn from_str(s: &str) -> Result<LinkSelfContainedDefault, Self::Err> {
        Ok(match s {
            "false" => LinkSelfContainedDefault::False,
            "true" | "wasm" => LinkSelfContainedDefault::True,
            "musl" => LinkSelfContainedDefault::InferredForMusl,
            "mingw" => LinkSelfContainedDefault::InferredForMingw,
            _ => {
                return Err(format!(
                    r"'{s}' is not a valid `-Clink-self-contained` default. Use 'false', 'true', 'wasm', 'musl' or 'mingw'",
                ));
            }
        })
    }
}

crate::json::serde_deserialize_from_str!(LinkSelfContainedDefault);
impl schemars::JsonSchema for LinkSelfContainedDefault {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "LinkSelfContainedDefault".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema! ({ 
            "type": "string",
            "enum": ["false", "true", "wasm", "musl", "mingw"]
        })
        .into()
    }
}

impl ToJson for LinkSelfContainedDefault {
    fn to_json(&self) -> Json {
        match *self {
            LinkSelfContainedDefault::WithComponents(components) => {
                // Serialize the components in a json object's `components` field, to prepare for a
                // future where `crt-objects-fallback` is removed from the json specs and
                // incorporated as a field here.
                let mut map = BTreeMap::new();
                map.insert("components", components);
                map.to_json()
            }

            // Stable backwards-compatible values
            LinkSelfContainedDefault::True => "true".to_json(),
            LinkSelfContainedDefault::False => "false".to_json(),
            LinkSelfContainedDefault::InferredForMusl => "musl".to_json(),
            LinkSelfContainedDefault::InferredForMingw => "mingw".to_json(),
        }
    }
}

impl LinkSelfContainedDefault {
    /// Returns whether the target spec has self-contained linking explicitly disabled. Used to emit
    /// errors if the user then enables it on the CLI.
    pub fn is_disabled(self) -> bool {
        self == LinkSelfContainedDefault::False
    }

    /// Returns the key to use when serializing the setting to json:
    /// - individual components in a `link-self-contained` object value
    /// - the other variants as a backwards-compatible `crt-objects-fallback` string
    fn json_key(self) -> &'static str {
        match self {
            LinkSelfContainedDefault::WithComponents(_) => "link-self-contained",
            _ => "crt-objects-fallback",
        }
    }

    /// Creates a `LinkSelfContainedDefault` enabling the self-contained linker for target specs
    /// (the equivalent of `-Clink-self-contained=+linker` on the CLI).
    pub fn with_linker() -> LinkSelfContainedDefault {
        LinkSelfContainedDefault::WithComponents(LinkSelfContainedComponents::LINKER)
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    /// The `-C link-self-contained` components that can individually be enabled or disabled.
    pub struct LinkSelfContainedComponents: u8 {
        /// CRT objects (e.g. on `windows-gnu`, `musl`, `wasi` targets)
        const CRT_OBJECTS = 1 << 0;
        /// libc static library (e.g. on `musl`, `wasi` targets)
        const LIBC        = 1 << 1;
        /// libgcc/libunwind (e.g. on `windows-gnu`, `fuchsia`, `fortanix`, `gnullvm` targets)
        const UNWIND      = 1 << 2;
        /// Linker, dlltool, and their necessary libraries (e.g. on `windows-gnu` and for `rust-lld`)
        const LINKER      = 1 << 3;
        /// Sanitizer runtime libraries
        const SANITIZERS  = 1 << 4;
        /// Other MinGW libs and Windows import libs
        const MINGW       = 1 << 5;
    }
}
rustc_data_structures::external_bitflags_debug! { LinkSelfContainedComponents }

impl LinkSelfContainedComponents {
    /// Return the component's name.
    ///
    /// Returns `None` if the bitflags aren't a singular component (but a mix of multiple flags).
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            LinkSelfContainedComponents::CRT_OBJECTS => "crto",
            LinkSelfContainedComponents::LIBC => "libc",
            LinkSelfContainedComponents::UNWIND => "unwind",
            LinkSelfContainedComponents::LINKER => "linker",
            LinkSelfContainedComponents::SANITIZERS => "sanitizers",
            LinkSelfContainedComponents::MINGW => "mingw",
            _ => return None,
        })
    }

    /// Returns an array of all the components.
    fn all_components() -> [LinkSelfContainedComponents; 6] {
        [
            LinkSelfContainedComponents::CRT_OBJECTS,
            LinkSelfContainedComponents::LIBC,
            LinkSelfContainedComponents::UNWIND,
            LinkSelfContainedComponents::LINKER,
            LinkSelfContainedComponents::SANITIZERS,
            LinkSelfContainedComponents::MINGW,
        ]
    }

    /// Returns whether at least a component is enabled.
    pub fn are_any_components_enabled(self) -> bool {
        !self.is_empty()
    }

    /// Returns whether `LinkSelfContainedComponents::LINKER` is enabled.
    pub fn is_linker_enabled(self) -> bool {
        self.contains(LinkSelfContainedComponents::LINKER)
    }

    /// Returns whether `LinkSelfContainedComponents::CRT_OBJECTS` is enabled.
    pub fn is_crt_objects_enabled(self) -> bool {
        self.contains(LinkSelfContainedComponents::CRT_OBJECTS)
    }
}

impl FromStr for LinkSelfContainedComponents {
    type Err = String;

    /// Parses a single `-Clink-self-contained` well-known component, not a set of flags.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "crto" => LinkSelfContainedComponents::CRT_OBJECTS,
            "libc" => LinkSelfContainedComponents::LIBC,
            "unwind" => LinkSelfContainedComponents::UNWIND,
            "linker" => LinkSelfContainedComponents::LINKER,
            "sanitizers" => LinkSelfContainedComponents::SANITIZERS,
            "mingw" => LinkSelfContainedComponents::MINGW,
            _ => {
                return Err(format!(
                    "'{s}' is not a valid link-self-contained component, expected 'crto', 'libc', 'unwind', 'linker', 'sanitizers', 'mingw'"
                ));
            }
        })
    }
}

crate::json::serde_deserialize_from_str!(LinkSelfContainedComponents);
impl schemars::JsonSchema for LinkSelfContainedComponents {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "LinkSelfContainedComponents".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let all = 
            Self::all_components().iter().map(|component| component.as_str()).collect::<Vec<_>>();
        schemars::json_schema! ({ 
            "type": "string",
            "enum": all,
        })
        .into()
    }
}

impl ToJson for LinkSelfContainedComponents {
    fn to_json(&self) -> Json {
        let components: Vec<_> = Self::all_components()
            .into_iter()
            .filter(|c| self.contains(*c))
            .map(|c| {
                // We can unwrap because we're iterating over all the known singular components,
                // not an actual set of flags where `as_str` can fail.
                c.as_str().unwrap().to_owned()
            })
            .collect();

        components.to_json()
    }
}
