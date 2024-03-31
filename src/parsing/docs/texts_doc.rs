use brew_derive_macros::{FromYml, Raw};
use serde::Deserialize;
use serde_yaml::from_str;

use crate::common::brew_error::BrewError;
use crate::parsing::raw::traits::inner_raws::InnerRaws;
use crate::parsing::raw::traits::raw::Raw;
use crate::parsing::{from_yml::FromYml, raw::text_raw::TextRaw};

use super::mergeable::Mergeable;

#[derive(Debug, Deserialize, FromYml, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextsDoc {
    pub file_name: Option<String>,
    pub texts: Vec<TextRaw>,
}

impl Mergeable for TextsDoc {
    fn merge(&mut self, other: &mut Self) {
        if other.texts.len() > 0 {
            self.texts.append(&mut other.texts);
        }
    }
}

impl InnerRaws for TextsDoc {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        for item in &mut self.texts {
            item.set_file_name(file_name);
        }
    }
}
