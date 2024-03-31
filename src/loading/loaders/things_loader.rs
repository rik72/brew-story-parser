use super::loadable::Loadable;
use crate::{
    common::brew_error::BrewError, loading::load_path::LoadPath,
    parsing::docs::things_doc::ThingsDoc,
};

pub struct ThingsLoader {}

impl Loadable for ThingsLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        match Self::parse_merge::<ThingsDoc>(load_path, "things.yml") {
            Some(doc) => {
                for _thing in doc.things {
                    // println!("{:?}", thing);
                }
            }
            None => println!("No things loaded"),
        };
        Ok(())
    }
}
