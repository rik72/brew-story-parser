use brew_derive_macros::Raw;
use serde::Deserialize;

use super::traits::{inner_raws::InnerRaws, raw::Raw};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PossibilityRaw {
    pub file_name: Option<String>,
    pub possible: Option<String>,
    pub impossible: Option<String>,
    pub inherit: Option<String>,
}

impl InnerRaws for PossibilityRaw {
    fn set_inner_raws_file_name(&mut self, _file_name: &String) {
        // NOP
    }
}
