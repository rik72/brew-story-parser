use brew_derive_macros::Raw;
use serde::Deserialize;

use super::{
    directions_raw::DirectionsRaw,
    possibility_raw::PossibilityRaw,
    traits::{inner_raws::InnerRaws, raw::Raw},
    word_raw::WordRaw,
};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LocationStatusRaw {
    pub file_name: Option<String>,
    pub status: String,
    pub image: Option<String>,
    pub description: Option<String>,
    pub word: Option<WordRaw>,
    pub directions: Option<DirectionsRaw>,
    pub possibilities: Option<Vec<PossibilityRaw>>,
    pub transition: Option<String>,
    pub finale: Option<String>,
}

impl InnerRaws for LocationStatusRaw {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        if let Some(word) = &mut self.word {
            word.set_file_name(file_name);
        }
        if let Some(directions) = &mut self.directions {
            directions.set_file_name(file_name);
        }
        if let Some(possibilities) = &mut self.possibilities {
            for item in possibilities {
                item.set_file_name(file_name);
            }
        }
    }
}
