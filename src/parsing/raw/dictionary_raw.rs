use brew_derive_macros::Raw;
use serde::Deserialize;

use super::{
    traits::{inner_raws::InnerRaws, raw::Raw},
    word_raw::WordRaw,
};

#[derive(Debug, Deserialize, Raw)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DictionaryRaw {
    pub file_name: Option<String>,
    pub commands: Option<Vec<WordRaw>>,
    pub direction_actions: Option<Vec<WordRaw>>,
    pub directions: Option<Vec<WordRaw>>,
    pub names: Option<Vec<WordRaw>>,
    pub particles: Option<Vec<WordRaw>>,
    pub one_actions: Option<Vec<WordRaw>>,
    pub prepositions: Option<Vec<WordRaw>>,
    pub stop_words: Option<Vec<WordRaw>>,
    pub two_actions: Option<Vec<WordRaw>>,
    pub zero_actions: Option<Vec<WordRaw>>,
}

impl InnerRaws for DictionaryRaw {
    fn set_inner_raws_file_name(&mut self, file_name: &String) {
        if let Some(commands) = &mut self.commands {
            for item in commands {
                item.set_file_name(file_name);
            }
        }
        if let Some(direction_actions) = &mut self.direction_actions {
            for item in direction_actions {
                item.set_file_name(file_name);
            }
        }
        if let Some(directions) = &mut self.directions {
            for item in directions {
                item.set_file_name(file_name);
            }
        }
        if let Some(names) = &mut self.names {
            for item in names {
                item.set_file_name(file_name);
            }
        }
        if let Some(particles) = &mut self.particles {
            for item in particles {
                item.set_file_name(file_name);
            }
        }
        if let Some(one_actions) = &mut self.one_actions {
            for item in one_actions {
                item.set_file_name(file_name);
            }
        }
        if let Some(prepositions) = &mut self.prepositions {
            for item in prepositions {
                item.set_file_name(file_name);
            }
        }
        if let Some(stop_words) = &mut self.stop_words {
            for item in stop_words {
                item.set_file_name(file_name);
            }
        }
        if let Some(two_actions) = &mut self.two_actions {
            for item in two_actions {
                item.set_file_name(file_name);
            }
        }
        if let Some(zero_actions) = &mut self.zero_actions {
            for item in zero_actions {
                item.set_file_name(file_name);
            }
        }
    }
}
