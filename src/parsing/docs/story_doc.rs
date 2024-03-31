use brew_derive_macros::{FromYml, Raw};
use serde::Deserialize;
use serde_yaml::from_str;

use crate::common::brew_error::BrewError;
use crate::parsing::raw::traits::inner_raws::InnerRaws;
use crate::parsing::raw::traits::raw::Raw;
use crate::parsing::{from_yml::FromYml, raw::story_raw::StoryRaw};

use super::mergeable::Mergeable;

#[derive(Debug, Deserialize, FromYml, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StoryDoc {
    pub file_name: Option<String>,
    pub story: StoryRaw,
}

impl Mergeable for StoryDoc {
    fn merge(&mut self, _other: &mut Self) {
        unimplemented!();
    }
}

impl InnerRaws for StoryDoc {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        self.story.set_file_name(file_name);
    }
}
