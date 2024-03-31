use brew_derive_macros::Raw;
use serde::Deserialize;

use super::{
    skin_raw::SkinRaw,
    traits::{inner_raws::InnerRaws, raw::Raw},
};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StoryRaw {
    pub file_name: Option<String>,
    pub artifact_id: String,
    pub version: String,
    pub save_version: i64,
    pub title: String,
    pub subtitle: String,
    pub intro: String,
    pub locale: Option<String>,
    pub skin: Option<SkinRaw>,
}

impl InnerRaws for StoryRaw {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        if let Some(skin) = &mut self.skin {
            skin.set_file_name(file_name);
        }
    }
}
