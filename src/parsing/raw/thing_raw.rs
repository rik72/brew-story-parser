use brew_derive_macros::Raw;
use serde::Deserialize;

use super::{
    thing_status_raw::ThingStatusRaw,
    traits::{inner_raws::InnerRaws, raw::Raw},
    word_raw::WordRaw,
};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ThingRaw {
    pub file_name: Option<String>,
    pub name: String,
    pub location: String,
    pub word: WordRaw,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_takeable")]
    pub takeable: bool,
    #[serde(default = "default_droppable")]
    pub droppable: bool,
    #[serde(default = "default_plural")]
    pub plural: bool,
    pub statuses: Vec<ThingStatusRaw>,
}

fn default_visible() -> bool {
    true
}

fn default_takeable() -> bool {
    false
}

fn default_droppable() -> bool {
    true
}

fn default_plural() -> bool {
    false
}

impl InnerRaws for ThingRaw {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        self.word.set_file_name(file_name);
        for item in &mut self.statuses {
            item.set_file_name(file_name);
        }
    }
}
