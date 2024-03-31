use brew_derive_macros::Raw;
use serde::Deserialize;

use super::traits::{inner_raws::InnerRaws, raw::Raw};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DirectionsRaw {
    pub file_name: Option<String>,
    pub north: Option<String>,
    pub northeast: Option<String>,
    pub east: Option<String>,
    pub southeast: Option<String>,
    pub south: Option<String>,
    pub southwest: Option<String>,
    pub west: Option<String>,
    pub northwest: Option<String>,
    pub up: Option<String>,
    pub down: Option<String>,
}

impl InnerRaws for DirectionsRaw {
    fn set_inner_raws_file_name(&mut self, _file_name: &String) {
        // NOP
    }
}
