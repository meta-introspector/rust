use std::str::FromStr;
use crate::json::ToJson;
use rustc_macros::{Decodable, Encodable, HashStable_Generic};
use rustc_span::{Symbol, kw, sym};

crate::target_spec_enum! {
    pub enum RelocModel {
        Static = "static",
        Pic = "pic",
        Pie = "pie",
        DynamicNoPic = "dynamic-no-pic",
        Ropi = "ropi",
        Rwpi = "rwpi",
        RopiRwpi = "ropi-rwpi",
    }

    parse_error_type = "relocation model";
}

impl RelocModel {
    pub const fn desc_symbol(&self) -> Symbol {
        match *self {
            RelocModel::Static => kw::Static,
            RelocModel::Pic => sym::pic,
            RelocModel::Pie => sym::pie,
            RelocModel::DynamicNoPic => sym::dynamic_no_pic,
            RelocModel::Ropi => sym::ropi,
            RelocModel::Rwpi => sym::rwpi,
            RelocModel::RopiRwpi => sym::ropi_rwpi,
        }
    }
}

impl crate::json::ToJson for RelocModel {
    fn to_json(&self) -> crate::json::Json {
        self.desc().to_json()
    }
}