use super::finder::Finder;
use walkdir::WalkDir;

pub struct FolderFinder;

impl Finder for FolderFinder {
    fn find_files(req_f_name: &str, path: &String) -> Vec<String> {
        let mut f_names = Vec::new();
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            if f_name.eq(req_f_name) {
                f_names.push(entry.path().to_string_lossy().to_string());
            }
        }
        f_names
    }
}
