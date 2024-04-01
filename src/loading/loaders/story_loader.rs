use vati::core::{vati_locale::VatiLocale, vati_locale_service::VATI_LOCALE};

use crate::{
    common::brew_error::BrewError, loading::load_path::LoadPath, parsing::docs::story_doc::StoryDoc,
};

use super::loadable::Loadable;

pub struct StoryLoader;

impl Loadable for StoryLoader {
    fn load(load_path: &LoadPath) -> Result<(), BrewError> {
        match Self::parse_merge::<StoryDoc>(load_path, "story.yml") {
            Some(doc) => {
                // println!("{:?}", doc);
                // println!("TODO: story clear");
                // println!("TODO: set descriptor");
                // println!("TODO: if story_doc.intro => load text groups");
                if let Some(locale) = doc.story.locale {
                    let vati_locale_opt = VatiLocale::from(locale.as_str());
                    match vati_locale_opt {
                        Some(vati_locale) => {
                            VATI_LOCALE.service_mut().unwrap().set_current(vati_locale);
                        }
                        None => {
                            return Err(BrewError::FailedToLoad(
                                doc.story.file_name.unwrap(),
                                format!("Invalid locale `{}`", locale),
                            ));
                        }
                    }
                }
                // println!("TODO: if story_doc.locale => set locale");
                // println!("TODO: set skin defaults");
                // println!("TODO: if story_doc.skin => set skin");
            }
            None => println!("No characters loaded"),
        };
        Ok(())
    }
}
