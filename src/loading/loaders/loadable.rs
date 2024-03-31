use crate::{
    common::brew_error::BrewError,
    loading::load_path::LoadPath,
    parsing::{docs::mergeable::Mergeable, from_yml::FromYml, raw::traits::raw::Raw},
    utils::files::file_utils::find_files,
};

pub trait Loadable {
    fn load(load_path: &LoadPath) -> Result<(), BrewError>;

    fn parse_merge<T: FromYml<T> + Mergeable + Raw>(
        load_path: &LoadPath,
        file_name: &str,
    ) -> Option<T> {
        let files = Self::find_files(load_path, file_name);
        let mut docs = Vec::new();
        for file in files {
            let doc = T::from_yml(file.as_str());
            match doc {
                Ok(data) => {
                    docs.push(data);
                }
                Err(e) => println!(
                    "    [Invalid YML error] Failed to parse `{}` <= {}",
                    file, e
                ),
            }
        }
        if docs.len() > 0 {
            let mut merged_doc = docs.pop().unwrap();
            for mut doc in docs {
                merged_doc.merge(&mut doc);
            }
            return Some(merged_doc);
        }
        None
    }

    fn find_files(load_path: &LoadPath, file_name: &str) -> Vec<String> {
        find_files(load_path, file_name)
    }
}
