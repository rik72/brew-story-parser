use brew_derive_macros::{FromYml, Raw};
use serde::Deserialize;
use serde_yaml::from_str;

use crate::common::brew_error::BrewError;
use crate::parsing::raw::traits::inner_raws::InnerRaws;
use crate::parsing::raw::traits::raw::Raw;
use crate::parsing::{from_yml::FromYml, raw::thing_raw::ThingRaw};

use super::mergeable::Mergeable;

#[derive(Debug, Deserialize, FromYml, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ThingsDoc {
    pub file_name: Option<String>,
    pub things: Vec<ThingRaw>,
}

impl Mergeable for ThingsDoc {
    fn merge(&mut self, other: &mut Self) {
        if other.things.len() > 0 {
            self.things.append(&mut other.things);
        }
    }
}

impl InnerRaws for ThingsDoc {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        for item in &mut self.things {
            item.set_file_name(file_name);
        }
    }
}
