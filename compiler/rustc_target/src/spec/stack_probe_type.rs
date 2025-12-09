use crate::json::{Json, ToJson};
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use serde_derive;
use serde_json::Value;
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Eq, serde_derive::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind")]
#[serde(rename_all = "kebab-case")]
pub enum StackProbeType {
    /// Don't emit any stack probes.
    None,
    /// It is harmless to use this option even on targets that do not have backend support for
    /// stack probes as the failure mode is the same as if no stack-probe option was specified in
    /// the first place.
    Inline,
    /// Call `__rust_probestack` whenever stack needs to be probed.
    Call,
    /// Use inline option for LLVM versions later than specified in `min_llvm_version_for_inline`
    /// and call `__rust_probestack` otherwise.
    InlineOrCall {
        #[serde(rename = "min-llvm-version-for-inline")]
        min_llvm_version_for_inline: (u32, u32, u32),
    },
}

impl ToJson for StackProbeType {
    fn to_json(&self) -> Json {
        Json::Object(match self {
            StackProbeType::None => {
                [(String::from("kind"), "none".to_json())].into_iter().collect()
            }
            StackProbeType::Inline => {
                [(String::from("kind"), "inline".to_json())].into_iter().collect()
            }
            StackProbeType::Call => {
                [(String::from("kind"), "call".to_json())].into_iter().collect()
            }
            StackProbeType::InlineOrCall { min_llvm_version_for_inline: (maj, min, patch) } => [
                (String::from("kind"), "inline-or-call".to_json()),
                (
                    String::from("min-llvm-version-for-inline"),
                    Json::Array(vec![maj.to_json(), min.to_json(), patch.to_json()]),
                ),
            ]
            .into_iter()
            .collect(),
        })
    }
}
