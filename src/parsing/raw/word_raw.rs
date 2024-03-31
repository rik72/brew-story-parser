use brew_derive_macros::Raw;
use serde::Deserialize;

use super::traits::{inner_raws::InnerRaws, raw::Raw};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WordRaw {
    pub file_name: Option<String>,
    pub text: String,
    pub synonyms: Option<Vec<String>>,
    pub complement: Option<String>,
    pub supplement: Option<String>,
}

impl InnerRaws for WordRaw {
    fn set_inner_raws_file_name(&mut self, _file_name: &String) {
        // NOP
    }
}
