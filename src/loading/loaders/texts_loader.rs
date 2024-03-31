use super::loadable::Loadable;
use crate::{
    common::brew_error::BrewError, loading::load_path::LoadPath, parsing::docs::texts_doc::TextsDoc,
};

pub struct TextsLoader {}

impl Loadable for TextsLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        match Self::parse_merge::<TextsDoc>(load_path, "texts.yml") {
            Some(doc) => {
                for _text in doc.texts {
                    // println!("{:?}", text);
                }
            }
            None => println!("No texts loaded"),
        };
        Ok(())
    }
}
