use crate::json::{Json, ToJson};
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use std::borrow::Cow;
use std::str::FromStr;
use serde_derive;
use serde_json::Value;
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum SmallDataThresholdSupport {
    None,
    DefaultForArch,
    LlvmModuleFlag(Cow<'static, str>),
    LlvmArg(Cow<'static, str>),
}

impl FromStr for SmallDataThresholdSupport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "none" {
            Ok(Self::None)
        } else if s == "default-for-arch" {
            Ok(Self::DefaultForArch)
        } else if let Some(flag) = s.strip_prefix("llvm-module-flag=") {
            Ok(Self::LlvmModuleFlag(flag.to_string().into()))
        } else if let Some(arg) = s.strip_prefix("llvm-arg=") {
            Ok(Self::LlvmArg(arg.to_string().into()))
        } else {
            Err(format!("'{s}' is not a valid value for small-data-threshold-support."))
        }
    }
}

crate::json::serde_deserialize_from_str!(SmallDataThresholdSupport);
impl schemars::JsonSchema for SmallDataThresholdSupport {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "SmallDataThresholdSupport".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema! ({
            "type": "string",
            "pattern": r#"^none|default-for-arch|llvm-module-flag=.+|llvm-arg=.+$"#,
        })
        .into()
    }
}

impl ToJson for SmallDataThresholdSupport {
    fn to_json(&self) -> Value {
        match self {
            Self::None => "none".to_json(),
            Self::DefaultForArch => "default-for-arch".to_json(),
            Self::LlvmModuleFlag(flag) => format!("llvm-module-flag={flag}").to_json(),
            Self::LlvmArg(arg) => format!("llvm-arg={arg}").to_json(),
        }
    }
}