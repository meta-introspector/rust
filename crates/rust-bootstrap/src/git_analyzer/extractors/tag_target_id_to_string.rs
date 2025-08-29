use git2::Tag;
use git2::Oid;
use std::convert::Into;

pub fn tag_target_id_to_string(tag_obj: &Tag) -> String {
    let target_id_option: Option<Oid> = tag_obj.target_id().into();
    match target_id_option {
        Some(oid) => oid.to_string(),
        None => "".to_string(),
    }
}
