use brew_derive_macros::Raw;
use serde::Deserialize;

use super::traits::{inner_raws::InnerRaws, raw::Raw};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActionRaw {
    pub file_name: Option<String>,
    pub action: String,
    pub consequences: Option<Vec<String>>,
    pub transition: Option<String>,
    pub finale: Option<String>,
}

impl InnerRaws for ActionRaw {
    fn set_inner_raws_file_name(&mut self, _file_name: &String) {
        // NOP
    }
}
