use brew_derive_macros::Raw;
use serde::Deserialize;

use super::{
    action_raw::ActionRaw,
    possibility_raw::PossibilityRaw,
    traits::{inner_raws::InnerRaws, raw::Raw},
    word_raw::WordRaw,
};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ThingStatusRaw {
    pub file_name: Option<String>,
    pub status: String,
    pub description: String,
    pub word: Option<WordRaw>,
    pub possibilities: Option<Vec<PossibilityRaw>>,
    pub actions: Option<Vec<ActionRaw>>,
}

impl InnerRaws for ThingStatusRaw {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        if let Some(word) = &mut self.word {
            word.set_file_name(file_name);
        }
        if let Some(possibilities) = &mut self.possibilities {
            for item in possibilities {
                item.set_file_name(file_name);
            }
        }
        if let Some(actions) = &mut self.actions {
            for item in actions {
                item.set_file_name(file_name);
            }
        }
    }
}
