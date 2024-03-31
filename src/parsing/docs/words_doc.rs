use brew_derive_macros::{FromYml, Raw};
use serde::Deserialize;
use serde_yaml::from_str;

use crate::common::brew_error::BrewError;

use crate::parsing::raw::traits::inner_raws::InnerRaws;
use crate::parsing::raw::traits::raw::Raw;
use crate::parsing::{from_yml::FromYml, raw::dictionary_raw::DictionaryRaw};

use super::mergeable::Mergeable;

#[derive(Debug, Deserialize, FromYml, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WordsDoc {
    pub file_name: Option<String>,
    pub words: DictionaryRaw,
}

#[macro_export]
macro_rules! append_dictionary_field {
    ($self:ident, $other:ident, $field:ident) => {
        if let Some(ref mut other_words) = $other.words.$field {
            if let Some(ref mut self_words) = $self.words.$field {
                self_words.append(other_words);
            } else {
                let mut vec = Vec::new();
                vec.append(other_words);
                $self.words.$field = Some(vec);
            }
        }
    };
}

impl Mergeable for WordsDoc {
    fn merge(&mut self, other: &mut Self) {
        append_dictionary_field!(self, other, commands);
        append_dictionary_field!(self, other, direction_actions);
        append_dictionary_field!(self, other, directions);
        append_dictionary_field!(self, other, names);
        append_dictionary_field!(self, other, one_actions);
        append_dictionary_field!(self, other, particles);
        append_dictionary_field!(self, other, prepositions);
        append_dictionary_field!(self, other, stop_words);
        append_dictionary_field!(self, other, two_actions);
        append_dictionary_field!(self, other, zero_actions);
    }
}

impl InnerRaws for WordsDoc {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        self.words.set_file_name(file_name);
    }
}
