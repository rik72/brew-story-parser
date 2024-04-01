use super::finder::Finder;

pub struct CanFinder;

impl Finder for CanFinder {
    fn find_files(filename: &str, path: &String) -> Vec<String> {
        println!("{filename} + {path}");
        Vec::new()
    }
}
