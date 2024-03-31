use super::loadable::Loadable;
use crate::parsing::parsers::directions_parser::DirectionsParser;
use crate::parsing::parsers::parseable::Parseable;
use crate::{
    common::brew_error::BrewError,
    loading::load_path::LoadPath,
    parsing::{docs::locations_doc::LocationsDoc, parsers::possibility_parser::PossibilityParser},
};

pub struct LocationsLoader {}

impl Loadable for LocationsLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        match Self::parse_merge::<LocationsDoc>(load_path, "locations.yml") {
            Some(doc) => {
                for location in doc.locations {
                    println!("\n    location `{}`", location.name);
                    let mut first_status = true;
                    let mut description = String::from("");
                    for st_item in location.statuses {
                        println!("        status `{}`", st_item.status);
                        if first_status && !"initial".eq(&st_item.status) {
                            return Err(BrewError::FailedToLoad(st_item.file_name.unwrap(), format!("Location `{}`: first item of status list must be the `initial` status", location.name)));
                        }

                        match st_item.description {
                            None => {
                                if description.len() == 0 && !"initial".eq(&st_item.status) {
                                    return Err(BrewError::FailedToLoad(st_item.file_name.unwrap(), format!("Location `{}`: description of `initial` status cannot be empty", location.name)));
                                }
                            }
                            Some(st_description) => {
                                description = st_description;
                            }
                        };

                        if let Some(mut directions) = st_item.directions {
                            let directions_parser = DirectionsParser::new();
                            match directions_parser.parse(&mut directions) {
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

                        if let Some(possibilities) = st_item.possibilities {
                            for mut p_doc in possibilities {
                                let possibility_parser = PossibilityParser::new();
                                match possibility_parser.parse(&mut p_doc) {
                                    Ok(parsed_opt) => {
                                        if let Some(parsed) = parsed_opt {
                                            println!("{:?}", parsed);
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
            None => println!("No locations loaded"),
        };
        Ok(())
    }
}
