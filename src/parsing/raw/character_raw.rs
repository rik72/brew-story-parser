use brew_derive_macros::Raw;
use serde::Deserialize;

use super::{
    character_status_raw::CharacterStatusRaw,
    traits::{inner_raws::InnerRaws, raw::Raw},
    word_raw::WordRaw,
};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CharacterRaw {
    pub file_name: Option<String>,
    pub name: String,
    #[serde(default = "default_main")]
    pub main: bool,
    pub start_location: String,
    pub word: WordRaw,
    #[serde(default = "default_visible")]
    pub visible: bool,
    pub statuses: Vec<CharacterStatusRaw>,
}

fn default_main() -> bool {
    false
}

fn default_visible() -> bool {
    true
}

// impl Raw for CharacterRaw {
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

impl InnerRaws for CharacterRaw {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        self.word.set_file_name(file_name);
        for item in &mut self.statuses {
            item.set_file_name(file_name);
        }
    }
}
