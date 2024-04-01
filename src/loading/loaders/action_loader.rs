use super::loadable::Loadable;
use crate::parsing::parsers::parseable::Parseable;
use crate::{
    common::brew_error::BrewError,
    loading::load_path::LoadPath,
    parsing::{
        docs::{characters_doc::CharactersDoc, things_doc::ThingsDoc},
        parsers::action_parser::ActionParser,
    },
};

pub struct ActionsLoader;

impl Loadable for ActionsLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        let action_parser = ActionParser::new();
        match Self::parse_merge::<ThingsDoc>(load_path, "things.yml") {
            Some(doc) => {
                for thing in doc.things {
                    println!("\n    thing `{}`", thing.name);
                    for st_item in thing.statuses {
                        println!("        status `{}`", st_item.status);
                        if let Some(actions) = st_item.actions {
                            for mut ac_item in actions {
                                match action_parser.parse(&mut ac_item) {
                                    Ok(parsed_opt) => {
                                        if let Some(parsed) = parsed_opt {
                                            println!("            `{:?}`", parsed);
                                        }
                                    }
                                    Err(error) => {
                                        return Err(BrewError::FailedToParse(
                                            st_item.file_name.unwrap(),
                                            error.to_string(),
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => println!("No character actions loaded"),
        };
        match Self::parse_merge::<CharactersDoc>(load_path, "characters.yml") {
            Some(doc) => {
                for character in doc.characters {
                    println!("\n    character `{}`", character.name);
                    for st_item in character.statuses {
                        println!("        status `{}`", st_item.status);
                        if let Some(actions) = st_item.actions {
                            for mut ac_item in actions {
                                match action_parser.parse(&mut ac_item) {
                                    Ok(parsed_opt) => {
                                        if let Some(parsed) = parsed_opt {
                                            println!("            `{:?}`", parsed);
                                        }
                                    }
                                    Err(error) => {
                                        return Err(BrewError::FailedToParse(
                                            st_item.file_name.unwrap(),
                                            error.to_string(),
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => println!("No character actions loaded"),
        };
        Ok(())
    }
}
