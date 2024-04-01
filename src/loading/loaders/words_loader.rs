use super::loadable::Loadable;
use crate::{
    common::brew_error::BrewError, loading::load_path::LoadPath, parsing::docs::words_doc::WordsDoc,
};

pub struct WordsLoader;

impl Loadable for WordsLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        for file_name in vec![
            "commands.yml",
            "prepositions.yml",
            "particles.yml",
            "verbs.yml",
            "stop_words.yml",
            "other_names.yml",
        ] {
            Self::parse_merge::<WordsDoc>(load_path, file_name);
        }
        Ok(())
    }
}
