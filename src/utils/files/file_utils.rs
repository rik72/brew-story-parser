use crate::loading::load_path::LoadPath;

use super::{can_finder::CanFinder, finder::Finder, folder_finder::FolderFinder};

pub fn find_files(load_path: &LoadPath, filename: &str) -> Vec<String> {
    match load_path {
        LoadPath::Folder(path) => FolderFinder::find_files(filename, path),
        LoadPath::CanFile(path) => CanFinder::find_files(filename, path),
    }
}
