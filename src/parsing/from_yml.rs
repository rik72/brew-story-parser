use std::fs;

use crate::common::brew_error::BrewError;

use super::raw::traits::raw::Raw;

/*
* Implementors must:
# - use brew_derive_macros::FromYml;
* - #[derive(Debug, Deserialize, FromYml)]
# - #[serde(deny_unknown_fields, rename_all = "camelCase")]
*/
pub trait FromYml<T: Raw> {
    fn from_yml(file_name: &str) -> Result<T, BrewError>;

    fn read_yml(file_name: &str) -> String {
        fs::read_to_string(file_name.to_string()).unwrap()
    }
}
