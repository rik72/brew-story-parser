use brew_derive_macros::Raw;
use serde::Deserialize;

use super::{
    location_status_raw::LocationStatusRaw,
    traits::{inner_raws::InnerRaws, raw::Raw},
    word_raw::WordRaw,
};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LocationRaw {
    pub file_name: Option<String>,
    pub name: String,
    pub word: WordRaw,
    pub statuses: Vec<LocationStatusRaw>,
}

impl InnerRaws for LocationRaw {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        self.word.set_file_name(file_name);
        for item in &mut self.statuses {
            item.set_file_name(file_name);
        }
    }
}
