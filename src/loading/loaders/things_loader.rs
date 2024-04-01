use super::loadable::Loadable;
use crate::{
    common::brew_error::BrewError, loading::load_path::LoadPath,
    parsing::docs::things_doc::ThingsDoc,
};

pub struct ThingsLoader;

impl Loadable for ThingsLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        match Self::parse_merge::<ThingsDoc>(load_path, "things.yml") {
            Some(doc) => {
                for thing in doc.things {
                    println!("\n    thing `{}`", thing.name);
                    let mut first_status = true;
                    let mut _description = String::from("");
                    for st_item in thing.statuses {
                        println!("        status `{}`", st_item.status);
                        if first_status && !"initial".eq(&st_item.status) {
                            return Err(BrewError::FailedToLoad(st_item.file_name.unwrap(), format!("Thing `{}`: first item of status list must be the `initial` status", thing.name)));
                        }
                        first_status = false;
                    }
                }
            }
            None => println!("No things loaded"),
        };
        Ok(())
    }
}
