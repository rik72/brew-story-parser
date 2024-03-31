use brew_derive_macros::{FromYml, Raw};
use serde::Deserialize;
use serde_yaml::from_str;

use crate::common::brew_error::BrewError;
use crate::parsing::raw::traits::inner_raws::InnerRaws;
use crate::parsing::raw::traits::raw::Raw;
use crate::parsing::{from_yml::FromYml, raw::location_raw::LocationRaw};

use super::mergeable::Mergeable;

#[derive(Debug, Deserialize, FromYml, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LocationsDoc {
    pub file_name: Option<String>,
    pub locations: Vec<LocationRaw>,
}

impl Mergeable for LocationsDoc {
    fn merge(&mut self, other: &mut Self) {
        if other.locations.len() > 0 {
            self.locations.append(&mut other.locations);
        }
    }
}

impl InnerRaws for LocationsDoc {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        for item in &mut self.locations {
            item.set_file_name(file_name);
        }
    }
}
