use brew_derive_macros::{FromYml, Raw};
use serde::Deserialize;
use serde_yaml::from_str;

use crate::common::brew_error::BrewError;
use crate::parsing::raw::traits::inner_raws::InnerRaws;
use crate::parsing::raw::traits::raw::Raw;
use crate::parsing::{from_yml::FromYml, raw::character_raw::CharacterRaw};

use super::mergeable::Mergeable;

#[derive(Debug, Deserialize, Raw, FromYml)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CharactersDoc {
    pub file_name: Option<String>,
    pub characters: Vec<CharacterRaw>,
}

impl Mergeable for CharactersDoc {
    fn merge(&mut self, other: &mut Self) {
        if other.characters.len() > 0 {
            self.characters.append(&mut other.characters);
        }
    }
}

// impl Raw for CharactersDoc {
//     fn set_file_name(&mut self, file_name: &String) {
//         self.file_name = Some(file_name.to_owned());
//         let inner_raws_opt = self.get_inner_raws();
//         if let Some(inner_raws) = inner_raws_opt {
//             for inner_raw in inner_raws {
//                 inner_raw.set_file_name(file_name);
//             }
//         }
//     }

//     fn get_file_name(&self) -> String {
//         match &self.file_name {
//             Some(file_name) => file_name.to_owned(),
//             None => String::from(""),
//         }
//     }
// }

// impl FromYml<CharactersDoc> for CharactersDoc {
//     fn from_yml(file_name: &str) -> Result<CharactersDoc, BrewError> {
//         let mut data: CharactersDoc = from_str(&Self::read_yml(file_name))?;
//         data.set_file_name(&file_name.to_string());
//         Ok(data)
//     }
// }

impl InnerRaws for CharactersDoc {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        for item in &mut self.characters {
            item.set_file_name(file_name);
        }
    }
}
