use super::loadable::Loadable;
use crate::{
    common::brew_error::BrewError,
    loading::load_path::LoadPath,
    parsing::{
        docs::characters_doc::CharactersDoc,
        parsers::{parseable::Parseable, possibility_parser::PossibilityParser},
    },
};

pub struct CharactersLoader {}

impl Loadable for CharactersLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        match Self::parse_merge::<CharactersDoc>(load_path, "characters.yml") {
            Some(doc) => {
                for character in doc.characters {
                    println!("\n    character `{}`", character.name);
                    let mut first_status = true;
                    let mut description = String::from("");
                    for st_item in character.statuses {
                        println!("        status `{}`", st_item.status);
                        if first_status && !"initial".eq(&st_item.status) {
                            return Err(BrewError::FailedToLoad(st_item.file_name.unwrap(), format!("Character `{}`: first item of status list must be the `initial` status", character.name)));
                        }

                        match st_item.description {
                            None => {
                                if description.len() == 0 && !"initial".eq(&st_item.status) {
                                    return Err(BrewError::FailedToLoad(st_item.file_name.unwrap(), format!("Character `{}`: description of `initial` status cannot be empty", character.name)));
                                }
                            }
                            Some(st_description) => {
                                description = st_description;
                            }
                        };

                        if let Some(possibilities) = st_item.possibilities {
                            for mut p_doc in possibilities {
                                let possibility_parser = PossibilityParser::new();
                                match possibility_parser.parse(&mut p_doc) {
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
                        first_status = false;
                    }
                }
            }
            None => println!("No characters loaded"),
        };
        Ok(())
    }
}
