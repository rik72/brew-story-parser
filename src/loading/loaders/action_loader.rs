use super::loadable::Loadable;
use crate::{
    common::brew_error::BrewError,
    loading::load_path::LoadPath,
    parsing::docs::{characters_doc::CharactersDoc, things_doc::ThingsDoc},
};

pub struct ActionsLoader {}

impl Loadable for ActionsLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        match Self::parse_merge::<ThingsDoc>(load_path, "things.yml") {
            Some(doc) => {
                for _thing in doc.things {
                    // println!("{:?} actions", thing);
                }
            }
            None => println!("No character actions loaded"),
        };
        match Self::parse_merge::<CharactersDoc>(load_path, "characters.yml") {
            Some(doc) => {
                for _character in doc.characters {
                    // println!("{:?} actions", character);
                }
            }
            None => println!("No character actions loaded"),
        };
        Ok(())
    }
}
