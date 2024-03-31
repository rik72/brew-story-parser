use anyhow::Result;
use brew_story_parser::{
    _rust_i18n_translate,
    loading::{
        load_path::LoadPath,
        loaders::{
            action_loader::ActionsLoader, characters_loader::CharactersLoader, loadable::Loadable,
            locations_loader::LocationsLoader, story_loader::StoryLoader,
            texts_loader::TextsLoader, things_loader::ThingsLoader, words_loader::WordsLoader,
        },
    },
    services::service_registry::ServiceRegistry,
};
use rust_i18n::t;

fn main() -> Result<()> {
    ServiceRegistry::init_services();

    let load_path = LoadPath::Folder("stories/test".to_string());

    println!("\nLoading texts...");
    if let Err(error) = TextsLoader::load(&load_path) {
        println!("    {}", error.to_string());
    } else {
        println!("    ...ok");
    }

    println!("before: {}", t!("begin_adventuring"));

    println!("\nLoading story...");
    if let Err(error) = StoryLoader::load(&load_path) {
        println!("    {}", error.to_string());
    } else {
        println!("    ...ok");
    }

    println!("after: {}", t!("begin_adventuring"));

    println!("\nLoading words...");
    if let Err(error) = WordsLoader::load(&load_path) {
        println!("    {}", error.to_string());
    } else {
        println!("    ...ok");
    }

    println!("\nLoading locations...");
    if let Err(error) = LocationsLoader::load(&load_path) {
        println!("    {}", error.to_string());
    } else {
        println!("    ...ok");
    }

    println!("\nLoading characters...");
    if let Err(error) = CharactersLoader::load(&load_path) {
        println!("    {}", error.to_string());
    } else {
        println!("    ...ok");
    }

    println!("\nLoading things...");
    if let Err(error) = ThingsLoader::load(&load_path) {
        println!("    {}", error.to_string());
    } else {
        println!("    ...ok");
    }

    println!("\nLoading actions...");
    if let Err(error) = ActionsLoader::load(&load_path) {
        println!("    {}", error.to_string());
    } else {
        println!("    ...ok");
    }

    Ok(())
}
